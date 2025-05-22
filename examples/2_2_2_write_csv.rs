// :dep polars = { version = "0.48", features = ["lazy"] }

use polars::prelude::*;

fn main() {
    // Read `pub0124.csv` as LazyFrame
    let lf = LazyCsvReader::new("./data/lfs_csv/pub0124.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    // Bring it into memory (by converting it to DataFrame)
    let mut df = lf.collect().unwrap();

    // Write `pub0124.csv`
    let mut file = std::fs::File::create("./data/temp_data/pub0124.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();
}
