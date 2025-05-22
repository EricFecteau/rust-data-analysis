// :dep polars = { version = "0.48", features = ["lazy", "parquet"] }

use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/lfs.parquet", args).unwrap();

    // Print first 5 rows
    println!("{}", lf.limit(5).collect().unwrap());

    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Print first 5 rows
    println!("{}", lf.limit(5).collect().unwrap());
}
