// :dep polars = { version = "0.46", features = ["lazy", "parquet", "pivot"] }
// :dep hypors = "0.2"

use polars::prelude::*;

use hypors::chi_square::independence;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Count individuals with paid overtime by gender and marital status
    let df = lf
        .clone()
        .filter(col("paidot").is_null().not())
        .group_by([col("gender"), col("marstat")])
        .agg([col("paidot")
            .gt(0)
            .cast(DataType::Int8)
            .sum()
            .alias("ot_flag")])
        .sort(["gender", "marstat"], Default::default())
        .collect()
        .unwrap();

    // Transpose
    let df = pivot::pivot_stable(
        &df,
        ["gender"],
        Some(["marstat"]),
        Some(["ot_flag"]),
        false,
        None,
        None,
    )
    .unwrap()
    .drop("marstat")
    .unwrap();

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

    // Perform Chi-Square Test for Independence
    let result = independence(&cols, 0.01).unwrap();
    println!(
        "Result: {}\nP value: {}\nReject null: {}",
        result.test_statistic, result.p_value, result.reject_null
    );
}
