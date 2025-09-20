# Hypothesis testing

You can perform various hypothesis tests in Rust using [hypors](https://docs.rs/hypors/latest/hypors/). This crate allows you to do t-tests, z-tests, proportion tests, ANOVA, Chi-square tests, and Mann-Whitney tests, using `polars` for data manipulations and `statrs` for statistical distributions. This section will give examples for a `Chi-square` test, an `ANOVA` and a `Mann-Whitney` test, focusing on modifying the `Polars` data used throughout the book to match the input necessary by the crate.

## Chi-square test

With `Hypors`, you can perform a [Chi-Square Test for Independence](https://docs.rs/hypors/latest/hypors/chi_square/categorical/fn.independence.html). You can run the following code with `cargo run --example 4_2_1_chi_square`.

We must first create some summary statistics for the contingency table needed for the Chi-Square test. We will create a count of individuals (unweighted) with paid overtime, by gender and marital status.

```rust
=== Rust 4_2_1_chi_square imports
=== Rust 4_2_1_chi_square block_1
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

```rust
=== Rust 4_2_1_chi_square block_2
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

And then convert the `Polars` data into the `Vec<Vec<f64>>` by materializing the series as `f64`:

```rust
=== Rust 4_2_1_chi_square block_3
```

Now that the data is ready, we can provide it to `Hypors` and get the results.

```rust
=== Rust 4_2_1_chi_square block_4
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

```rust
=== Rust 4_2_2_anova imports
=== Rust 4_2_2_anova block_1
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

To provide the data as `Vec<Vec<f64>>`, as required by the `anova()` function of `Hypors` the data will first have to be pivoted:

```rust
=== Rust 4_2_2_anova block_2
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
And then convert the `Polars` data into the `Vec<Vec<f64>>` by materializing the series as `f64`:

```rust
=== Rust 4_2_2_anova block_3
```

We can now pass these columns to `Hypors`.

```rust
=== Rust 4_2_2_anova block_4
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

```rust
=== Rust 4_2_3_mwu imports
=== Rust 4_2_3_mwu block_1
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

To get the required `Vec<Vec<f64>>`, the data will first be pivoted:

```rust
=== Rust 4_2_3_mwu block_2
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

And then convert the `Polars` data into the `Vec<Vec<f64>>` by materializing the series as `f64`:

```rust
=== Rust 4_2_3_mwu block_3
```

We can now pass each column to the `u_test()` function from `Hypors`. 

```rust
=== Rust 4_2_3_mwu block_4
```

```
U-statistic: 252970481.5
P-value: 0
Null hypothesis: H0: The distributions of both groups are equal.
Reject null: true
```