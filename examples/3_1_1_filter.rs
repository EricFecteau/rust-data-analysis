// :dep polars = { version = "0.49", features = ["lazy", "parquet", "is_in"] }

use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    //Filtering the data in multiple steps
    let lf_filt = lf
        .clone()
        .filter(col("survyear").eq(lit(2023)))
        .filter(col("survmnth").gt(lit(6)))
        .filter(col("hrlyearn").is_not_null());

    println!("{}", lf_filt.limit(5).collect().unwrap());

    // Filtering the data in one step
    let lf_filt = lf.clone().filter(
        col("survyear")
            .eq(lit(2023))
            .and(col("survmnth").gt(lit(6)))
            .and(col("hrlyearn").is_not_null()),
    );

    println!("{}", lf_filt.limit(5).collect().unwrap());

    // Complex expression
    let expr = (col("survyear")
        .eq(lit(2023))
        .and(col("survmnth").gt(lit(6))))
    .or(col("survyear")
        .eq(lit(2024))
        .and(col("survmnth").lt_eq(lit(6))));

    println!("Expression: {expr}"); // You can print it

    // Apply the expression to a LazyFrame
    let lf_filt = lf.clone().filter(expr);

    println!("{}", lf_filt.limit(5).collect().unwrap());

    // Using `is_in` crate feature with literals
    let lf_filt = lf.clone().filter(col("survyear").is_in(
        lit(Series::from_iter(vec![2022, 2023, 2024])).implode(),
        false,
    ));

    println!("{}", lf_filt.limit(5).collect().unwrap());
}
