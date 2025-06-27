// :dep polars = { version = "0.48", features = ["lazy", "parquet", "regex", "replace"] }

use polars::prelude::*;

fn main() {
    // Connect to parquet (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf_jan =
        LazyFrame::scan_parquet("./data/lfs_parquet/pub0123.parquet", args.clone()).unwrap();
    let lf_feb =
        LazyFrame::scan_parquet("./data/lfs_parquet/pub0223.parquet", args.clone()).unwrap();
    let lf_mar =
        LazyFrame::scan_parquet("./data/lfs_parquet/pub0323.parquet", args.clone()).unwrap();
    let lf_apr =
        LazyFrame::scan_parquet("./data/lfs_parquet/pub0423.parquet", args.clone()).unwrap();

    // Concatonate vertically two (or more) datasts
    let lf_jan_to_apr = concat(
        [
            lf_jan.clone(), // Cloned to use later
            lf_feb.clone(),
            lf_mar.clone(),
            lf_apr.clone(),
        ],
        UnionArgs::default(),
    )
    .unwrap();

    // See `survmnth` going from 1 to 4 for 2023
    println!("{}", lf_jan_to_apr.collect().unwrap());

    // Get data ready for join
    let lf_jan = lf_jan.select([col("rec_num"), col("hrlyearn").alias("earn_jan")]);
    let lf_feb = lf_feb.select([col("rec_num"), col("hrlyearn").alias("earn_feb")]);
    let lf_mar = lf_mar.select([col("rec_num"), col("hrlyearn").alias("earn_mar")]);
    let lf_apr = lf_apr.select([col("rec_num"), col("hrlyearn").alias("earn_apr")]);

    // Left join
    let longitudinal = lf_jan
        .left_join(lf_feb, col("rec_num"), col("rec_num"))
        .left_join(lf_mar, col("rec_num"), col("rec_num"))
        .left_join(lf_apr, col("rec_num"), col("rec_num"));

    println!("{}", longitudinal.collect().unwrap());
}
