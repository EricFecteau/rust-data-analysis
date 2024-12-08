use polars::prelude::*;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("./data/lfs_csv/pub0824.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    let stats = StatisticsOptions {
        min_value: true,
        max_value: true,
        distinct_count: true,
        null_count: true,
    };

    let write_options = ParquetWriteOptions {
        compression: ParquetCompression::Zstd(Some(ZstdLevel::try_new(10)?)),
        statistics: stats,
        row_group_size: Some(512_usize.pow(2)),
        data_page_size: Some(1024_usize.pow(2)),
        maintain_order: true,
    };

    // This functionality is unstable according to the docs
    write_partitioned_dataset(
        &mut df,
        Path::new("./data/part/"),
        vec!["PROV", "SEX"],
        &write_options,
        4294967296,
    )?;

    Ok(())
}
