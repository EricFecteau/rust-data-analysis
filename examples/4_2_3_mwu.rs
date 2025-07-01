// :dep polars = { version = "0.49", features = ["lazy", "parquet", "pivot"] }
// :dep hypors = "0.2"

use df_interchange::Interchange;
use hypors::common::types::TailType;
use hypors::mann_whitney::u_test;
use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

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

    // Convert from Polars 0.49 to Polars 0.43
    let df = Interchange::from_polars_0_49(df)
        .unwrap()
        .to_polars_0_43()
        .unwrap();

    // Create Vec<Series> for MWU
    let cols = df.get_columns();

    // Perform the Mann-Whiteny U test
    let alpha = 0.05;
    let result = u_test(
        &cols[0].drop_nulls(),
        &cols[1].drop_nulls(),
        alpha,
        TailType::Two,
    )
    .unwrap();

    println!(
        "U-statistic: {}\nP-value: {}\nNull hypothesis: {}\nReject null: {}",
        result.test_statistic, result.p_value, result.null_hypothesis, result.reject_null
    );
}
