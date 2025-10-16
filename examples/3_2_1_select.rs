// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame
    let args = ScanArgsParquet::default();
    let mut lf =
        LazyFrame::scan_parquet(PlPath::from_str("./data/large/partitioned"), args).unwrap();

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
        col("^age.*$"), // survyear, survmnth
        col("region"),
        col("income").alias("yearly_income"),
    ]);

    // Print selected column (top 5 values)
    println!("{}", lf.clone().limit(5).collect().unwrap());

    // === block_4

    // Drop variables (better to simply select the columns needed)
    let lf = lf.select([all().exclude_cols(["region", "yearly_income"]).as_expr()]);

    // Print selected column (top 5 values)
    println!("{}", lf.clone().limit(5).collect().unwrap());

    // === end
}
