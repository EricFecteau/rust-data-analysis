// :dep polars = { version = "0.46", features = ["lazy", "parquet", "pivot"] }
// :dep hypors = "0.2"

use hypors::anova::anova;
use polars::prelude::*;

fn main() {
    let lf = LazyCsvReader::new("/home/eric/R/large_csv/anova.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    // Transpose (for ANOVA)
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

    // Create Vec<Series> for ANOVA
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

    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Count individuals with paid overtime by gender and marital status
    let df = lf
        .clone()
        .filter(col("survyear").eq(lit(2010)))
        .filter(col("survmnth").eq(lit(1)))
        .filter(col("hrlyearn").is_null().not())
        .select([
            (col("hrlyearn").cast(DataType::Float64) / lit(100)).alias("hrlyearn"),
            col("immig"),
        ])
        .sort(["immig"], Default::default())
        .with_row_index("index", None)
        .collect()
        .unwrap();

    println!("{:?}", df);

    // Transpose
    let df = pivot::pivot_stable(
        &df,
        ["immig"],
        Some(["index"]),
        Some(["hrlyearn"]),
        false,
        None,
        None,
    )
    .unwrap()
    .drop("index")
    .unwrap();

    println!("{:?}", df);

    let cols: Vec<Series> = df
        .get_columns()
        .iter()
        .map(|c| c.as_materialized_series().to_float().unwrap().drop_nulls())
        .collect();

    println!("Start ANOVA");

    // Perform one-way ANOVA
    let result = anova(&[&cols[0], &cols[1], &cols[2]], 0.05).unwrap();

    println!(
        "F-statistic: {}, p-value: {}",
        result.test_statistic, result.p_value
    );

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
