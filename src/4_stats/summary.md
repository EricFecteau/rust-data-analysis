# Summary statistics



## Setup

First, lets connect to the partitioned parquet files and modify the hourly earnings variable by dividing it by 100 (converting it from cents (2375) to dollars and cents (23.75)) and providing a 2-letter label for the "province" variable and a label for the gender:

```rust
=== Rust 4_1_1_summary imports
=== Rust 4_1_1_summary block_1
```

## Simple statistics

You can do simple statistics functions like `mean()`, `sum()` or `median()` by limiting the `LazyFrame` to one variable and calling the appropreate function. Here is how you would get a median of `hourly_wages`:

```rust
=== Rust 4_1_1_summary block_2
```

```
shape: (1, 1)
┌──────────────┐
│ hourly_wages │
│ ---          │
│ f64          │
╞══════════════╡
│ 23.75        │
└──────────────┘
```

You can get =different statistics with other functions like `min()`, `quantile()` or `max()`. For the quantiles, you have to provide a `QuantileMethod`.

```rust
=== Rust 4_1_1_summary block_3
```

```
shape: (9, 2)
┌───────────────┬───────────┐
│ variable      ┆ value     │
│ ---           ┆ ---       │
│ str           ┆ f64       │
╞═══════════════╪═══════════╡
│ count (x1000) ┆ 8662.0    │
│ mean          ┆ 27.568068 │
│ min           ┆ 3.0       │
│ p01           ┆ 9.85      │
│ p25           ┆ 16.5      │
│ median        ┆ 23.75     │
│ p75           ┆ 35.0      │
│ p99           ┆ 81.73     │
│ max           ┆ 230.77    │
└───────────────┴───────────┘
```


## Statistics by category

Once you want to get statistics by another variable, you have to use `group_by()` with `agg()`. This example provides the mean hourly wages by province for the whole period:

```rust
=== Rust 4_1_1_summary block_4
```

```
shape: (10, 2)
┌──────┬──────────────┐
│ prov ┆ hourly_wages │
│ ---  ┆ ---          │
│ str  ┆ f64          │
╞══════╪══════════════╡
│ ON   ┆ 28.81        │
│ NL   ┆ 27.27        │
│ QC   ┆ 26.68        │
│ AB   ┆ 31.07        │
│ NS   ┆ 24.49        │
│ MB   ┆ 24.94        │
│ SK   ┆ 27.87        │
│ NB   ┆ 23.9         │
│ PE   ┆ 23.37        │
│ BC   ┆ 28.4         │
└──────┴──────────────┘
```

## Weighted statistics

If you have survey weights, like the `finalwt` variable on the Labour Force Survey (LFS), you can create statistics by that weight, with formulas. In this example, a custom function called `weighted_quantile` is create the provide quantiles that respect the survey weights:

```rust
=== Rust 4_1_1_summary block_5
```

```
shape: (9, 2)
┌──────────────────┬────────┐
│ variable         ┆ value  │
│ ---              ┆ ---    │
│ str              ┆ f64    │
╞══════════════════╪════════╡
│ count (x100,000) ┆ 2666.0 │
│ mean             ┆ 28.37  │
│ min              ┆ 3.0    │
│ p01              ┆ 10.0   │
│ p25              ┆ 17.0   │
│ median           ┆ 24.04  │
│ p75              ┆ 35.86  │
│ p99              ┆ 86.54  │
│ max              ┆ 230.77 │
└──────────────────┴────────┘
```

It also works with the `group_by()` and `agg()` function. In this example, weighted statistics grouped by gender are computed and then pivoted for nicer printing:

```rust
=== Rust 4_1_1_summary block_6
```

```
shape: (9, 3)
┌──────────────────┬────────┬────────┐
│ variable         ┆ Men+   ┆ Women+ │
│ ---              ┆ ---    ┆ ---    │
│ str              ┆ f64    ┆ f64    │
╞══════════════════╪════════╪════════╡
│ count (x100,000) ┆ 1354.0 ┆ 1311.0 │
│ mean             ┆ 30.42  ┆ 26.25  │
│ min              ┆ 3.0    ┆ 3.0    │
│ p01              ┆ 10.0   ┆ 9.75   │
│ p25              ┆ 18.0   ┆ 16.0   │
│ median           ┆ 26.0   ┆ 22.12  │
│ p75              ┆ 38.46  ┆ 32.97  │
│ p99              ┆ 93.54  ┆ 75.91  │
│ max              ┆ 226.44 ┆ 230.77 │
└──────────────────┴────────┴────────┘
```