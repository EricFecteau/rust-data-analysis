// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1
    // Connect to LazyFrame (no data is brought into memory)
    let lf = LazyCsvReader::new(PlPath::from_str("./data/large/census.csv"))
        .with_has_header(true)
        .finish()
        .unwrap();

    // === block_2

    println!("{}", lf.limit(5).collect().unwrap());

    // === end
}
