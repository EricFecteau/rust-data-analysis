use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Add new variables from literals and columns
    let lf = lf.select([
        col("^surv.*$"),                         // keep survyear, survmnth
        col("hrlyearn").alias("hourly_wages"),   // keep hrlyearn as hourly_wages
        lit(5).alias("five"),                    // add single value literal
        (lit(5) + lit(7) - lit(2)).alias("ten"), // add single value from two or more literals
    ]);

    println!("{}", lf.clone().limit(5).collect().unwrap());

    // Can't use created columns in the same `select()`
    let lf = lf.select([
        all(), // keep all previously kept variables (same as col("*"))
        (col("five") + col("ten")).alias("fifteen"), // add two columns
    ]);

    println!("{}", lf.clone().limit(5).collect().unwrap());

    // Cast the value from an `i64` to a `f64` and modify it (divide by 100)
    let lf = lf
        .drop([col("five"), col("ten"), col("fifteen")])
        .filter(col("hourly_wages").is_not_null())
        .select([
            all(), // keep all previously kept variables (same as col("*"))
            (col("hourly_wages").cast(DataType::Float64) / lit(100)).alias("wages_dollars"),
        ]);

    println!("{}", lf.clone().limit(5).collect().unwrap());

    let lf = lf.select([
        all(),
        when(col("wages_dollars").lt_eq(lit(10.00)))
            .then(lit("Low"))
            .when(
                col("wages_dollars")
                    .gt(lit(10.00))
                    .and(col("wages_dollars").lt_eq(lit(30.00))),
            )
            .then(lit("Medium"))
            .otherwise(lit("High"))
            .alias("wage_cat"),
    ]);

    println!("{}", lf.clone().limit(5).collect().unwrap());
}
