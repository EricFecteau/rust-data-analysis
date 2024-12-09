use polars::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

fn main() {
    // Get all files in path
    let paths = fs::read_dir("./data/lfs_csv").unwrap();

    // For each file, save as Parquet
    for path in paths {
        let path_csv = path.unwrap().path();
        let file_name = Path::new(&path_csv).file_stem().unwrap().to_str().unwrap();
        let path_parquet = format!("./data/lfs_parquet/{file_name}.parquet");

        // Read CSV
        let mut df = CsvReadOptions::default()
            .with_infer_schema_length(Some(10_000)) // Default 100, missing = String
            .try_into_reader_with_file_path(Some(path_csv.clone()))
            .unwrap()
            .finish()
            .unwrap();

        // Write Parquet
        let mut file = File::create(path_parquet).unwrap();
        ParquetWriter::new(&mut file).finish(&mut df).unwrap();
    }
}
