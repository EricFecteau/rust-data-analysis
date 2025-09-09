// === evcxr
// :dep polars = { version = "0.50", features = ["lazy", "parquet", "is_in"] }

// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/lfs_large/part"), args).unwrap();

    // === block_2

    //Filtering the data in multiple steps
    let lf_filt_mult = lf
        .clone()
        .filter(col("survyear").eq(lit(2023)))
        .filter(col("survmnth").gt(lit(6)))
        .filter(col("hrlyearn").is_not_null());

    // === block_3

    // Filtering the data in one step
    let lf_filt_one = lf.clone().filter(
        col("survyear")
            .eq(lit(2023))
            .and(col("survmnth").gt(lit(6)))
            .and(col("hrlyearn").is_not_null()),
    );

    // === block_4

    // ((survyear == 2023 & survmnt > 6) | (survyear == 2024 & survmnt <= 6))
    let expr = (col("survyear")
        .eq(lit(2023))
        .and(col("survmnth").gt(lit(6))))
    .or(col("survyear")
        .eq(lit(2024))
        .and(col("survmnth").lt_eq(lit(6))));

    println!("{expr}"); // You can print it

    // === block_5

    // Apply the expression to a LazyFrame
    let lf_filt_complex = lf.clone().filter(expr);

    // === block_6

    // Using `is_in` crate feature with literals
    let lf_filt_is_in = lf.clone().filter(col("survyear").is_in(
        lit(Series::from_iter(vec![2022, 2023, 2024])).implode(),
        false,
    ));

    // === end

    println!("{}", lf_filt_mult.limit(5).collect().unwrap());
    println!("{}", lf_filt_one.limit(5).collect().unwrap());
    println!("{}", lf_filt_complex.limit(5).collect().unwrap());
    println!("{}", lf_filt_is_in.limit(5).collect().unwrap());
}
