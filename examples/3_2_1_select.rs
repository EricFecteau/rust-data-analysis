// :dep polars = { version = "0.48", features = ["lazy", "parquet", "regex"] }

use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let mut lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Get names of columns
    let cols: Vec<String> = lf
        .collect_schema()
        .unwrap()
        .iter_names()
        .map(|c| c.to_owned().to_string())
        .collect();

    println!(
        "Vector of the {} variables in the LazyFrame: {:?}",
        cols.len(),
        cols
    );

    // Select some columns by name & with regex & with rename
    let lf = lf.select([
        col("^surv.*$"), // survyear, survmnth
        col("prov"),
        col("hrlyearn").alias("hourly_wages"),
        col("finalwt"),
    ]);

    // Print selected column (top 5 values)
    println!("{}", lf.clone().limit(5).collect().unwrap());

    // Drop variables
    let lf = lf.drop([col("prov"), col("hourly_wages")]);

    // Print selected column (top 5 values)
    println!("{}", lf.clone().limit(5).collect().unwrap());
}
