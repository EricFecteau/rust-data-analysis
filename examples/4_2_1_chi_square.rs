// === imports
use hypors::chi_square::independence;
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect to the parquet data
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/large/partitioned"), args).unwrap();

    // === block_2

    // Count individuals with fair or better health by sex and marital status
    let df = lf
        .filter(col("keep_type").eq(lit(1))) // Usual resident
        .filter(col("health").neq(lit(-8)))
        .filter(col("mar_stat").neq(lit(-8)))
        .filter(col("sex").neq(lit(-8)))
        .group_by([col("sex"), col("mar_stat")])
        .agg([col("health")
            .lt_eq(3)
            .cast(DataType::Int8)
            .sum()
            .alias("health_flag")])
        .sort(["sex", "mar_stat"], Default::default())
        .with_column(col("sex").replace_strict(
            lit(Series::from_iter(vec!["1", "2"])),
            lit(Series::from_iter(vec!["Female", "Male"])),
            None,
            Some(DataType::String),
        ))
        .with_column(col("mar_stat").replace_strict(
            lit(Series::from_iter(vec![1, 2, 3, 4, 5])),
            lit(Series::from_iter(vec![
                "Never married",
                "Married",
                "Separated",
                "Divorced ",
                "Widowed",
            ])),
            None,
            Some(DataType::String),
        ))
        .collect()
        .unwrap();

    println!("{}", &df);

    // === block_3

    // Transpose
    let df = pivot::pivot_stable(
        &df,
        ["sex"],
        Some(["mar_stat"]),
        Some(["health_flag"]),
        false,
        None,
        None,
    )
    .unwrap()
    .drop("mar_stat")
    .unwrap();

    println!("{}", &df);

    // === block_4

    // Create an array of arrays of float64
    let cols = df
        .get_columns()
        .iter()
        .map(|c| {
            c.as_materialized_series()
                .to_float()
                .unwrap()
                .f64()
                .unwrap()
                .to_vec_null_aware()
                .left()
                .unwrap()
        })
        .collect::<Vec<Vec<f64>>>();

    // === block_5

    // Perform Chi-Square Test for Independence
    let alpha = 0.05;
    let result = independence(&cols, alpha).unwrap();

    println!(
        "Result: {}\nP-value: {}\nNull hypothesis: {}\nReject null: {}",
        result.test_statistic, result.p_value, result.null_hypothesis, result.reject_null
    );

    // === end
}
