use polars::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

fn main() {
    // Get all files in path
    let paths = fs::read_dir("./data/lfs_parquet").unwrap();

    let mut lf_vec = vec![];

    for path in paths {
        let parquet = path.unwrap().path();

        let args = ScanArgsParquet::default();
        let lf = LazyFrame::scan_parquet(parquet, args.clone()).unwrap();

        lf_vec.push(lf);
    }

    let union_args = UnionArgs::default();
    let lf = concat(lf_vec, union_args).unwrap();

    // Bring to memory (large)
    let mut df = lf.collect().unwrap();

    // Write large file as `lfs_large.csv`
    let mut file = std::fs::File::create("./data/lfs_large/lfs.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();

    // Write Single Parquet
    let mut file = File::create("./data/lfs_large/lfs.parquet").unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();

    // Write Partitioned Parquet (by survyear, survmnth) - unstable according to the docs
    write_partitioned_dataset(
        &mut df,
        Path::new("./data/lfs_large/part/"),
        vec!["survyear", "survmnth"],
        &ParquetWriteOptions::default(),
        4294967296,
    )
    .unwrap();
}
