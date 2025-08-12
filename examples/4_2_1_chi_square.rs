// === evcxr
// :dep polars = { version = "0.49", features = ["lazy", "parquet", "pivot"] }
// :dep hypors = "0.2"

// === imports
use hypors::chi_square::independence;
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect to the parquet LFS data
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/lfs_large/part"), args).unwrap();

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
        .with_column(col("gender").replace_strict(
            lit(Series::from_iter(vec!["1", "2"])),
            lit(Series::from_iter(vec!["Men+", "Women+"])),
            None,
            Some(DataType::String),
        ))
        .with_column(col("marstat").replace_strict(
            lit(Series::from_iter(vec!["1", "2", "3", "4", "5", "6"])),
            lit(Series::from_iter(vec![
                "Married",
                "Common-law",
                "Widowed",
                "Separated",
                "Divorced",
                "Single",
            ])),
            None,
            Some(DataType::String),
        ))
        .collect()
        .unwrap();

    println!("{}", &df);

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

    println!("{}", &df);

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
    let alpha = 0.05;
    let result = independence(&cols, alpha).unwrap();

    println!(
        "Result: {}\nP-value: {}\nNull hypothesis: {}\nReject null: {}",
        result.test_statistic, result.p_value, result.null_hypothesis, result.reject_null
    );

    // === end
}
