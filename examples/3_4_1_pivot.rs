// :dep polars = { version = "0.49", features = ["lazy", "parquet", "regex", "replace"] }

use polars::prelude::pivot::pivot_stable;
use polars::prelude::*;

fn main() {
    // Connect to parquet (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Mean hourly wage by year and province (for those with an hourly wage) - unweighted
    let lf = lf
        .filter(col("hrlyearn").is_not_null())
        .group_by([col("survyear"), col("prov")])
        .agg([(col("hrlyearn") / lit(100))
            .mean()
            .alias("mean_hrlyearn")
            .round(2, RoundMode::HalfAwayFromZero)])
        .sort(["survyear", "prov"], SortMultipleOptions::default());

    // Change numeric province code to alpha-code
    let df = lf
        .with_column(col("prov").replace_strict(
            lit(Series::from_iter(vec![
                "10", "11", "12", "13", "24", "35", "46", "47", "48", "59",
            ])),
            lit(Series::from_iter(vec![
                "NL", "PE", "NS", "NB", "QC", "ON", "MB", "SK", "AB", "BC",
            ])),
            None,
            Some(DataType::String),
        ))
        .collect()
        .unwrap();

    println!("{}", &df);

    // Pivot wider / pivot
    let df_wide = pivot_stable(
        &df,
        ["prov"],
        Some(["survyear"]),
        Some(["mean_hrlyearn"]),
        false,
        None,
        None,
    )
    .unwrap();

    println!("{}", &df_wide);

    // Pivot longer / unpivot
    let df_long = df_wide
        .unpivot(
            ["NL", "PE", "NS", "NB", "QC", "ON", "MB", "SK", "AB", "BC"],
            ["survyear"],
        )
        .unwrap();

    println!("{}", &df_long);
}
