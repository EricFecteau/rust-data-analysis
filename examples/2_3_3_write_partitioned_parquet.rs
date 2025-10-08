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

    // This functionality is unstable according to the docs
    write_partitioned_dataset(
        &mut df,
        PlPath::from_str("./data/temp_data/partitioned/").as_ref(),
        vec!["region".into(), "age_group".into()],
        &ParquetWriteOptions::default(),
        None,
        4294967296,
    )
    .unwrap();

    // === end
}
