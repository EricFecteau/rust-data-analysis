use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let lf = LazyCsvReader::new("./data/lfs_large/lfs.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    // Count the number of rows
    let count = lf.select([len().alias("count")]).collect().unwrap();

    // Print the count df
    println!("{}", &count)
}
