use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let lf: LazyFrame = LazyCsvReader::new("./data/lfs_csv/pub0824.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    // Collect the LazyFrame into a DataFrame
    let df = lf.collect().unwrap();

    println!("{}", &df);
}
