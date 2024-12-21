use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let mut lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Select all columns (short for `col("*")`)
    let lf_no_limit = lf.clone().select([all()]);

    // Print selected column (top 5 values)
    println!("{}", lf_no_limit.limit(5).collect().unwrap());

    // Get names of columns
    let cols: Vec<String> = lf
        .collect_schema()
        .unwrap()
        .iter_names()
        .map(|c| c.to_owned().to_string())
        .collect();

    println!("{:?}", cols);

    // Select some columns by name & with regex & with rename
    let lf_limit = lf.clone().select([
        col("^surv.*$"), // survyear, survmnth
        col("prov"),
        col("hrlyearn").alias("hourly_wages"),
        col("finalwt"),
    ]);

    // Print selected column (top 5 values)
    println!("{}", lf_limit.explain(true).unwrap());
    println!("{}", lf_limit.limit(5).collect().unwrap());
}
