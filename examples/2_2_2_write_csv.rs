// === evcxr
// :dep polars = { version = "0.49", features = ["lazy"] }

// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1
    // Read `pub0124.csv` as LazyFrame
    let lf = LazyCsvReader::new("./data/lfs_csv/pub0124.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    // Bring it into memory (by converting it to DataFrame)
    let mut df = lf.collect().unwrap();

    // === block_2

    // Write `pub0124.csv`
    let mut file = std::fs::File::create("./data/temp_data/pub0124.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();

    // === end
}
