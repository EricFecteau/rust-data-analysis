use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let lf = LazyCsvReader::new("./data/lfs_large/lfs.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    // Print first 5 rows
    println!("{}", lf.limit(5).collect().unwrap());
}
