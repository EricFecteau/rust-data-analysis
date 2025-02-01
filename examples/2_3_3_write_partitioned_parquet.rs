// :dep polars = { version = "0.46", features = ["lazy", "parquet"] }

use polars::prelude::*;

fn main() {
    // Read `pub0824.csv` as LazyFrame
    let lf = LazyCsvReader::new("./data/lfs_csv/pub0824.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    // Bring it into memory (by converting it to DataFrame)
    let mut df = lf.collect().unwrap();

    // This functionality is unstable according to the docs
    write_partitioned_dataset(
        &mut df,
        std::path::Path::new("./data/_temp/"),
        vec!["prov".into(), "sex".into()],
        &ParquetWriteOptions::default(),
        None,
        4294967296,
    )
    .unwrap();

    // Delete the files to clean up
    let _ = std::fs::remove_dir_all("./data/_temp");
}
