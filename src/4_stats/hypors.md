# Hypothesis testing

You can perform various hypothesis tests in Rust using [hypors](https://docs.rs/hypors/latest/hypors/). This crate allows you to do t-tests, z-tests, proportion tests, ANOVA, Chi-square tests, and Mann-Whitney tests, using `polars` for data manipulations and `statrs` for statistical distributions. This section will give examples for a `Chi-square` test, an `ANOVA` and a `Mann-Whitney` test, focusing on modifying the `Polars` data used throughout the book to match the input necessary by the crate.

> [!IMPORTANT]
> `Hypors 0.2` uses `Polars 0.43`, while the rest of this book uses `Polars 0.48`. You will have to use [df-interchange](https://github.com/EricFecteau/df-interchange) as explained in the [concepts](../1_start/concepts.md) section of the book.

## Chi-square test

With `Hypors`, you can perform a [Chi-Square Test for Independence](https://docs.rs/hypors/latest/hypors/chi_square/categorical/fn.independence.html). You can run the following code with `cargo run --example 4_2_1_chi_square`.

We first must create some summary statistics for the contingency table needed for the Chi-Square test. We will create a count of individuals (unweighted) with paid overtime, by gender and marital status.

```Rust
// Connect to the parquet LFS data 
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

// Count individuals with paid overtime by gender and marital status
let df = lf
    .clone()
    .filter(col("paidot").is_null().not())
    .group_by([col("gender"), col("marstat")])
    .agg([col("paidot")
        .gt(0)
        .cast(DataType::Int8)
        .sum()
        .alias("ot_flag")])
    .sort(["gender", "marstat"], Default::default())
    .with_column(col("gender").replace_strict(
        lit(Series::from_iter(vec!["1", "2"])),
        lit(Series::from_iter(vec!["Men+", "Women+"])),
        None,
        Some(DataType::String),
    ))
    .with_column(col("marstat").replace_strict(
        lit(Series::from_iter(vec!["1", "2", "3", "4", "5", "6"])),
        lit(Series::from_iter(vec![
            "Married",
            "Common-law",
            "Widowed",
            "Separated",
            "Divorced",
            "Single",
        ])),
        None,
        Some(DataType::String),
    ))
    .collect()
    .unwrap();

println!("{}", &df);
```

We then have this table:

```
shape: (12, 3)
┌────────┬────────────┬─────────┐
│ gender ┆ marstat    ┆ ot_flag │
│ ---    ┆ ---        ┆ ---     │
│ str    ┆ str        ┆ i64     │
╞════════╪════════════╪═════════╡
│ Men+   ┆ Married    ┆ 270505  │
│ Men+   ┆ Common-law ┆ 103560  │
│ Men+   ┆ Widowed    ┆ 2640    │
│ Men+   ┆ Separated  ┆ 14970   │
│ Men+   ┆ Divorced   ┆ 20856   │
│ …      ┆ …          ┆ …       │
│ Women+ ┆ Common-law ┆ 47962   │
│ Women+ ┆ Widowed    ┆ 4480    │
│ Women+ ┆ Separated  ┆ 10586   │
│ Women+ ┆ Divorced   ┆ 18101   │
│ Women+ ┆ Single     ┆ 71710   │
└────────┴────────────┴─────────┘
```

Now that we have this contingency table, we can convert it to a `Vec<Vec<f64>>`, as required by the `independence()` function of `Hypors`. To do this, we can pivot the table:

```Rust
// Transpose
let df = pivot::pivot_stable(
    &df,
    ["gender"],
    Some(["marstat"]),
    Some(["ot_flag"]),
    false,
    None,
    None,
)
.unwrap()
.drop("marstat")
.unwrap();

println!("{}", &df);
```

```
shape: (6, 2)
┌────────┬────────┐
│ Men+   ┆ Women+ │
│ ---    ┆ ---    │
│ i64    ┆ i64    │
╞════════╪════════╡
│ 270505 ┆ 111365 │
│ 103560 ┆ 47962  │
│ 2640   ┆ 4480   │
│ 14970  ┆ 10586  │
│ 20856  ┆ 18101  │
│ 153548 ┆ 71710  │
└────────┴────────┘
```

And then convert the `Polars` data into the `Vec<Vec<f64>>`:

```Rust
// Create an array of arrays of f64
let cols = df
    .get_columns()
    .iter()
    .map(|c| {
        c.as_materialized_series()
            .to_float()
            .unwrap()
            .f64()
            .unwrap()
            .to_vec_null_aware()
            .left()
            .unwrap()
    })
    .collect::<Vec<Vec<f64>>>();
```

Now that the data is ready, we can provide it to `Hypors` and get the results.

```Rust
// Perform Chi-Square Test for Independence
let alpha = 0.05;
let result = independence(&cols, alpha).unwrap();

println!(
    "Result: {}\nP-value: {}\nNull hypothesis: {}\nReject null: {}",
    result.test_statistic, result.p_value, result.null_hypothesis, result.reject_null
);
```

```
Result: 9355.961452109936
P value: 0
Null hypothesis: H0: Variables are independent
Reject null: true
```

## ANOVA 

With `Hypors`, you can perform a [one-way Analysis of Variance (ANOVA)](https://docs.rs/hypors/latest/hypors/anova/index.html). You can run the following code with `cargo run --example 4_2_2_anova`.

We first must subset our data for the analysis. The data we will use for the ANOVA will be the (unweighted) hourly earnings by immigration status (from January 2020).

```Rust
// Connect to LazyFrame (no data is brought into memory)
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

// Hourly earnings by immigration category (Jan 2020)
let df = lf
    .clone()
    .filter(col("survyear").eq(lit(2020)))
    .filter(col("survmnth").eq(lit(1)))
    .filter(col("hrlyearn").is_null().not())
    .select([
        (col("hrlyearn").cast(DataType::Float64) / lit(100)).alias("hrlyearn"),
        col("immig"),
    ])
    .sort(["immig"], Default::default())
    .with_column(col("immig").replace_strict(
        lit(Series::from_iter(vec!["1", "2", "3"])),
        lit(Series::from_iter(vec![
            "Immigrant (<= 10 years)",
            "Immigrant (> 10 years)",
            "Non-immigrant",
        ])),
        None,
        Some(DataType::String),
    ))
    .with_row_index("index", None)
    .collect()
    .unwrap();

println!("{}", &df);
```

```
shape: (49_391, 3)
┌───────┬──────────┬─────────────────────────┐
│ index ┆ hrlyearn ┆ immig                   │
│ ---   ┆ ---      ┆ ---                     │
│ u32   ┆ f64      ┆ str                     │
╞═══════╪══════════╪═════════════════════════╡
│ 0     ┆ 30.0     ┆ Immigrant (<= 10 years) │
│ 1     ┆ 13.85    ┆ Immigrant (<= 10 years) │
│ 2     ┆ 12.5     ┆ Immigrant (<= 10 years) │
│ 3     ┆ 91.35    ┆ Immigrant (<= 10 years) │
│ 4     ┆ 17.0     ┆ Immigrant (<= 10 years) │
│ …     ┆ …        ┆ …                       │
│ 49386 ┆ 18.0     ┆ Non-immigrant           │
│ 49387 ┆ 34.62    ┆ Non-immigrant           │
│ 49388 ┆ 22.0     ┆ Non-immigrant           │
│ 49389 ┆ 32.34    ┆ Non-immigrant           │
│ 49390 ┆ 21.74    ┆ Non-immigrant           │
└───────┴──────────┴─────────────────────────┘
```

To get Polars `Series`, as needed by `Hypors` as the input, we will have to first pivot the data:

```Rust
// Transpose
let df = pivot::pivot_stable(
    &df,
    ["immig"],
    Some(["index"]),
    Some(["hrlyearn"]),
    false,
    None,
    None,
)
.unwrap()
.drop("index")
.unwrap();

println!("{}", &df);
```

```
shape: (49_391, 3)
┌─────────────────────────┬────────────────────────┬───────────────┐
│ Immigrant (<= 10 years) ┆ Immigrant (> 10 years) ┆ Non-immigrant │
│ ---                     ┆ ---                    ┆ ---           │
│ f64                     ┆ f64                    ┆ f64           │
╞═════════════════════════╪════════════════════════╪═══════════════╡
│ 30.0                    ┆ null                   ┆ null          │
│ 13.85                   ┆ null                   ┆ null          │
│ 12.5                    ┆ null                   ┆ null          │
│ 91.35                   ┆ null                   ┆ null          │
│ 17.0                    ┆ null                   ┆ null          │
│ …                       ┆ …                      ┆ …             │
│ null                    ┆ null                   ┆ 18.0          │
│ null                    ┆ null                   ┆ 34.62         │
│ null                    ┆ null                   ┆ 22.0          │
│ null                    ┆ null                   ┆ 32.34         │
│ null                    ┆ null                   ┆ 21.74         │
└─────────────────────────┴────────────────────────┴───────────────┘
```

Now that we have the data from each immigration category, we can convert the `DataFrame` into `Series` with `get_columns()`. But first, `Hypors` assumes `Polars 0.43` `Series`, not `Polars 0.48` `Series`. So we must first pass the `DataFrame` through [df-interchange](https://github.com/EricFecteau/df-interchange) to convert it.

```Rust
// Convert from Polars 0.48 to Polars 0.43
let df = Interchange::from_polars_0_48(df)
    .unwrap()
    .to_polars_0_43()
    .unwrap();

// Create Vec<Series> for ANOVA
let cols = df.get_columns();
```

We can now pass each `Series` to the `anova()` function from `Hypors`. Because of the way the data was pivoted, we must deal with null values with `drop_null()`.

```Rust
// Perform one-way ANOVA
let alpha = 0.05;
let result = anova(
    &[
        &cols[0].drop_nulls(),
        &cols[1].drop_nulls(),
        &cols[2].drop_nulls(),
    ],
    alpha,
)
.unwrap();

println!(
    "F-statistic: {}\nP-value: {}\nNull hypothesis: {}\nReject null: {}",
    result.test_statistic, result.p_value, result.null_hypothesis, result.reject_null
);
```

```
F-statistic: 142.63289933265534
P-value: 0
Null hypothesis: H0: µ1 = µ2 = µ3
Reject null: true
```

## Mann-Whitney test

With `Hypors`, you can perform a [Mann-Whitney U Test](https://docs.rs/hypors/latest/hypors/mann_whitney/u/fn.u_test.html). You can run the following code with `cargo run --example 4_2_3_mwu`.

We first must subset our data for the analysis. The data we will use for the Mann-Whitney U test will be the (unweighted) hourly earnings by gender (from January 2020).

```Rust
// Connect to LazyFrame (no data is brought into memory)
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

// Hourly earnings by gender (Jan 2020)
let df = lf
    .clone()
    .filter(col("survyear").eq(lit(2020)))
    .filter(col("survmnth").eq(lit(1)))
    .filter(col("hrlyearn").is_null().not())
    .select([
        col("gender").replace_strict(
            lit(Series::from_iter(vec!["1", "2"])),
            lit(Series::from_iter(vec!["Men+", "Women+"])),
            None,
            Some(DataType::String),
        ),
        (col("hrlyearn").cast(DataType::Float64) / lit(100)),
    ])
    .with_row_index("index", None)
    .collect()
    .unwrap();

println!("{}", &df);
```

```
shape: (49_391, 3)
┌───────┬────────┬──────────┐
│ index ┆ gender ┆ hrlyearn │
│ ---   ┆ ---    ┆ ---      │
│ u32   ┆ str    ┆ f64      │
╞═══════╪════════╪══════════╡
│ 0     ┆ Women+ ┆ 14.0     │
│ 1     ┆ Men+   ┆ 22.0     │
│ 2     ┆ Men+   ┆ 31.25    │
│ 3     ┆ Women+ ┆ 13.25    │
│ 4     ┆ Women+ ┆ 12.98    │
│ …     ┆ …      ┆ …        │
│ 49386 ┆ Men+   ┆ 27.0     │
│ 49387 ┆ Women+ ┆ 34.62    │
│ 49388 ┆ Women+ ┆ 22.0     │
│ 49389 ┆ Men+   ┆ 32.34    │
│ 49390 ┆ Women+ ┆ 21.74    │
└───────┴────────┴──────────┘
```

To get Polars `Series`, as needed by `Hypors` as the input, we will have to first pivot the data:

```Rust
// Transpose
let df = pivot::pivot_stable(
    &df,
    ["gender"],
    Some(["index"]),
    Some(["hrlyearn"]),
    false,
    None,
    None,
)
.unwrap()
.drop("index")
.unwrap();

println!("{}", &df);
```

```
shape: (49_391, 2)
┌────────┬───────┐
│ Women+ ┆ Men+  │
│ ---    ┆ ---   │
│ f64    ┆ f64   │
╞════════╪═══════╡
│ 14.0   ┆ null  │
│ null   ┆ 22.0  │
│ null   ┆ 31.25 │
│ 13.25  ┆ null  │
│ 12.98  ┆ null  │
│ …      ┆ …     │
│ null   ┆ 27.0  │
│ 34.62  ┆ null  │
│ 22.0   ┆ null  │
│ null   ┆ 32.34 │
│ 21.74  ┆ null  │
└────────┴───────┘
```

Now that we have the data from each gender, we can convert the `DataFrame` into `Series` with `get_columns()`. But first, `Hypors` assumes `Polars 0.43` `Series`, not `Polars 0.48` `Series`. So we must first pass the `DataFrame` through [df-interchange](https://github.com/EricFecteau/df-interchange) to convert it.

```Rust
// Convert from Polars 0.48 to Polars 0.43
let df = Interchange::from_polars_0_48(df)
    .unwrap()
    .to_polars_0_43()
    .unwrap();

// Create Vec<Series> for MWU
let cols = df.get_columns();
```

We can now pass each `Series` to the `u_test()` function from `Hypors`. Because of the way the data was pivoted, we must deal with null values with `drop_null()`.

```Rust
// Perform the Mann-Whiteny U test
let alpha = 0.05;
let result = u_test(
    &cols[0].drop_nulls(),
    &cols[1].drop_nulls(),
    alpha,
    TailType::Two,
)
.unwrap();

println!(
    "F-statistic: {}\nP-value: {}\nNull hypothesis: {}\nReject null: {}",
    result.test_statistic, result.p_value, result.null_hypothesis, result.reject_null
);
```

```
U-statistic: 252970481.5
P-value: 0
Null hypothesis: H0: The distributions of both groups are equal.
Reject null: true
```