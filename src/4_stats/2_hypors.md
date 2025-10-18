# Hypothesis testing

You can perform various hypothesis tests in Rust using [hypors](https://docs.rs/hypors/latest/hypors/). This crate allows you to do t-tests, z-tests, proportion tests, ANOVA, Chi-square tests, and Mann-Whitney tests, using `polars` for data manipulations and `statrs` for statistical distributions. This section will give examples for a `Chi-square` test, an `ANOVA` and a `Mann-Whitney` test, focusing on modifying the `Polars` data used throughout the book to match the input necessary by the crate.

## Chi-square test

With `Hypors`, you can perform a [Chi-Square Test for Independence](https://docs.rs/hypors/latest/hypors/chi_square/categorical/fn.independence.html). You can run the following code with `cargo run --example 4_2_1_chi_square`.

We must first create some summary statistics for the contingency table needed for the Chi-Square test. We will create a count of individuals with fair or better health, by sex and marital status.

```rust
=== Rust 4_2_1_chi_square imports
=== Rust 4_2_1_chi_square block_1
```

We then have this table:

```
shape: (10, 3)
┌────────┬───────────────┬─────────────┐
│ sex    ┆ mar_stat      ┆ health_flag │
│ ---    ┆ ---           ┆ ---         │
│ str    ┆ str           ┆ i64         │
╞════════╪═══════════════╪═════════════╡
│ Female ┆ Never married ┆ 8399500     │
│ Female ┆ Married       ┆ 10270600    │
│ Female ┆ Separated     ┆ 546500      │
│ Female ┆ Divorced      ┆ 2274400     │
│ Female ┆ Widowed       ┆ 1865500     │
│ Male   ┆ Never married ┆ 9267400     │
│ Male   ┆ Married       ┆ 10252700    │
│ Male   ┆ Separated     ┆ 412200      │
│ Male   ┆ Divorced      ┆ 1628800     │
│ Male   ┆ Widowed       ┆ 605300      │
└────────┴───────────────┴─────────────┘
```

Now that we have this table, we can convert it to a `Vec<Vec<f64>>`, as required by the `independence()` function of `Hypors`. To do this, we can pivot the table:

```rust
=== Rust 4_2_1_chi_square block_2
```

```
shape: (5, 2)
┌──────────┬──────────┐
│ Female   ┆ Male     │
│ ---      ┆ ---      │
│ i64      ┆ i64      │
╞══════════╪══════════╡
│ 8399500  ┆ 9267400  │
│ 10270600 ┆ 10252700 │
│ 546500   ┆ 412200   │
│ 2274400  ┆ 1628800  │
│ 1865500  ┆ 605300   │
└──────────┴──────────┘
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
Result: 780419.0168616888
P-value: 0
Null hypothesis: H0: Variables are independent
Reject null: true
```

## ANOVA 

With `Hypors`, you can perform a [one-way Analysis of Variance (ANOVA)](https://docs.rs/hypors/latest/hypors/anova/index.html). You can run the following code with `cargo run --example 4_2_2_anova`.

We first must subset our data for the analysis. The data we will use for the ANOVA will be the income by economically active type, for those living in London.

```rust
=== Rust 4_2_2_anova imports
=== Rust 4_2_2_anova block_1
```

```
shape: (4_715_100, 3)
┌─────────┬────────┬───────────────────┐
│ index   ┆ income ┆ econ              │
│ ---     ┆ ---    ┆ ---               │
│ u32     ┆ i64    ┆ str               │
╞═════════╪════════╪═══════════════════╡
│ 0       ┆ 68493  ┆ Employee          │
│ 1       ┆ 59601  ┆ Employee          │
│ 2       ┆ 70050  ┆ Employee          │
│ 3       ┆ 55982  ┆ Employee          │
│ 4       ┆ 47560  ┆ Employee          │
│ …       ┆ …      ┆ …                 │
│ 4715095 ┆ 24742  ┆ Full-time student │
│ 4715096 ┆ 24742  ┆ Full-time student │
│ 4715097 ┆ 24742  ┆ Full-time student │
│ 4715098 ┆ 24742  ┆ Full-time student │
│ 4715099 ┆ 24742  ┆ Full-time student │
└─────────┴────────┴───────────────────┘
```

To provide the data as `Vec<Vec<f64>>`, as required by the `anova()` function of `Hypors` the data will first have to be pivoted:

```rust
=== Rust 4_2_2_anova block_2
```

```
shape: (4_715_100, 4)
┌──────────┬───────────────┬────────────┬───────────────────┐
│ Employee ┆ Self-employed ┆ Unemployed ┆ Full-time student │
│ ---      ┆ ---           ┆ ---        ┆ ---               │
│ i64      ┆ i64           ┆ i64        ┆ i64               │
╞══════════╪═══════════════╪════════════╪═══════════════════╡
│ 68493    ┆ null          ┆ null       ┆ null              │
│ 59601    ┆ null          ┆ null       ┆ null              │
│ 70050    ┆ null          ┆ null       ┆ null              │
│ 55982    ┆ null          ┆ null       ┆ null              │
│ 47560    ┆ null          ┆ null       ┆ null              │
│ …        ┆ …             ┆ …          ┆ …                 │
│ null     ┆ null          ┆ null       ┆ 24742             │
│ null     ┆ null          ┆ null       ┆ 24742             │
│ null     ┆ null          ┆ null       ┆ 24742             │
│ null     ┆ null          ┆ null       ┆ 24742             │
│ null     ┆ null          ┆ null       ┆ 24742             │
└──────────┴───────────────┴────────────┴───────────────────┘
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
F-statistic: 47.254477424875745
P-value: 0
Null hypothesis: H0: µ1 = µ2 = µ3 = µ4
Reject null: true
```

## Mann-Whitney test

With `Hypors`, you can perform a [Mann-Whitney U Test](https://docs.rs/hypors/latest/hypors/mann_whitney/u/fn.u_test.html). You can run the following code with `cargo run --example 4_2_3_mwu`.

We first must subset our data for the analysis. The data we will use for the Mann-Whitney U test will be the income by sex, for those living in London.

```rust
=== Rust 4_2_3_mwu imports
=== Rust 4_2_3_mwu block_1
```

```
shape: (4_715_100, 3)
┌─────────┬────────┬────────┐
│ index   ┆ sex    ┆ income │
│ ---     ┆ ---    ┆ ---    │
│ u32     ┆ str    ┆ i64    │
╞═════════╪════════╪════════╡
│ 0       ┆ Female ┆ 68493  │
│ 1       ┆ Female ┆ 65290  │
│ 2       ┆ Female ┆ 59601  │
│ 3       ┆ Female ┆ 70050  │
│ 4       ┆ Female ┆ 21406  │
│ …       ┆ …      ┆ …      │
│ 4715095 ┆ Male   ┆ 93411  │
│ 4715096 ┆ Male   ┆ 94912  │
│ 4715097 ┆ Male   ┆ 14029  │
│ 4715098 ┆ Male   ┆ 88617  │
│ 4715099 ┆ Male   ┆ 50330  │
└─────────┴────────┴────────┘
```

To get the required `Vec<Vec<f64>>`, the data will first be pivoted:

```rust
=== Rust 4_2_3_mwu block_2
```

```
shape: (4_715_100, 2)
┌────────┬───────┐
│ Female ┆ Male  │
│ ---    ┆ ---   │
│ i64    ┆ i64   │
╞════════╪═══════╡
│ 68493  ┆ null  │
│ 65290  ┆ null  │
│ 59601  ┆ null  │
│ 70050  ┆ null  │
│ 21406  ┆ null  │
│ …      ┆ …     │
│ null   ┆ 93411 │
│ null   ┆ 94912 │
│ null   ┆ 14029 │
│ null   ┆ 88617 │
│ null   ┆ 50330 │
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
U-statistic: 2765995405000
P-value: 0.00000000001017697037752896
Null hypothesis: H0: The distributions of both groups are equal.
Reject null: true
```