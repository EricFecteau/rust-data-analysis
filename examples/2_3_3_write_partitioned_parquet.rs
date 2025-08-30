// === evcxr
// :dep polars = { version = "0.49", features = ["lazy", "parquet"] }

// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Read `pub0124.csv` as LazyFrame
    let lf = LazyCsvReader::new(PlPath::from_str("./data/lfs_csv/pub0124.csv"))
        .with_has_header(true)
        .finish()
        .unwrap();

    // Bring it into memory (by converting it to DataFrame)
    let mut df = lf.collect().unwrap();

    // === block_2

    // This functionality is unstable according to the docs
    write_partitioned_dataset(
        &mut df,
        PlPath::from_str("./data/temp_data/_temp/").as_ref(),
        vec!["prov".into(), "gender".into()],
        &ParquetWriteOptions::default(),
        None,
        4294967296,
    )
    .unwrap();

    // === end
}
