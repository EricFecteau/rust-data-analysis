// === imports
use polars::prelude::*;
use std::fs::File;

// === main
fn main() {
    // === program
    // Get all files in path
    let paths = std::fs::read_dir("./data/parquet").unwrap();

    let mut lf_vec = vec![];

    for path in paths {
        let parquet = path.unwrap().path().into_os_string().into_string().unwrap();

        let args = ScanArgsParquet::default();
        let lf = LazyFrame::scan_parquet(PlPath::from_string(parquet), args.clone()).unwrap();

        lf_vec.push(lf);
    }

    // Create one big file
    let union_args = UnionArgs::default();
    let lf = concat(lf_vec, union_args).unwrap();

    // Drop weight for the large file
    let lf = lf.select([all().exclude_cols(["weight"]).as_expr()]);

    // Get regions
    let regions = lf
        .clone()
        .select([col("region").unique()])
        .collect()
        .unwrap()
        .column("region")
        .unwrap()
        .str()
        .unwrap()
        .drop_nulls()
        .into_no_null_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // Get age_group
    let ages = lf
        .clone()
        .select([col("age_group").unique()])
        .collect()
        .unwrap()
        .column("age_group")
        .unwrap()
        .i64()
        .unwrap()
        .to_vec_null_aware()
        .left()
        .unwrap();

    // Ready write large parquet file by batch
    let file = File::create("./data/large/census.parquet").unwrap();
    let schema: Arc<Schema> = lf.clone().collect_schema().unwrap();
    let mut pq_writer: polars::io::parquet::write::BatchedWriter<File> = ParquetWriter::new(file)
        .set_parallel(true)
        .batched(&schema)
        .unwrap();

    // Ready write large csv file by batch
    let file = File::create("./data/large/census.csv").unwrap();
    let schema: Arc<Schema> = lf.clone().collect_schema().unwrap();
    let mut csv_writer: polars::io::csv::write::BatchedWriter<File> =
        CsvWriter::new(file).batched(&schema).unwrap();

    // By region
    for region in regions {
        for age in ages.clone() {
            let mut chunk_df = lf
                .clone()
                .filter(col("region").eq(lit(region.clone())))
                .filter(col("age_group").eq(lit(age)))
                .collect()
                .unwrap();

            // Write Partitioned Parquet (by region and age_group) - unstable according to the docs
            write_partitioned_dataset(
                &mut chunk_df,
                PlPath::from_str("./data/large/partitioned/").as_ref(),
                vec!["region".into(), "age_group".into()],
                &ParquetWriteOptions::default(),
                None,
                4294967296,
            )
            .unwrap();

            // Write chunk to a large parquet file
            pq_writer.write_batch(&chunk_df).unwrap();
            csv_writer.write_batch(&chunk_df).unwrap();
        }
    }

    pq_writer.finish().unwrap();
    csv_writer.finish().unwrap();

    // === end
}
