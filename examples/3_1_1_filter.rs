// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/large/partitioned"), args).unwrap();

    // === block_2

    //Filtering the data in multiple steps
    let lf_filt_mult = lf
        .clone()
        .filter(col("keep_type").eq(lit(1))) // Usual resident
        .filter(col("region").eq(lit("E12000007"))) // London
        .filter(col("age_group").gt_eq(lit(5))) // Aged 45+
        .filter(col("income").is_not_null());

    // === block_3

    // Filtering the data in one step
    let lf_filt_one = lf.clone().filter(
        col("keep_type")
            .eq(lit(1)) // Usual resident
            .and(col("region").eq(lit("E12000007"))) // London
            .and(col("age_group").gt_eq(lit(5))) // Aged 45+
            .and(col("income").is_not_null()),
    );

    // === block_4

    // ((region == "E12000001" & age_group >= 6) | (region == "E12000002" & age_group < 6))
    let expr = (col("region")
        .eq(lit("E12000001")) // North East
        .and(col("age_group").gt_eq(lit(6)))) // 55 and over
    .or(col("region")
        .eq(lit("E12000002")) // North West
        .and(col("age_group").lt_eq(lit(6)))); // 54 and under

    println!("{expr}"); // You can print it

    // === block_5

    // Apply the expression to a LazyFrame
    let lf_filt_complex = lf.clone().filter(expr);

    // === block_6

    // Using `is_in` crate feature with literals
    let lf_filt_is_in = lf
        .clone()
        .filter(col("industry").is_in(lit(Series::from_iter(vec![2, 4, 6, 8])).implode(), false));

    // === end

    println!("{}", lf_filt_mult.limit(5).collect().unwrap());
    println!("{}", lf_filt_one.limit(5).collect().unwrap());
    println!("{}", lf_filt_complex.limit(5).collect().unwrap());
    println!("{}", lf_filt_is_in.limit(5).collect().unwrap());
}
