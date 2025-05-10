// :dep polars = { version = "0.46", features = ["lazy", "parquet", "pivot"] }
// :dep hypors = "0.2"

use df_interchange::Interchange;
use hypors::anova::anova;
use polars::prelude::*;

// 1	Immigrant, landed 10 or less years earlier
// 2	Immigrant, landed more than 10 years earlier
// 3	Non-immigrant

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    //
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
        .with_row_index("index", None)
        .collect()
        .unwrap();

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

    let df = Interchange::from_polars_0_46(df)
        .unwrap()
        .to_polars_0_43()
        .unwrap();

    // Create Vec<Series> for ANOVA
    let cols = df.get_columns();

    // Perform one-way ANOVA
    let result = anova(
        &[
            &cols[0].drop_nulls(),
            &cols[1].drop_nulls(),
            &cols[2].drop_nulls(),
        ],
        0.05,
    )
    .unwrap();

    println!(
        "F-statistic: {}, p-value: {}",
        result.test_statistic, result.p_value
    );
}
