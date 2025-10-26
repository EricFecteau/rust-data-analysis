use std::env;

// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    unsafe {
        env::set_var("POLARS_VERBOSE", "1");
    }

    // === block_2

    // Connect to LazyFrame (one large parquet file)
    let args = ScanArgsParquet::default();
    let lf_one =
        LazyFrame::scan_parquet(PlPath::from_str("./data/large/census.parquet"), args).unwrap();

    // Filter it
    let lf_one = lf_one
        .filter(col("region").eq(lit("E12000007"))) // London
        .filter(col("age_group").eq(lit(5))) // Age 45 to 54
        .filter(col("income").is_not_null());

    // === block_3

    // Connect to LazyFrame (partitioned parquet file)
    let args = ScanArgsParquet::default();
    let lf_part =
        LazyFrame::scan_parquet(PlPath::from_str("./data/large/partitioned"), args).unwrap();

    // Filter it
    let lf_part = lf_part
        .filter(col("region").eq(lit("E12000007"))) // London
        .filter(col("age_group").eq(lit(5))) // Age 45 to 54
        .filter(col("income").is_not_null());

    // === block_4

    let before = std::time::Instant::now();
    let _ = lf_one.collect().unwrap();
    println!("Elapsed time: {:.2?}", before.elapsed());

    let before = std::time::Instant::now();
    let _ = lf_part.collect().unwrap();
    println!("Elapsed time: {:.2?}", before.elapsed());

    // === end
}
