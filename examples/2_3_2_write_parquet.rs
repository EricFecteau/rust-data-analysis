// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Read `census_0.csv` as LazyFrame
    let lf = LazyCsvReader::new(PlPath::from_str("./data/csv/census_0.csv"))
        .with_has_header(true)
        .finish()
        .unwrap();

    // Bring it into memory (by converting it to DataFrame)
    let mut df = lf.collect().unwrap();

    // === block_2

    // Write `pub0124.parquet`
    let mut file = std::fs::File::create("./data/temp_data/census_0.parquet").unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();

    // === end
}
