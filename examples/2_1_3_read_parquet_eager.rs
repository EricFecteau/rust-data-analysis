use human_bytes::human_bytes;
use polars::prelude::*;
use std::fs::File;

fn main() {
    // Read Parquet into memory
    let mut file = File::open("./data/lfs_parquet/pub0124.parquet").unwrap();
    let df = ParquetReader::new(&mut file).finish().unwrap();

    // Print size of df (in MiB)
    println!("{}", human_bytes(df.estimated_size() as f64));

    // Print df to log
    println!("{}", &df);
}
