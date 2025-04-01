// :dep polars = { version = "0.46", features = ["lazy", "parquet", "regex"] }

use polars::prelude::pivot::pivot_stable;
use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Mean hourly wage by year and province (for those with an hourly wage) - unweighted
    let lf = lf
        .filter(col("hrlyearn").is_not_null())
        .group_by([col("survyear"), col("prov")])
        .agg([(col("hrlyearn") / lit(100))
            .mean()
            .alias("mean_hrlyearn")
            .round(2)])
        .sort(["survyear", "prov"], SortMultipleOptions::default());

    println!("{}", lf.clone().collect().unwrap());

    // Pivot wider / pivot
    let df_wide = pivot_stable(
        &lf.collect().unwrap(),
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
            ["10", "11", "12", "13", "24", "35", "46", "47", "48", "59"],
            ["survyear"],
        )
        .unwrap();

    println!("{}", &df_long);
}
