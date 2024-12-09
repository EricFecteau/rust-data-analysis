use polars::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

fn main() {
    // Get all files in path
    let mut paths = fs::read_dir("./data/lfs_parquet").unwrap();

    let parquet = paths.next().unwrap().unwrap().path();
    let mut df = ParquetReader::new(File::open(parquet).unwrap())
        .finish()
        .unwrap();

    // For all Parquet files
    for path in paths {
        let parquet = path.unwrap().path();

        // Read-in and stack new parquet file
        df.vstack_mut(
            &ParquetReader::new(File::open(parquet).unwrap())
                .finish()
                .unwrap(),
        )
        .unwrap();

        // Recommended in the docs
        df.align_chunks_par();
    }

    // Write large file as `lfs_large.csv`
    let mut file = std::fs::File::create("./data/lfs_large/lfs.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();

    // Write Single Parquet
    let mut file = File::create("./data/lfs_large/lfs.parquet").unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();

    // Write Partitioned Parquet (by survyear, survmnth) - unstable according to the docs
    let stats = StatisticsOptions {
        min_value: true,
        max_value: true,
        distinct_count: true,
        null_count: true,
    };

    let write_options = ParquetWriteOptions {
        compression: ParquetCompression::Zstd(Some(ZstdLevel::try_new(10).unwrap())),
        statistics: stats,
        row_group_size: Some(512_usize.pow(2)),
        data_page_size: Some(1024_usize.pow(2)),
        maintain_order: true,
    };

    write_partitioned_dataset(
        &mut df,
        Path::new("./data/lfs_large/part/"),
        vec!["survyear", "survmnth"],
        &write_options,
        4294967296,
    )
    .unwrap();
}
