// :dep polars = { version = "0.45", features = ["lazy", "parquet"] }

use polars::prelude::*;

fn main() {
    // Get all files in path
    let paths = std::fs::read_dir("./data/lfs_csv").unwrap();

    // For each file, save as Parquet
    for path in paths {
        let path_csv = path.unwrap().path();
        let file_name = std::path::Path::new(&path_csv)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let path_parquet = format!("./data/lfs_parquet/{file_name}.parquet");

        // Read CSV
        let mut df = LazyCsvReader::new(path_csv.clone())
            .with_infer_schema_length(Some(10_000)) // Default 100, missing = String
            .with_has_header(true)
            .finish()
            .unwrap()
            .collect() // Can't collect in finish below
            .unwrap();

        // Write Parquet
        let mut file = std::fs::File::create(path_parquet).unwrap();
        ParquetWriter::new(&mut file).finish(&mut df).unwrap();
    }
}
