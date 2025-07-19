// === evcxr
// :dep polars = { version = "0.49", features = ["lazy", "parquet"] }

use std::fs::File;

// === imports
use polars::prelude::*;

// === main
fn main() {
    // === program
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

    // Get latest year available in the data
    let years = lf
        .clone()
        .select([col("survyear").unique()])
        .collect()
        .unwrap()
        .column("survyear")
        .unwrap()
        .i64()
        .unwrap()
        .to_vec_null_aware()
        .left()
        .unwrap();

    // Ready write large parquet file by batch
    let file = File::create("./data/lfs_large/lfs.parquet").unwrap();
    let schema = lf.clone().collect_schema().unwrap();
    let mut pq_writer = ParquetWriter::new(file)
        .set_parallel(true)
        .batched(&schema)
        .unwrap();

    // Ready write large csv file by batch
    let file = File::create("./data/lfs_large/lfs.csv").unwrap();
    let schema = lf.clone().collect_schema().unwrap();
    let mut csv_writer = CsvWriter::new(file).batched(&schema).unwrap();

    for y in years {
        // Collect one year of data
        let mut year_df = lf
            .clone()
            .filter(col("survyear").eq(lit(y)))
            .collect()
            .unwrap();

        // Write Partitioned Parquet (by survyear, survmnth) - unstable according to the docs
        write_partitioned_dataset(
            &mut year_df,
            std::path::Path::new("./data/lfs_large/part/"),
            vec!["survyear".into(), "survmnth".into()],
            &ParquetWriteOptions::default(),
            None,
            4294967296,
        )
        .unwrap();

        // Write 1 year to a large parquet file
        pq_writer.write_batch(&year_df).unwrap();
        csv_writer.write_batch(&year_df).unwrap();
    }

    pq_writer.finish().unwrap();
    csv_writer.finish().unwrap();

    // === end
}
