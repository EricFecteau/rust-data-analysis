// === evcxr
// :dep polars = { version = "0.50", features = ["lazy", "parquet"] }

// === imports
use polars::prelude::*;

// === main
fn main() {
    // === program
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
        let path_csv_string = path_csv.into_os_string().into_string().unwrap();

        // Read CSV
        let mut df = LazyCsvReader::new(PlPath::from_string(path_csv_string.clone()))
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
    // === end
}
