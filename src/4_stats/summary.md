# Summary statistics



## Setup

First, lets connect to the partitioned parquet files and modify the hourly earnings variable by dividing it by 100 (converting it from cents to dollars and cents) and providing a 2-letter label for the "province" variable and a label for the gender:

```Rust
// Connect to LazyFrame
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

// Modify var
let lf = lf
    .filter(col("hrlyearn").is_not_null())
    .with_column((col("hrlyearn").cast(DataType::Float64) / lit(100)).alias("hourly_wages"))
    .with_column(col("prov").replace_strict(
        lit(Series::from_iter(vec![
            "10", "11", "12", "13", "24", "35", "46", "47", "48", "59",
        ])),
        lit(Series::from_iter(vec![
            "NL", "PE", "NS", "NB", "QC", "ON", "MB", "SK", "AB", "BC",
        ])),
        None,
        Some(DataType::String),
    ))
    .with_column(col("gender").replace_strict(
        lit(Series::from_iter(vec!["1", "2"])),
        lit(Series::from_iter(vec!["Men+", "Women+"])),
        None,
        Some(DataType::String),
    ));
```

## Simple statistics

You can do simple statistics functions like `mean()`, `sum()` or `median()` by limiting the `LazyFrame` to one variable. Here is how you would get a median of `hourly_wages`:

```Rust
// Simple statistics (single point)
let mean_hourly_wages = lf
    .clone()
    .select([col("hourly_wages")])
    .median()
    .collect()
    .unwrap();

println!("Mean hourly wages (whole period):\n\n{mean_hourly_wages}\n");
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

You can get a lot more different statistics with other functions like `min()`, `quantile()` or `max()`. For the quantiles, you have to provide a `QuantileMethod`.

```Rust
let hourly_wages_stats = lf
    .clone()
    .select([
        (len() / lit(1000)).alias("count (x1000)"),
        col("hourly_wages").mean().alias("mean"),
        col("hourly_wages").min().alias("min"),
        col("hourly_wages")
            .quantile(lit(0.01), QuantileMethod::Nearest)
            .alias("p01"),
        col("hourly_wages")
            .quantile(lit(0.25), QuantileMethod::Nearest)
            .alias("p25"),
        col("hourly_wages").median().alias("median"),
        col("hourly_wages")
            .quantile(lit(0.75), QuantileMethod::Nearest)
            .alias("p75"),
        col("hourly_wages")
            .quantile(lit(0.99), QuantileMethod::Nearest)
            .alias("p99"),
        col("hourly_wages").max().alias("max"),
    ])
    .unpivot(UnpivotArgsDSL::default())
    .collect()
    .unwrap();

println!(
    "Table of summary statistics about hourly wages (whole period):\n\n{hourly_wages_stats}\n"
);
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

```Rust
// Simple statistics by category
let mean_hourly_wages_by_prov = lf
    .clone()
    .group_by([col("prov")])
    .agg([col("hourly_wages")
        .mean()
        .round(2, RoundMode::HalfAwayFromZero)])
    .collect()
    .unwrap();

println!("Mean hourly wages by province (whole period):\n\n{mean_hourly_wages_by_prov}\n");
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

If you have survey weights, like the `finalwt` variable on the Labour Force Survey (LFS), you can create statistics by that weight, wight formulas. In this example, a custom function called `weighted_quantile` is create the provide quantiles that respect the survey weights:

```Rust
// Calculate weighted quantile
fn weighted_quantile(col: Expr, wt: Expr, percentile: Expr) -> Expr {
    col.sort_by(
        [(wt.clone().cast(DataType::Float64).cum_sum(false)
            / wt.clone().cast(DataType::Float64).sum()
            - percentile)
            .abs()],
        SortMultipleOptions::default(),
    )
    .first()
    .alias("median")
}

// Weighted statistics
let hourly_wages_stats_wt = lf
    .clone()
    .sort(["hourly_wages"], SortMultipleOptions::new())
    .select([
        ((col("finalwt").sum()) / lit(1000000)).alias("count (x100,000)"),
        (((col("hourly_wages") * col("finalwt")).sum()) / col("finalwt").sum())
            .alias("mean")
            .round(2, RoundMode::HalfAwayFromZero),
        col("hourly_wages").min().alias("min"),
        weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.01)).alias("p01"),
        weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.25)).alias("p25"),
        weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.50)).alias("median"),
        weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.75)).alias("p75"),
        weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.99)).alias("p99"),
        col("hourly_wages").max().alias("max"),
    ])
    .unpivot(UnpivotArgsDSL::default())
    .collect()
    .unwrap();

println!(
    "Table of weighted summary statistics about hourly wages (whole period):\n\n{hourly_wages_stats_wt}\n"
);
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

```Rust
// Weighted statistics (by gender)
let hourly_wages_stats_wt_by_gender = lf
    .clone()
    .sort(["gender", "hourly_wages"], SortMultipleOptions::new())
    .group_by(["gender"])
    .agg([
        ((col("finalwt").sum()) / lit(1000000)).alias("count (x100,000)"),
        (((col("hourly_wages") * col("finalwt")).sum()) / col("finalwt").sum())
            .alias("mean")
            .round(2, RoundMode::HalfAwayFromZero),
        col("hourly_wages").min().alias("min"),
        weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.01)).alias("p01"),
        weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.25)).alias("p25"),
        weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.50)).alias("median"),
        weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.75)).alias("p75"),
        weighted_quantile(col("hourly_wages"), col("finalwt"), lit(0.99)).alias("p99"),
        col("hourly_wages").max().alias("max"),
    ])
    .collect()
    .unwrap()
    .unpivot(
        [
            "count (x100,000)",
            "mean",
            "min",
            "p01",
            "p25",
            "median",
            "p75",
            "p99",
            "max",
        ],
        ["gender"],
    )
    .unwrap();

let hourly_wages_stats_wt_by_gender = pivot_stable(
    &hourly_wages_stats_wt_by_gender,
    ["gender"],
    Some(["variable"]),
    Some(["value"]),
    false,
    None,
    None,
)
.unwrap();

println!(
    "Table of weighted summary statistics about hourly wages (whole period):\n\n{hourly_wages_stats_wt_by_gender}\n"
);
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