// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/large/partitioned"), args).unwrap();

    // === block_2

    // Add new variables from literals
    let lf = lf.select([
        col("age_group"),
        col("region"),
        col("income"),
        lit(5).alias("five"),                    // add single value literal
        (lit(5) + lit(7) - lit(2)).alias("ten"), // add single value from two or more literals
    ]);

    println!("{}", lf.clone().limit(5).collect().unwrap());

    // === block_3

    // Can't use created columns in the same `select()`, but you can add new column(s) with `with_column()`
    let lf = lf.with_column(
        (col("five") + col("ten")).alias("fifteen"), // add two columns
    );

    println!("{}", lf.clone().limit(5).collect().unwrap());

    // === block_4

    // Cast the value from an `i64` to a `f64` and modify it (add a inflation modifier)
    let lf = lf
        .select([all().exclude_cols(["five", "ten", "fifteen"]).as_expr()])
        .filter(col("income").is_not_null())
        .with_column((col("income").cast(DataType::Float64) * lit(1.02)).alias("income_infl"));

    println!("{}", lf.clone().limit(5).collect().unwrap());

    // === block_5

    // Create categorical variables
    let lf = lf.with_column(
        when(col("income_infl").lt_eq(lit(30_000)))
            .then(lit("Low"))
            .when(
                col("income_infl")
                    .gt(lit(30000))
                    .and(col("income_infl").lt_eq(lit(70_000))),
            )
            .then(lit("Medium"))
            .otherwise(lit("High"))
            .alias("income_cat"),
    );

    println!("{}", lf.clone().limit(5).collect().unwrap());

    // === block_6

    // Change alpha-numeric region code to the region name
    let lf = lf.with_column(col("region").replace_strict(
        lit(Series::from_iter(vec![
            "E12000001",
            "E12000002",
            "E12000003",
            "E12000004",
            "E12000005",
            "E12000006",
            "E12000007",
            "E12000008",
            "E12000009",
            "W92000004",
        ])),
        lit(Series::from_iter(vec![
            "North East",
            "North West",
            "Yorkshire and The Humber",
            "East Midlands",
            "West Midlands",
            "East of England",
            "London",
            "South East",
            "South West",
            "Wales",
        ])),
        None,
        Some(DataType::String),
    ));

    println!("{}", lf.clone().limit(5).collect().unwrap());

    // === end
}
