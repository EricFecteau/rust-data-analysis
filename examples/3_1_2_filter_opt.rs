// :dep polars = { version = "0.49", features = ["lazy", "parquet"] }

use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (one large parquet file)
    let args = ScanArgsParquet::default();
    let lf_one =
        LazyFrame::scan_parquet(PlPath::from_str("./data/lfs_large/lfs.parquet"), args).unwrap();

    // Filter it
    let lf_one = lf_one
        .filter(col("survyear").eq(lit(2023)))
        .filter(col("survmnth").gt(lit(6)))
        .filter(col("hrlyearn").is_not_null());

    // Connect to LazyFrame (partitioned parquet file)
    let args = ScanArgsParquet::default();
    let lf_part = LazyFrame::scan_parquet(PlPath::from_str("./data/lfs_large/part"), args).unwrap();

    // Filter it
    let lf_part = lf_part
        .filter(col("survyear").eq(lit(2023)))
        .filter(col("survmnth").gt(lit(6)))
        .filter(col("hrlyearn").is_not_null());

    // Unoptimized
    println!(
        "\nUnoptimized single-parquet file:\n\n{}",
        lf_one.explain(false).unwrap()
    );
    println!(
        "\nUnoptimized multi-parquet file:\n\n{}",
        lf_part.explain(false).unwrap()
    );

    // Optimized
    println!(
        "\nOptimized single-parquet file:\n\n{}",
        lf_one.explain(true).unwrap()
    );
    println!(
        "\nOptimized multi-parquet file:\n\n{}",
        lf_part.explain(true).unwrap()
    );

    let before = std::time::Instant::now();
    let _ = lf_one.select([col("hrlyearn")]).mean().collect().unwrap();
    println!("Elapsed time: {:.2?}", before.elapsed());

    let before = std::time::Instant::now();
    let _ = lf_part.select([col("hrlyearn")]).mean().collect().unwrap();
    println!("Elapsed time: {:.2?}", before.elapsed());
}
