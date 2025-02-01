// :dep polars = { version = "0.45", features = ["lazy", "parquet", "regex"] }

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

    // Can't use created columns in the same `select()`, but you can add new colum(s) with `with_column()`
    let lf = lf.with_column(
        (col("five") + col("ten")).alias("fifteen"), // add two columns
    );

    println!("{}", lf.clone().limit(5).collect().unwrap());

    // Cast the value from an `i64` to a `f64` and modify it (divide by 100)
    let lf = lf
        .drop([col("five"), col("ten"), col("fifteen")])
        .filter(col("hourly_wages").is_not_null())
        .with_column(
            (col("hourly_wages").cast(DataType::Float64) / lit(100)).alias("hourly_wages"),
        );

    println!("{}", lf.clone().limit(5).collect().unwrap());

    //car
    let lf = lf.with_column(
        when(col("hourly_wages").lt_eq(lit(10.00)))
            .then(lit("Low"))
            .when(
                col("hourly_wages")
                    .gt(lit(10.00))
                    .and(col("hourly_wages").lt_eq(lit(30.00))),
            )
            .then(lit("Medium"))
            .otherwise(lit("High"))
            .alias("wage_cat"),
    );

    println!("{}", lf.clone().limit(5).collect().unwrap());
}
