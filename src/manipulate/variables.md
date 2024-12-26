# New variables

This section will explore how to create new variables, derived from other variables and literals.

First, lets connect to the partitioned parquet file:

```Rust
// Connect to LazyFrame (no data is brought into memory)
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();
```

You can create new variables right inside a `select()`, the same way you would select existing variables, with the same syntax. Using `col()` for existing variables and `lit()` values, you can create new values by adding new lines in the select, and giving them a name with `alias()`. As you can see in this example, you can create a new column from a literal with `lit(5).alias("five")` or a new variable from a formula of a mix of literals and columns (e.g. `(lit(5) + lit(7) - lit(2)).alias("ten")`). You can use any of the arithmetic expressions (`-`, `+`, `*`, `/`, `%`).


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
│ 2006     ┆ 1        ┆ null         ┆ 5    ┆ 10  │
│ 2006     ┆ 1        ┆ null         ┆ 5    ┆ 10  │
│ 2006     ┆ 1        ┆ null         ┆ 5    ┆ 10  │
│ 2006     ┆ 1        ┆ null         ┆ 5    ┆ 10  │
│ 2006     ┆ 1        ┆ 1300         ┆ 5    ┆ 10  │
└──────────┴──────────┴──────────────┴──────┴─────┘
```

You can not use a newly created variable (or a renamed variable) in the same select. For example, you would not be able to run `(col("five") + col("ten")).alias("fifteen")` in the same `select()` as you created the `five` and `ten` columns. Instead, you can chain multiple `filter()`, or add a new step like below:

```Rust
let lf = lf.select([
    all(), // keep all previously kept variables (same as col("*"))
    (col("five") + col("ten")).alias("fifteen"), // add two columns
]);

println!("{}", lf.clone().limit(5).collect().unwrap());
```

```
shape: (5, 6)
┌──────────┬──────────┬──────────────┬──────┬─────┬─────────┐
│ survyear ┆ survmnth ┆ hourly_wages ┆ five ┆ ten ┆ fifteen │
│ ---      ┆ ---      ┆ ---          ┆ ---  ┆ --- ┆ ---     │
│ i64      ┆ i64      ┆ i64          ┆ i32  ┆ i32 ┆ i32     │
╞══════════╪══════════╪══════════════╪══════╪═════╪═════════╡
│ 2006     ┆ 1        ┆ null         ┆ 5    ┆ 10  ┆ 15      │
│ 2006     ┆ 1        ┆ null         ┆ 5    ┆ 10  ┆ 15      │
│ 2006     ┆ 1        ┆ null         ┆ 5    ┆ 10  ┆ 15      │
│ 2006     ┆ 1        ┆ null         ┆ 5    ┆ 10  ┆ 15      │
│ 2006     ┆ 1        ┆ 1300         ┆ 5    ┆ 10  ┆ 15      │
└──────────┴──────────┴──────────────┴──────┴─────┴─────────┘
```

As we can see, `hourly_wages` is an `i64` in cents. We might want to convert it to dollars and cents (e.g. from `1300` to `13.00`). To do this, we have to convert the column to a `f64` using `.cast(DataType::Float32)`:

```Rust
// Cast the value from an `i64` to a `f64` and modify it (divide by 100)
let lf = lf
    .drop([col("five"), col("ten"), col("fifteen")]) // Remove unneeded variables (could also exclude them from the select)
    .filter(col("hourly_wages").is_not_null()) // Filter those with null wages
    .select([
        all(), // keep all previously kept variables (same as col("*"))
        (col("hourly_wages").cast(DataType::Float64) / lit(100)).alias("wages_dollars"),
    ]);

println!("{}", lf.clone().limit(5).collect().unwrap());
```

```
shape: (5, 4)
┌──────────┬──────────┬──────────────┬───────────────┐
│ survyear ┆ survmnth ┆ hourly_wages ┆ wages_dollars │
│ ---      ┆ ---      ┆ ---          ┆ ---           │
│ i64      ┆ i64      ┆ i64          ┆ f64           │
╞══════════╪══════════╪══════════════╪═══════════════╡
│ 2006     ┆ 1        ┆ 1300         ┆ 13.0          │
│ 2006     ┆ 1        ┆ 3409         ┆ 34.09         │
│ 2006     ┆ 1        ┆ 3000         ┆ 30.0          │
│ 2006     ┆ 1        ┆ 813          ┆ 8.13          │
│ 2006     ┆ 1        ┆ 1827         ┆ 18.27         │
└──────────┴──────────┴──────────────┴───────────────┘
```

You can also create conditional values for a column, using the `when()`, `then()` and `otherwise()` chain. This example below creates the `wage_cat` column if `<= 10` = `Low`, `> 10 & <= 30` = `Medium` and `> 30` = `High`.

```Rust
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
```

```
shape: (5, 5)
┌──────────┬──────────┬──────────────┬───────────────┬──────────┐
│ survyear ┆ survmnth ┆ hourly_wages ┆ wages_dollars ┆ wage_cat │
│ ---      ┆ ---      ┆ ---          ┆ ---           ┆ ---      │
│ i64      ┆ i64      ┆ i64          ┆ f64           ┆ str      │
╞══════════╪══════════╪══════════════╪═══════════════╪══════════╡
│ 2006     ┆ 1        ┆ 1300         ┆ 13.0          ┆ Medium   │
│ 2006     ┆ 1        ┆ 3409         ┆ 34.09         ┆ High     │
│ 2006     ┆ 1        ┆ 3000         ┆ 30.0          ┆ Medium   │
│ 2006     ┆ 1        ┆ 813          ┆ 8.13          ┆ Low      │
│ 2006     ┆ 1        ┆ 1827         ┆ 18.27         ┆ Medium   │
└──────────┴──────────┴──────────────┴───────────────┴──────────┘
```
