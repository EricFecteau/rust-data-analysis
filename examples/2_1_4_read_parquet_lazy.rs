use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/lfs.parquet", args).unwrap();

    // Count the number of rows
    let count = lf.select([len().alias("count")]).collect().unwrap();

    // Print the count df
    println!("{}", &count);

    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Count the number of rows
    let count = lf.select([len().alias("count")]).collect().unwrap();

    // Print the count df
    println!("{}", &count);
}
