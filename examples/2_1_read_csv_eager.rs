use human_bytes::human_bytes;
use polars::prelude::*;

fn main() {
    // Read CSV into memory
    let df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("./data/lfs_csv/pub0124.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    // Print df to log
    println!("{}", &df);

    // Print size of df (in MiB)
    println!("{}", human_bytes(df.estimated_size() as f64));
}
