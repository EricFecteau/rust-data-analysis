// :dep polars = { version = "0.45", features = ["lazy"] }

use polars::prelude::*;

fn main() {
    // Read `pub0824.csv` as LazyFrame
    let lf = LazyCsvReader::new("./data/lfs_csv/pub0824.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    // Bring it into memory (by converting it to DataFrame)
    let mut df = lf.collect().unwrap();

    // Write `pub0824.csv`
    let mut file = std::fs::File::create("./data/lfs_csv/pub0824.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();
}
