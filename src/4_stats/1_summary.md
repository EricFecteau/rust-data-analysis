# Summary statistics

Summary statistics are the basic tools of data analysts. This chapter will demonstrate how to do all the simple summary statistics using Polars. Run this code using `cargo run -r --example 4_1_1_summary`.

> [!IMPORTANT]
> **Reminder**: The "income" variable is synthetic (random value between £10,000 and £100,000). This means that the distribution will not provide real-world summary results.

## Setup

First, lets connect to the partitioned parquet files and replace the region code with their name:

```rust
=== Rust 4_1_1_summary imports
=== Rust 4_1_1_summary block_1
```

## Simple statistics

You can do simple statistics functions like `mean()`, `sum()` or `median()` by limiting the `LazyFrame` to one variable and calling the appropreate function. Here is how you would get a median of `income`:

```rust
=== Rust 4_1_1_summary block_2
```

```
shape: (1, 1)
┌─────────┐
│ income  │
│ ---     │
│ f64     │
╞═════════╡
│ 55008.5 │
└─────────┘
```

You can get different statistics with other functions like `min()`, `quantile()` or `max()`. For the quantiles, you have to provide a `QuantileMethod`.

```rust
=== Rust 4_1_1_summary block_3
```

```
shape: (9, 2)
┌──────────────────┬──────────┐
│ statistic        ┆ value    │
│ ---              ┆ ---      │
│ str              ┆ f64      │
╞══════════════════╪══════════╡
│ count (x100,000) ┆ 294.0    │
│ mean             ┆ 55023.78 │
│ min              ┆ 10000.0  │
│ p01              ┆ 10917.0  │
│ p25              ┆ 32573.0  │
│ median           ┆ 55008.5  │
│ p75              ┆ 77492.0  │
│ p99              ┆ 99097.0  │
│ max              ┆ 99999.0  │
└──────────────────┴──────────┘
```


## Statistics by category

To get statistics by a variable group, you have to use `group_by()` with `agg()`. This example provides the mean income by region:

```rust
=== Rust 4_1_1_summary block_4
```

```
shape: (10, 2)
┌──────────────────────────┬──────────┐
│ region                   ┆ income   │
│ ---                      ┆ ---      │
│ str                      ┆ f64      │
╞══════════════════════════╪══════════╡
│ South East               ┆ 55102.39 │
│ East of England          ┆ 54976.91 │
│ North West               ┆ 55063.74 │
│ Wales                    ┆ 55075.05 │
│ Yorkshire and The Humber ┆ 54972.74 │
│ South West               ┆ 55108.22 │
│ East Midlands            ┆ 54697.23 │
│ London                   ┆ 54858.61 │
│ North East               ┆ 55177.93 │
│ West Midlands            ┆ 55320.6  │
└──────────────────────────┴──────────┘
```

## Weighted statistics

If you have survey weights, like the synthetic `weight` variable found on the 1% samples of the Census, you can create statistics by that weight, with formulas. In this example, a custom function called `weighted_quantile` provides quantiles that respect the survey weights. It provides an approximation of the 100%, using only the 1% sample.

```rust
=== Rust 4_1_1_summary block_5
```

```
shape: (9, 2)
┌──────────────────┬──────────┐
│ statistic        ┆ value    │
│ ---              ┆ ---      │
│ str              ┆ f64      │
╞══════════════════╪══════════╡
│ count (x100,000) ┆ 293.0    │
│ mean             ┆ 55015.96 │
│ min              ┆ 10000.0  │
│ p01              ┆ 10917.0  │
│ p25              ┆ 32565.0  │
│ median           ┆ 55000.0  │
│ p75              ┆ 77471.0  │
│ p99              ┆ 99093.0  │
│ max              ┆ 99999.0  │
└──────────────────┴──────────┘
```

It also works with the `group_by()` and `agg()` function. In this example, weighted statistics grouped by sex are computed and then pivoted for nicer printing:

```rust
=== Rust 4_1_1_summary block_6
```

```
shape: (9, 3)
┌──────────────────┬──────────┬──────────┐
│ variable         ┆ Female   ┆ Male     │
│ ---              ┆ ---      ┆ ---      │
│ str              ┆ f64      ┆ f64      │
╞══════════════════╪══════════╪══════════╡
│ count (x100,000) ┆ 139.9    ┆ 153.1    │
│ mean             ┆ 54948.81 ┆ 55077.33 │
│ min              ┆ 10000.0  ┆ 10000.0  │
│ p01              ┆ 10908.0  ┆ 10921.0  │
│ p25              ┆ 32483.0  ┆ 32635.0  │
│ median           ┆ 55007.0  ┆ 54993.0  │
│ p75              ┆ 77351.0  ┆ 77569.0  │
│ p99              ┆ 99049.0  ┆ 99126.0  │
│ max              ┆ 99999.0  ┆ 99999.0  │
└──────────────────┴──────────┴──────────┘
```