// === imports
use hypors::anova::anova;
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/lfs_large/part"), args).unwrap();

    // Hourly earnings by immigration category (Jan 2020)
    let df = lf
        .clone()
        .filter(col("survyear").eq(lit(2020)))
        .filter(col("survmnth").eq(lit(1)))
        .filter(col("hrlyearn").is_null().not())
        .select([
            (col("hrlyearn").cast(DataType::Float64) / lit(100)).alias("hrlyearn"),
            col("immig"),
        ])
        .sort(["immig"], Default::default())
        .with_column(col("immig").replace_strict(
            lit(Series::from_iter(vec!["1", "2", "3"])),
            lit(Series::from_iter(vec![
                "Immigrant (<= 10 years)",
                "Immigrant (> 10 years)",
                "Non-immigrant",
            ])),
            None,
            Some(DataType::String),
        ))
        .with_row_index("index", None)
        .collect()
        .unwrap();

    println!("{}", &df);

    // === block_2

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

    println!("{}", &df);

    // === block_3

    // Create Vec<Vec<f64>> for ANOVA
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

    // Perform one-way ANOVA
    let alpha = 0.05;
    let result = anova(&cols, alpha).unwrap();

    println!(
        "F-statistic: {}\nP-value: {}\nNull hypothesis: {}\nReject null: {}",
        result.test_statistic, result.p_value, result.null_hypothesis, result.reject_null
    );

    // === end
}
