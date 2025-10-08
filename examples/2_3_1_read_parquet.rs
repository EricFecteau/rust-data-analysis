// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf =
        LazyFrame::scan_parquet(PlPath::from_str("./data/large/census.parquet"), args).unwrap();

    // === block_end

    println!("{}", lf.limit(5).collect().unwrap());

    // === block_2

    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/large/partitioned"), args).unwrap();

    // === block_3

    println!("{}", lf.limit(5).collect().unwrap());

    // === end
}
