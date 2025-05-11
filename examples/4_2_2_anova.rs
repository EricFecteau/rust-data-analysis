// :dep polars = { version = "0.46", features = ["lazy", "parquet", "pivot"] }
// :dep hypors = "0.2"

use df_interchange::Interchange;
use hypors::anova::anova;
use polars::prelude::*;

// 1	Immigrant (<= 10 years)
// 2	Immigrant (> 10 years)
// 3	Non-immigrant

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

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

    // Convert from Polars 0.46 to Polars 0.43
    let df = Interchange::from_polars_0_47(df)
        .unwrap()
        .to_polars_0_43()
        .unwrap();

    // Create Vec<Series> for ANOVA
    let cols = df.get_columns();

    // Perform one-way ANOVA
    let alpha = 0.05;
    let result = anova(
        &[
            &cols[0].drop_nulls(),
            &cols[1].drop_nulls(),
            &cols[2].drop_nulls(),
        ],
        alpha,
    )
    .unwrap();

    println!(
        "F-statistic: {}\nP-value: {}\nNull hypothesis: {}\nReject null: {}",
        result.test_statistic, result.p_value, result.null_hypothesis, result.reject_null
    );
}
