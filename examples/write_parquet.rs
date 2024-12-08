use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("./data/lfs_csv/pub0824.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    let mut file = std::fs::File::create("./data/pub0824.parquet").unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();

    Ok(())
}
