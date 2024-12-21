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

    println!("{:?}", cols);

    // Select some columns by name & with regex & with rename
    let sub_lf = lf.select([
        col("^surv.*$"), // survyear, survmnth
        col("prov"),
        col("hrlyearn").alias("hourly_wages"),
    ]);

    // Print selected column (top 5 values)
    println!("{}", sub_lf.limit(5).collect().unwrap())

    // Filtering the data

    // let e11_filter = df
    // .lazy()
    // .filter(col("CK").gt(lit(0)).and(col("CP").gt(lit(0))))
    // .collect()
    // .unwrap();

    // let e11_filter = df
    //     .lazy()
    //     .filter(all_exprs([cols(["CK", "CP"]).gt(lit(0))]))
    //     .collect()
    //     .unwrap();
}
