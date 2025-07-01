use polars::prelude::{pivot::pivot_stable, *};

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Modify var
    let lf = lf
        .filter(col("hrlyearn").is_not_null())
        .with_column((col("hrlyearn").cast(DataType::Float64) / lit(100)).alias("hourly_wages"));

    // Simple statistics (single point)
    let mean_hourly_wages = lf
        .clone()
        .select([col("hourly_wages")])
        .mean()
        .collect()
        .unwrap();

    println!("Mean hourly wages (whole period):\n\n{mean_hourly_wages}\n");

    // Simple statistics by category
    let mean_hourly_wages_by_prov = lf
        .clone()
        .group_by([col("prov")])
        .agg([col("hourly_wages")
            .mean()
            .round(2, RoundMode::HalfAwayFromZero)])
        .collect()
        .unwrap();

    println!("Mean hourly wages by province (whole period):\n\n{mean_hourly_wages_by_prov}\n");

    // Multiple statistics (calculated)
    let hourly_wages_stats = lf
        .clone()
        .select([
            (len() / lit(1000)).alias("count (x1000)"),
            col("hourly_wages").mean().alias("mean"),
            col("hourly_wages").min().alias("min"),
            col("hourly_wages")
                .quantile(lit(0.01), QuantileMethod::Nearest)
                .alias("p01"),
            col("hourly_wages")
                .quantile(lit(0.25), QuantileMethod::Nearest)
                .alias("p25"),
            col("hourly_wages").median().alias("median"),
            col("hourly_wages")
                .quantile(lit(0.75), QuantileMethod::Nearest)
                .alias("p75"),
            col("hourly_wages")
                .quantile(lit(0.99), QuantileMethod::Nearest)
                .alias("p99"),
            col("hourly_wages").max().alias("max"),
        ])
        .unpivot(UnpivotArgsDSL::default())
        .collect()
        .unwrap();

    println!(
        "Table of summary statistics about hourly wages (whole period):\n\n{hourly_wages_stats}\n"
    );

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
        .select([
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
        .unpivot(UnpivotArgsDSL::default())
        .collect()
        .unwrap();

    println!(
        "Table of weighted summary statistics about hourly wages (whole period):\n\n{hourly_wages_stats_wt}\n"
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
        "Table of weighted summary statistics about hourly wages (whole period):\n\n{hourly_wages_stats_wt_by_gender}\n"
    );
}
