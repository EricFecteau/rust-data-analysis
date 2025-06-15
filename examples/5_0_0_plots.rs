use polars::prelude::{pivot::pivot_stable, *};

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Modify var
    let lf = lf
        .filter(col("hrlyearn").is_not_null())
        .with_column((col("hrlyearn").cast(DataType::Float64) / lit(100)).alias("hourly_wages"));

    // Mean wage over time
}
