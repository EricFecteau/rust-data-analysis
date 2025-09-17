// === evcxr
// :dep polars = { version = "0.51", features = ["lazy", "parquet", "pivot"] }
// :dep hypors = "0.2"

// === imports
use hypors::common::types::TailType;
use hypors::mann_whitney::u_test;
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/lfs_large/part"), args).unwrap();

    // Hourly earnings by gender (Jan 2020)
    let df = lf
        .clone()
        .filter(col("survyear").eq(lit(2020)))
        .filter(col("survmnth").eq(lit(1)))
        .filter(col("hrlyearn").is_null().not())
        .select([
            col("gender").replace_strict(
                lit(Series::from_iter(vec!["1", "2"])),
                lit(Series::from_iter(vec!["Men+", "Women+"])),
                None,
                Some(DataType::String),
            ),
            (col("hrlyearn").cast(DataType::Float64) / lit(100)),
        ])
        .with_row_index("index", None)
        .collect()
        .unwrap();

    println!("{}", &df);

    // === block_2

    // Transpose
    let df = pivot::pivot_stable(
        &df,
        ["gender"],
        Some(["index"]),
        Some(["hrlyearn"]),
        false,
        None,
        None,
    )
    .unwrap()
    .drop("index")
    .unwrap();

    println!("{}", &df);

    // === block_3

    // Create Vec<Series> for MWU
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

    // === block_4

    // Perform the Mann-Whitney U test
    let alpha = 0.05;
    let result = u_test(cols[0].clone(), cols[1].clone(), alpha, TailType::Two).unwrap();

    println!(
        "U-statistic: {}\nP-value: {}\nNull hypothesis: {}\nReject null: {}",
        result.test_statistic, result.p_value, result.null_hypothesis, result.reject_null
    );

    // === end
}
