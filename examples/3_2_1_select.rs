// === evcxr
// :dep polars = { version = "0.50", features = ["lazy", "parquet", "regex"] }

// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let mut lf = LazyFrame::scan_parquet(PlPath::from_str("./data/lfs_large/part"), args).unwrap();

    // === block_2

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

    // === block_3

    // Select some columns by name & with regex & with rename
    let lf = lf.select([
        col("^surv.*$"), // survyear, survmnth
        col("prov"),
        col("hrlyearn").alias("hourly_wages"),
        col("finalwt"),
    ]);

    // Print selected column (top 5 values)
    println!("{}", lf.clone().limit(5).collect().unwrap());

    // === block_4

    // Drop variables (better to simply select the columns needed)
    let lf = lf.select([all().exclude_cols(["prov", "hourly_wages"]).as_expr()]);

    // Print selected column (top 5 values)
    println!("{}", lf.clone().limit(5).collect().unwrap());

    // === end
}
