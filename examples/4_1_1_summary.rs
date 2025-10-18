// === imports
use polars::prelude::{pivot::pivot_stable, *};

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/large/partitioned"), args).unwrap();

    // Modify var
    let lf = lf
        .filter(col("keep_type").eq(lit(1))) // Usual resident
        .filter(col("income").is_not_null())
        .with_column(col("region").replace_strict(
            lit(Series::from_iter(vec![
                "E12000001",
                "E12000002",
                "E12000003",
                "E12000004",
                "E12000005",
                "E12000006",
                "E12000007",
                "E12000008",
                "E12000009",
                "W92000004",
            ])),
            lit(Series::from_iter(vec![
                "North East",
                "North West",
                "Yorkshire and The Humber",
                "East Midlands",
                "West Midlands",
                "East of England",
                "London",
                "South East",
                "South West",
                "Wales",
            ])),
            None,
            Some(DataType::String),
        ));

    // === block_2

    // Simple statistics (single point)
    let mean_income: DataFrame = lf
        .clone()
        .select([col("income")])
        .median()
        .collect()
        .unwrap();

    println!("Mean income:\n\n{mean_income}\n");

    // === block_3

    // Multiple statistics (calculated)
    let income_stats = lf
        .clone()
        .select([
            (len() / lit(100_000)).alias("count (x100,000)"),
            col("income")
                .mean()
                .round(2, RoundMode::HalfAwayFromZero)
                .alias("mean"),
            col("income").min().alias("min"),
            col("income")
                .quantile(lit(0.01), QuantileMethod::Nearest)
                .alias("p01"),
            col("income")
                .quantile(lit(0.25), QuantileMethod::Nearest)
                .alias("p25"),
            col("income").median().alias("median"),
            col("income")
                .quantile(lit(0.75), QuantileMethod::Nearest)
                .alias("p75"),
            col("income")
                .quantile(lit(0.99), QuantileMethod::Nearest)
                .alias("p99"),
            col("income").max().alias("max"),
        ])
        .unpivot(UnpivotArgsDSL {
            on: Selector::Empty,
            index: Selector::Empty,
            variable_name: Some("statistic".into()),
            value_name: Some("value".into()),
        })
        .collect()
        .unwrap();

    println!("Table of summary statistics about income:\n\n{income_stats}\n");

    // === block_4

    // Simple statistics by category
    let mean_income_by_region = lf
        .clone()
        .group_by([col("region")])
        .agg([col("income").mean().round(2, RoundMode::HalfAwayFromZero)])
        .collect()
        .unwrap();

    println!("Mean income by region:\n\n{mean_income_by_region}\n");

    // === block_5

    // Connect to LazyFrame
    let args = ScanArgsParquet::default();
    let lf =
        LazyFrame::scan_parquet(PlPath::from_str("./data/parquet/census_0.parquet"), args).unwrap();

    // Filter and format
    let lf = lf
        .filter(col("keep_type").eq(lit(1))) // Usual resident
        .filter(col("income").is_not_null())
        .with_column(col("sex").replace_strict(
            lit(Series::from_iter(vec!["1", "2"])),
            lit(Series::from_iter(vec!["Female", "Male"])),
            None,
            Some(DataType::String),
        ))
        .with_column(col("weight").cast(DataType::Float64));

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
    let income_stats_wt = lf
        .clone()
        .sort(["income"], SortMultipleOptions::new())
        .select([
            ((col("weight").sum()) / lit(100_000))
                .round(1, RoundMode::HalfAwayFromZero)
                .alias("count (x100,000)"),
            (((col("income") * col("weight")).sum()) / col("weight").sum())
                .round(2, RoundMode::HalfAwayFromZero)
                .alias("mean"),
            col("income").min().alias("min"),
            weighted_quantile(col("income"), col("weight"), lit(0.01)).alias("p01"),
            weighted_quantile(col("income"), col("weight"), lit(0.25)).alias("p25"),
            weighted_quantile(col("income"), col("weight"), lit(0.50)).alias("median"),
            weighted_quantile(col("income"), col("weight"), lit(0.75)).alias("p75"),
            weighted_quantile(col("income"), col("weight"), lit(0.99)).alias("p99"),
            col("income").max().alias("max"),
        ])
        .unpivot(UnpivotArgsDSL {
            on: Selector::Empty,
            index: Selector::Empty,
            variable_name: Some("statistic".into()),
            value_name: Some("value".into()),
        })
        .collect()
        .unwrap();

    println!("Table of weighted summary statistics about income:\n\n{income_stats_wt}\n");

    // === block_6

    // Weighted statistics (by sex)
    let income_stats_wt_by_sex = lf
        .clone()
        .sort(["sex", "income"], SortMultipleOptions::new())
        .group_by(["sex"])
        .agg([
            ((col("weight").sum()) / lit(100_000))
                .round(1, RoundMode::HalfAwayFromZero)
                .alias("count (x100,000)"),
            (((col("income") * col("weight")).sum()) / col("weight").sum())
                .round(2, RoundMode::HalfAwayFromZero)
                .alias("mean"),
            col("income").min().alias("min"),
            weighted_quantile(col("income"), col("weight"), lit(0.01)).alias("p01"),
            weighted_quantile(col("income"), col("weight"), lit(0.25)).alias("p25"),
            weighted_quantile(col("income"), col("weight"), lit(0.50)).alias("median"),
            weighted_quantile(col("income"), col("weight"), lit(0.75)).alias("p75"),
            weighted_quantile(col("income"), col("weight"), lit(0.99)).alias("p99"),
            col("income").max().alias("max"),
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
            ["sex"],
        )
        .unwrap();

    let income_stats_wt_by_sex = pivot_stable(
        &income_stats_wt_by_sex,
        ["sex"],
        Some(["variable"]),
        Some(["value"]),
        false,
        None,
        None,
    )
    .unwrap();

    println!("Table of weighted summary statistics about income:\n\n{income_stats_wt_by_sex}\n");

    // === end
}
