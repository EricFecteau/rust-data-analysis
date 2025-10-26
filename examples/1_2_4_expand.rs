// === imports
use polars::prelude::*;

// === main
fn main() {
    // === program

    // Read CSV
    let lf = LazyCsvReader::new(PlPath::from_string("./data/raw/census.csv".to_string()))
        .with_infer_schema_length(Some(10_000)) // Default 100, missing = String
        .with_has_header(true)
        .finish()
        .unwrap();

    // Create 100x 1% sample
    for chunk in 0..100 {
        // Write csv
        let mut file = std::fs::File::create(format!("./data/csv/census_{chunk}.csv")).unwrap();

        let mut df = lf
            .clone()
            .with_column(lit(chunk).alias("chunk"))
            .collect()
            .unwrap();

        CsvWriter::new(&mut file).finish(&mut df).unwrap();
    }
    // === end
}
