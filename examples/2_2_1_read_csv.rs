// === evcxr
// :dep polars = { version = "0.49", features = ["lazy"] }

// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1
    // Connect to LazyFrame (no data is brought into memory)
    let lf = LazyCsvReader::new(PlPath::from_str("./data/lfs_csv/pub0124.csv"))
        .with_has_header(true)
        .finish()
        .unwrap();

    // === block_2

    println!("{}", lf.limit(5).collect().unwrap());

    // === end
}
