use human_bytes::human_bytes;
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read CSV into
    let df = CsvReadOptions::default()
        .with_low_memory(true)
        .try_into_reader_with_file_path(Some("./data/lfs_large.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    println!("{}", human_bytes(df.estimated_size() as f64));

    Ok(())
}
