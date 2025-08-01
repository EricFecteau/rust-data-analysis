// :dep polars = { version = "0.49", features = ["lazy", "parquet", "regex"] }

use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/lfs_large/part"), args).unwrap();

    // Add new variables from literals
    let lf = lf.select([
        col("^surv.*$"),                         // keep survyear, survmnth
        col("prov"),                             // Province of residence
        col("hrlyearn").alias("hourly_wages"),   // keep hrlyearn as hourly_wages
        lit(5).alias("five"),                    // add single value literal
        (lit(5) + lit(7) - lit(2)).alias("ten"), // add single value from two or more literals
    ]);

    println!("{}", lf.clone().limit(5).collect().unwrap());

    // Can't use created columns in the same `select()`, but you can add new column(s) with `with_column()`
    let lf = lf.with_column(
        (col("five") + col("ten")).alias("fifteen"), // add two columns
    );

    println!("{}", lf.clone().limit(5).collect().unwrap());

    // Cast the value from an `i64` to a `f64` and modify it (divide by 100)
    let lf = lf
        .drop([col("five"), col("ten"), col("fifteen")])
        .filter(col("hourly_wages").is_not_null())
        .with_column(
            (col("hourly_wages").cast(DataType::Float64) / lit(100)).alias("wages_dollars"),
        );

    println!("{}", lf.clone().limit(5).collect().unwrap());

    // Create categorical variables
    let lf = lf.with_column(
        when(col("wages_dollars").lt_eq(lit(20.00)))
            .then(lit("Low"))
            .when(
                col("wages_dollars")
                    .gt(lit(20.00))
                    .and(col("wages_dollars").lt_eq(lit(50.00))),
            )
            .then(lit("Medium"))
            .otherwise(lit("High"))
            .alias("wage_cat"),
    );

    println!("{}", lf.clone().limit(5).collect().unwrap());

    // Change numeric province code to alpha-code
    let lf = lf.with_column(col("prov").replace_strict(
        lit(Series::from_iter(vec![
            "10", "11", "12", "13", "24", "35", "46", "47", "48", "59",
        ])),
        lit(Series::from_iter(vec![
            "NL", "PE", "NS", "NB", "QC", "ON", "MB", "SK", "AB", "BC",
        ])),
        None,
        Some(DataType::String),
    ));

    println!("{}", lf.clone().limit(5).collect().unwrap());
}
