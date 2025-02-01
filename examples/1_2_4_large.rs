// :dep polars = { version = "0.46", features = ["lazy", "parquet"] }

use polars::prelude::*;

fn main() {
    // Get all files in path
    let paths = std::fs::read_dir("./data/lfs_parquet").unwrap();

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
    let mut file = std::fs::File::create("./data/lfs_large/lfs.parquet").unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();

    // Write Partitioned Parquet (by survyear, survmnth) - unstable according to the docs
    write_partitioned_dataset(
        &mut df,
        std::path::Path::new("./data/lfs_large/part/"),
        vec!["survyear".into(), "survmnth".into()],
        &ParquetWriteOptions::default(),
        None,
        4294967296,
    )
    .unwrap();
}
