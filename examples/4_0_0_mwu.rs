// https://www.r-tutor.com/elementary-statistics/non-parametric-methods/mann-whitney-wilcoxon-test
// https://docs.rs/hypors/latest/hypors/mann_whitney/u/fn.u_test.html

// :dep polars = { version = "0.46", features = ["lazy", "parquet", "pivot"] }
// :dep hypors = "0.2"

use df_interchange::Interchange;
use hypors::common::types::TailType;
use hypors::mann_whitney::u_test;
use polars::prelude::*;

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
            col("gender"),
            (col("hrlyearn").cast(DataType::Float64) / lit(100)),
        ])
        .with_row_index("index", None)
        .collect()
        .unwrap();

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

    let df = Interchange::from_polars_0_46(df)
        .unwrap()
        .to_polars_0_43()
        .unwrap();

    let cols = df.get_columns();

    let result = u_test(
        &cols[0].drop_nulls(),
        &cols[1].drop_nulls(),
        0.05,
        TailType::Two,
    )
    .unwrap();

    // Verified
    println!("U Statistic: {}", result.test_statistic);
    println!("P-value: {}", result.p_value);
    println!("Reject Null: {}", result.reject_null);
}
