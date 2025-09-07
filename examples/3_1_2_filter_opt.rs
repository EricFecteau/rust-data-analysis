// === evcxr
// :dep polars = { version = "0.50", features = ["lazy", "parquet"] }

// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame (one large parquet file)
    let args = ScanArgsParquet::default();
    let lf_one =
        LazyFrame::scan_parquet(PlPath::from_str("./data/lfs_large/lfs.parquet"), args).unwrap();

    // Filter it
    let lf_one = lf_one
        .filter(col("survyear").eq(lit(2023)))
        .filter(col("survmnth").gt(lit(6)))
        .filter(col("hrlyearn").is_not_null());

    // === block_2

    // Connect to LazyFrame (partitioned parquet file)
    let args = ScanArgsParquet::default();
    let lf_part = LazyFrame::scan_parquet(PlPath::from_str("./data/lfs_large/part"), args).unwrap();

    // Filter it
    let lf_part = lf_part
        .filter(col("survyear").eq(lit(2023)))
        .filter(col("survmnth").gt(lit(6)))
        .filter(col("hrlyearn").is_not_null());

    // === block_3

    // Unoptimized
    println!(
        "\nUnoptimized single-parquet file:\n\n{}",
        lf_one.explain(false).unwrap()
    );

    // === block_4

    println!(
        "\nUnoptimized multi-parquet file:\n\n{}",
        lf_part.explain(false).unwrap()
    );

    // Optimized

    // === block_5
    println!(
        "\nOptimized single-parquet file:\n\n{}",
        lf_one.explain(true).unwrap()
    );

    // === block_6
    println!(
        "\nOptimized multi-parquet file:\n\n{}",
        lf_part.explain(true).unwrap()
    );

    // === block_7
    let before = std::time::Instant::now();
    let _ = lf_one.select([col("hrlyearn")]).mean().collect().unwrap();
    println!("Elapsed time: {:.2?}", before.elapsed());

    let before = std::time::Instant::now();
    let _ = lf_part.select([col("hrlyearn")]).mean().collect().unwrap();
    println!("Elapsed time: {:.2?}", before.elapsed());

    // === end
}
