# Pivots

Pivoting a dataframe allows you to make wide data longer or long data wider. This is done by increasing the number of columns and decreasing the number of rows, or vice versa. 

With Polars, pivots have to be done in-memory. As explained by Polars, "lazy does not implement a pivot because it is impossible to know the schema without materializing the whole dataset". In other words, if, for example, you wanted to pivot wider on the province variable (e.g. make a column for each province), until Polars reads every single row in your dataset it can not know how many columns it would create. Therefore, it can not move forward lazily and continue optimizing the query, without materializing the dataframe. Polars does not allow you to provide a schema to solve this issue lazily. 
Caution should be taken when pivoting large dataframes as it will have to be done eagerly.

## Setup

Since the data has to be materialized (i.e. in-memory) to be pivoted, lets first create some summary data to get a manageable `DataFrame`. To learn how to do summary statistics, see the [summary chapter](../4_stats/summary.md).

In this example, we connect to the Parquet file, summarize the hourly wage data (unweighted) by year and province.

```rust
=== Rust 3_4_1_pivot evcxr
=== Rust 3_4_1_pivot imports
=== Rust 3_4_1_pivot block_1
```

```
shape: (140, 3)
┌──────────┬──────┬───────────────┐
│ survyear ┆ prov ┆ mean_hrlyearn │
│ ---      ┆ ---  ┆ ---           │
│ i64      ┆ str  ┆ f64           │
╞══════════╪══════╪═══════════════╡
│ 2011     ┆ NL   ┆ 22.02         │
│ 2011     ┆ PE   ┆ 19.28         │
│ 2011     ┆ NS   ┆ 20.46         │
│ 2011     ┆ NB   ┆ 19.65         │
│ 2011     ┆ QC   ┆ 21.59         │
│ …        ┆ …    ┆ …             │
│ 2024     ┆ ON   ┆ 36.33         │
│ 2024     ┆ MB   ┆ 30.2          │
│ 2024     ┆ SK   ┆ 32.62         │
│ 2024     ┆ AB   ┆ 36.8          │
│ 2024     ┆ BC   ┆ 36.37         │
└──────────┴──────┴───────────────┘
```

We now have a fairly long `DataFrame` in memory (10 provinces per year, for multiple years (2011 to 2024)).

## Pivot wider

To pivot this long `DataFrame` wider (also simply known as "pivot"), we can take either the year column or the province column and make them individual columns, using the data found in `mean_hrlyearn`. In this example, we will take the 10 provinces and make them columns.

```rust
=== Rust 3_4_1_pivot block_2
```

```
shape: (14, 11)
┌──────────┬───────┬───────┬───────┬───┬───────┬───────┬───────┬───────┐
│ survyear ┆ NL    ┆ PE    ┆ NS    ┆ … ┆ MB    ┆ SK    ┆ AB    ┆ BC    │
│ ---      ┆ ---   ┆ ---   ┆ ---   ┆   ┆ ---   ┆ ---   ┆ ---   ┆ ---   │
│ i64      ┆ f64   ┆ f64   ┆ f64   ┆   ┆ f64   ┆ f64   ┆ f64   ┆ f64   │
╞══════════╪═══════╪═══════╪═══════╪═══╪═══════╪═══════╪═══════╪═══════╡
│ 2011     ┆ 22.02 ┆ 19.28 ┆ 20.46 ┆ … ┆ 20.77 ┆ 23.25 ┆ 25.79 ┆ 23.25 │
│ 2012     ┆ 23.56 ┆ 20.08 ┆ 21.05 ┆ … ┆ 21.26 ┆ 24.14 ┆ 27.18 ┆ 23.69 │
│ 2013     ┆ 24.36 ┆ 20.51 ┆ 21.76 ┆ … ┆ 21.68 ┆ 24.96 ┆ 28.19 ┆ 24.4  │
│ 2014     ┆ 25.04 ┆ 20.85 ┆ 22.41 ┆ … ┆ 22.28 ┆ 25.68 ┆ 28.53 ┆ 24.59 │
│ 2015     ┆ 24.56 ┆ 21.28 ┆ 22.37 ┆ … ┆ 23.05 ┆ 26.34 ┆ 30.04 ┆ 25.38 │
│ …        ┆ …     ┆ …     ┆ …     ┆ … ┆ …     ┆ …     ┆ …     ┆ …     │
│ 2020     ┆ 27.45 ┆ 24.49 ┆ 25.61 ┆ … ┆ 26.74 ┆ 29.5  ┆ 33.42 ┆ 30.0  │
│ 2021     ┆ 28.22 ┆ 25.37 ┆ 26.0  ┆ … ┆ 27.12 ┆ 29.83 ┆ 33.22 ┆ 31.06 │
│ 2022     ┆ 30.24 ┆ 27.16 ┆ 27.1  ┆ … ┆ 27.86 ┆ 30.68 ┆ 33.58 ┆ 32.46 │
│ 2023     ┆ 31.79 ┆ 27.91 ┆ 28.32 ┆ … ┆ 29.13 ┆ 31.79 ┆ 35.24 ┆ 34.77 │
│ 2024     ┆ 33.63 ┆ 29.46 ┆ 30.38 ┆ … ┆ 30.2  ┆ 32.62 ┆ 36.8  ┆ 36.37 │
└──────────┴───────┴───────┴───────┴───┴───────┴───────┴───────┴───────┘
```

This `DataFrame` is simpler to display and analyze.

## Pivot longer

We can do the inverse, by pivoting this wide `DataFrame` to longer (also known as "unpivot"). This will move the 10 province column into one province row. 

```rust
=== Rust 3_4_1_pivot block_3
```

```
shape: (140, 3)
┌──────────┬──────────┬───────┐
│ survyear ┆ variable ┆ value │
│ ---      ┆ ---      ┆ ---   │
│ i64      ┆ str      ┆ f64   │
╞══════════╪══════════╪═══════╡
│ 2011     ┆ NL       ┆ 22.02 │
│ 2012     ┆ NL       ┆ 23.56 │
│ 2013     ┆ NL       ┆ 24.36 │
│ 2014     ┆ NL       ┆ 25.04 │
│ 2015     ┆ NL       ┆ 24.56 │
│ …        ┆ …        ┆ …     │
│ 2020     ┆ BC       ┆ 30.0  │
│ 2021     ┆ BC       ┆ 31.06 │
│ 2022     ┆ BC       ┆ 32.46 │
│ 2023     ┆ BC       ┆ 34.77 │
│ 2024     ┆ BC       ┆ 36.37 │
└──────────┴──────────┴───────┘
```
We are back to the same shape as prior to the pivot longer.
