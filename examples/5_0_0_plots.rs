use polars::prelude::{pivot::pivot_stable, *};

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Modify var
    let lf = lf
        .filter(col("hrlyearn").is_not_null())
        .with_column((col("hrlyearn").cast(DataType::Float64) / lit(100)).alias("hourly_wages"));

    // Calculate weighted quantile
    fn weighted_quantile(col: Expr, wt: Expr, percentile: Expr) -> Expr {
        col.sort_by(
            [(wt.clone().cast(DataType::Float64).cum_sum(false)
                / wt.clone().cast(DataType::Float64).sum()
                - percentile)
                .abs()],
            SortMultipleOptions::default(),
        )
        .first()
        .alias("median")
    }

    // Weighted statistics
    let hourly_wages_stats_wt = lf
        .clone()
        .sort(["hourly_wages"], SortMultipleOptions::new())
        .select([weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.50)).alias("median")])
        .unpivot(UnpivotArgsDSL::default())
        .collect()
        .unwrap();

    println!(
        "Table of weighted summary statistics about hourly wages (whole period): {}",
        hourly_wages_stats_wt
    );

    // Weighted statistics (by gender)
    let hourly_wages_stats_wt_by_gender = lf
        .clone()
        .sort(["gender", "hourly_wages"], SortMultipleOptions::new())
        .group_by(["gender"])
        .agg([
            ((col("finalwt").sum()) / lit(1000000)).alias("count (x100,000)"),
            (((col("hourly_wages") * col("finalwt")).sum()) / col("finalwt").sum())
                .alias("mean")
                .round(2, RoundMode::HalfAwayFromZero),
            col("hourly_wages").min().alias("min"),
            weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.01)).alias("p01"),
            weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.25)).alias("p25"),
            weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.50)).alias("median"),
            weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.75)).alias("p75"),
            weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.99)).alias("p99"),
            col("hourly_wages").max().alias("max"),
        ])
        .collect()
        .unwrap()
        .unpivot(
            [
                "count (x100,000)",
                "mean",
                "min",
                "p01",
                "p25",
                "median",
                "p75",
                "p99",
                "max",
            ],
            ["gender"],
        )
        .unwrap();

    let hourly_wages_stats_wt_by_gender = pivot_stable(
        &hourly_wages_stats_wt_by_gender,
        ["gender"],
        Some(["variable"]),
        Some(["value"]),
        false,
        None,
        None,
    )
    .unwrap();

    println!(
        "Table of weighted summary statistics about hourly wages (whole period): {}",
        hourly_wages_stats_wt_by_gender
    );
}
