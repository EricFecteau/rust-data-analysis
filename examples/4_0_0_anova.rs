// :dep polars = { version = "0.46", features = ["lazy", "parquet", "pivot"] }
// :dep hypors = "0.2"

use hypors::anova::anova;
use polars::prelude::*;

fn main() {
    let lf = LazyCsvReader::new("/home/eric/R/large_csv/anova.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    // Transpose
    let df = pivot::pivot_stable(
        &lf.collect().unwrap(),
        ["species"],
        Some([""]),
        Some(["flipper_length_mm"]),
        false,
        None,
        None,
    )
    .unwrap()
    .drop("")
    .unwrap();

    // Create an array of arrays of float64
    let cols: Vec<Series> = df
        .get_columns()
        .iter()
        .map(|c| c.as_materialized_series().to_float().unwrap().drop_nulls())
        .collect();

    println!("{:?}", cols);

    // Perform one-way ANOVA
    let result = anova(&[&cols[0], &cols[1], &cols[2]], 0.05).unwrap();

    println!(
        "F-statistic: {}, p-value: {}",
        result.test_statistic, result.p_value
    );

    // // Connect to LazyFrame (no data is brought into memory)
    // let args = ScanArgsParquet::default();
    // let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // // Count individuals with paid overtime by sex and marital status
    // let df = lf
    //     .clone()
    //     .filter(col("paidot").is_null().not())
    //     .group_by([col("sex"), col("marstat")])
    //     .agg([col("paidot")
    //         .gt(0)
    //         .cast(DataType::Int8)
    //         .sum()
    //         .alias("ot_flag")])
    //     .sort(["sex", "marstat"], Default::default())
    //     .collect()
    //     .unwrap();

    // // Transpose
    // let df = pivot::pivot_stable(
    //     &df,
    //     ["sex"],
    //     Some(["marstat"]),
    //     Some(["ot_flag"]),
    //     false,
    //     None,
    //     None,
    // )
    // .unwrap()
    // .drop("marstat")
    // .unwrap();

    // // Create an array of arrays of float64
    // let cols = df
    //     .get_columns()
    //     .iter()
    //     .map(|c| {
    //         c.as_materialized_series()
    //             .to_float()
    //             .unwrap()
    //             .f64()
    //             .unwrap()
    //             .to_vec_null_aware()
    //             .left()
    //             .unwrap()
    //     })
    //     .collect::<Vec<Vec<f64>>>();

    // // Perform Chi-Square Test for Independence
    // let result = independence(&cols, 0.01).unwrap();
    // println!(
    //     "Result: {}\nP value: {}\nReject null: {}",
    //     result.test_statistic, result.p_value, result.reject_null
    // );
}
