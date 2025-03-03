# Variables

This section will explore how to create new variables, derived from other variables and literals.

You can create new variables right inside of a `select()`, the same way you would select existing variables, with the same syntax. You can run this code with `cargo run --example 3_3_1_variables`.

First, lets connect to the partitioned parquet file:

```Rust
// Connect to LazyFrame (no data is brought into memory)
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();
```
Using `col()` for existing variables and `lit()` for literal values, you can create new values by adding new lines in the `select()`, and giving them a name with `alias()`. As you can see in this example, you can create a new column from a literal with `lit(5).alias("five")` or a new variable from a formula of a mix of literals (e.g. `(lit(5) + lit(7) - lit(2)).alias("ten")`). You can use any of the arithmetic expressions (`-`, `+`, `*`, `/`, `%`).


```Rust
// Add new variables from literals and columns
let lf = lf.select([
    col("^surv.*$"),                         // keep survyear, survmnth
    col("hrlyearn").alias("hourly_wages"),   // keep hrlyearn as hourly_wages
    lit(5).alias("five"),                    // add single value literal
    (lit(5) + lit(7) - lit(2)).alias("ten"), // add single value from two or more literals
]);

println!("{}", lf.clone().limit(5).collect().unwrap());
```

```
shape: (5, 5)
┌──────────┬──────────┬──────────────┬──────┬─────┐
│ survyear ┆ survmnth ┆ hourly_wages ┆ five ┆ ten │
│ ---      ┆ ---      ┆ ---          ┆ ---  ┆ --- │
│ i64      ┆ i64      ┆ i64          ┆ i32  ┆ i32 │
╞══════════╪══════════╪══════════════╪══════╪═════╡
│ 2011     ┆ 1        ┆ null         ┆ 5    ┆ 10  │
│ 2011     ┆ 1        ┆ null         ┆ 5    ┆ 10  │
│ 2011     ┆ 1        ┆ null         ┆ 5    ┆ 10  │
│ 2011     ┆ 1        ┆ null         ┆ 5    ┆ 10  │
│ 2011     ┆ 1        ┆ 2462         ┆ 5    ┆ 10  │
└──────────┴──────────┴──────────────┴──────┴─────┘
```

You can not use a newly created variable (or a renamed variable) in the same select. For example, you would not be able to run `(col("five") + col("ten")).alias("fifteen")` in the same `select()` as you created the `five` and `ten` columns. Instead, you can add more `select()` (with the `all()` keyword to keep previous variables), or simpler, you can use `with_column()` or `with_columns()` in the same way as you would a `select()` to add new columns and keep the existing columns. You can chain it or create a new step. Creating a column with an already existing name in a `with_column()` will replace the existing column:

```Rust
let lf = lf.with_column(
    (col("five") + col("ten")).alias("fifteen"), // add two columns
);

println!("{}", lf.clone().limit(5).collect().unwrap());
```

```
shape: (5, 6)
┌──────────┬──────────┬──────────────┬──────┬─────┬─────────┐
│ survyear ┆ survmnth ┆ hourly_wages ┆ five ┆ ten ┆ fifteen │
│ ---      ┆ ---      ┆ ---          ┆ ---  ┆ --- ┆ ---     │
│ i64      ┆ i64      ┆ i64          ┆ i32  ┆ i32 ┆ i32     │
╞══════════╪══════════╪══════════════╪══════╪═════╪═════════╡
│ 2011     ┆ 1        ┆ null         ┆ 5    ┆ 10  ┆ 15      │
│ 2011     ┆ 1        ┆ null         ┆ 5    ┆ 10  ┆ 15      │
│ 2011     ┆ 1        ┆ null         ┆ 5    ┆ 10  ┆ 15      │
│ 2011     ┆ 1        ┆ null         ┆ 5    ┆ 10  ┆ 15      │
│ 2011     ┆ 1        ┆ 2462         ┆ 5    ┆ 10  ┆ 15      │
└──────────┴──────────┴──────────────┴──────┴─────┴─────────┘
```

As we can see, `hourly_wages` is an `i64` in cents. We might want to convert it to dollars and cents (e.g. from `2462` to `24.62`). To do this, we have to convert the column to a `f64` using `.cast(DataType::Float32)`:

```Rust
// Cast the value from an `i64` to a `f64` and modify it (divide by 100)
let lf = lf
    .drop([col("five"), col("ten"), col("fifteen")]) // Remove unneeded variables (could also exclude them from the select)
    .filter(col("hourly_wages").is_not_null()) // Filter those with null wages
    .with_column(
        (col("hourly_wages").cast(DataType::Float64) / lit(100)).alias("wages_dollars"),
    );

println!("{}", lf.clone().limit(5).collect().unwrap());
```

```
shape: (5, 4)
┌──────────┬──────────┬──────────────┬───────────────┐
│ survyear ┆ survmnth ┆ hourly_wages ┆ wages_dollars │
│ ---      ┆ ---      ┆ ---          ┆ ---           │
│ i64      ┆ i64      ┆ i64          ┆ f64           │
╞══════════╪══════════╪══════════════╪═══════════════╡
│ 2011     ┆ 1        ┆ 2462         ┆ 24.62         │
│ 2011     ┆ 1        ┆ 3769         ┆ 37.69         │
│ 2011     ┆ 1        ┆ 2706         ┆ 27.06         │
│ 2011     ┆ 1        ┆ 2644         ┆ 26.44         │
│ 2011     ┆ 1        ┆ 1724         ┆ 17.24         │
└──────────┴──────────┴──────────────┴───────────────┘
```

You can also create conditional values for a column, using the `when()`, `then()` and `otherwise()` chain. This example below creates the `wage_cat` column if `<= 20` = `Low`, `> 20 & <= 50` = `Medium` and `> 50` = `High`.

```Rust
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
```

```
shape: (5, 5)
┌──────────┬──────────┬──────────────┬───────────────┬──────────┐
│ survyear ┆ survmnth ┆ hourly_wages ┆ wages_dollars ┆ wage_cat │
│ ---      ┆ ---      ┆ ---          ┆ ---           ┆ ---      │
│ i64      ┆ i64      ┆ i64          ┆ f64           ┆ str      │
╞══════════╪══════════╪══════════════╪═══════════════╪══════════╡
│ 2011     ┆ 1        ┆ 2462         ┆ 24.62         ┆ Medium   │
│ 2011     ┆ 1        ┆ 3769         ┆ 37.69         ┆ Medium   │
│ 2011     ┆ 1        ┆ 2706         ┆ 27.06         ┆ Medium   │
│ 2011     ┆ 1        ┆ 2644         ┆ 26.44         ┆ Medium   │
│ 2011     ┆ 1        ┆ 1724         ┆ 17.24         ┆ Low      │
└──────────┴──────────┴──────────────┴───────────────┴──────────┘
```
