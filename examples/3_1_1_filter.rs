// === evcxr
// :dep polars = { version = "0.49", features = ["lazy", "parquet", "is_in"] }

// === imports
use polars::prelude::*;

// === main
fn main() {
    // === chunk_1

    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // === chunk_2

    //Filtering the data in multiple steps
    let lf_filt = lf
        .clone()
        .filter(col("survyear").eq(lit(2023)))
        .filter(col("survmnth").gt(lit(6)))
        .filter(col("hrlyearn").is_not_null());

    // === end

    println!("{}", lf_filt.limit(5).collect().unwrap());

    // === chunk_3

    // Filtering the data in one step
    let lf_filt = lf.clone().filter(
        col("survyear")
            .eq(lit(2023))
            .and(col("survmnth").gt(lit(6)))
            .and(col("hrlyearn").is_not_null()),
    );

    // === end

    println!("{}", lf_filt.limit(5).collect().unwrap());

    // === chunk_4

    // ((survyear == 2023 & survmnt > 6) | (survyear == 2024 & survmnt <= 6))
    let expr = (col("survyear")
        .eq(lit(2023))
        .and(col("survmnth").gt(lit(6))))
    .or(col("survyear")
        .eq(lit(2024))
        .and(col("survmnth").lt_eq(lit(6))));

    println!("{expr}"); // You can print it

    // === chunk_5

    // Apply the expression to a LazyFrame
    let lf_filt = lf.clone().filter(expr);

    // === end

    println!("{}", lf_filt.limit(5).collect().unwrap());

    // === chunk_6

    // Using `is_in` crate feature with literals
    let lf_filt = lf.clone().filter(col("survyear").is_in(
        lit(Series::from_iter(vec![2022, 2023, 2024])).implode(),
        false,
    ));

    // === end

    println!("{}", lf_filt.limit(5).collect().unwrap());
}
