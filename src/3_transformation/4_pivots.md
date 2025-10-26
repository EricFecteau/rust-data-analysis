# Pivots

Pivoting a dataframe allows you to make wide data longer or long data wider. This is done by increasing the number of columns and decreasing the number of rows, or vice versa. 

With Polars, pivots have to be done in-memory. As explained by Polars, "lazy does not implement a pivot because it is impossible to know the schema without materializing the whole dataset". In other words, if, for example, you wanted to pivot wider on the region variable (e.g. make a column for each region), until Polars reads every single row in your dataset it can not know how many columns it would create. Therefore, it can not move forward lazily and continue optimizing the query, without materializing the dataframe. Polars does not allow you to provide a schema to solve this issue lazily. Caution should be taken when pivoting large dataframes as it will have to be done eagerly.

## Setup

Since the data has to be materialized (i.e. in-memory) to be pivoted, this section will first create some summary data to get a manageable `DataFrame`. To learn how to do summary statistics, see the [summary chapter](../4_stats/1_summary.md). For now, you can simply run the code below using `cargo run -r --example 3_4_1_pivot`.

In this example, we connect to the Parquet file and summarize the income by region and age group.

```rust
=== Rust 3_4_1_pivot imports
=== Rust 3_4_1_pivot block_1
```

This provides an in-memory `DataFrame` that's fairly long:


```
shape: (60, 3)
┌────────────┬────────────────────────┬─────────────┐
│ region     ┆ age_group              ┆ mean_income │
│ ---        ┆ ---                    ┆ ---         │
│ str        ┆ str                    ┆ f64         │
╞════════════╪════════════════════════╪═════════════╡
│ North East ┆ Aged 16 to 24 years    ┆ 55149.43    │
│ North East ┆ Aged 25 to 34 years    ┆ 55871.1     │
│ North East ┆ Aged 35 to 44 years    ┆ 54567.28    │
│ North East ┆ Aged 45 to 54 years    ┆ 55088.2     │
│ North East ┆ Aged 55 to 64 years    ┆ 55363.42    │
│ …          ┆ …                      ┆ …           │
│ Wales      ┆ Aged 25 to 34 years    ┆ 54208.02    │
│ Wales      ┆ Aged 35 to 44 years    ┆ 55476.43    │
│ Wales      ┆ Aged 45 to 54 years    ┆ 54919.49    │
│ Wales      ┆ Aged 55 to 64 years    ┆ 55173.07    │
│ Wales      ┆ Aged 65 years and over ┆ 56589.58    │
└────────────┴────────────────────────┴─────────────┘
```

## Pivot wider

To pivot this "long" `DataFrame` "wider" (also simply known as a "pivot"), we can take either the region or the age group column and make them individual columns, using the data found in `mean_income`. This example will take the regions and distribute their `mean_income` as columns.

```rust
=== Rust 3_4_1_pivot block_2
```

This `DataFrame` is now simpler to display and analyze.


```
shape: (6, 11)
┌────────────────────────┬────────────┬────────────┬──────────────────────────┬───┬──────────┬────────────┬────────────┬──────────┐
│ age_group              ┆ North East ┆ North West ┆ Yorkshire and The Humber ┆ … ┆ London   ┆ South East ┆ South West ┆ Wales    │
│ ---                    ┆ ---        ┆ ---        ┆ ---                      ┆   ┆ ---      ┆ ---        ┆ ---        ┆ ---      │
│ str                    ┆ f64        ┆ f64        ┆ f64                      ┆   ┆ f64      ┆ f64        ┆ f64        ┆ f64      │
╞════════════════════════╪════════════╪════════════╪══════════════════════════╪═══╪══════════╪════════════╪════════════╪══════════╡
│ Aged 16 to 24 years    ┆ 55149.43   ┆ 54719.54   ┆ 55379.91                 ┆ … ┆ 54769.35 ┆ 54942.88   ┆ 55528.66   ┆ 55487.98 │
│ Aged 25 to 34 years    ┆ 55871.1    ┆ 55198.37   ┆ 55372.78                 ┆ … ┆ 55148.76 ┆ 54538.09   ┆ 54706.78   ┆ 54208.02 │
│ Aged 35 to 44 years    ┆ 54567.28   ┆ 55076.23   ┆ 54601.97                 ┆ … ┆ 54908.1  ┆ 55366.31   ┆ 55010.97   ┆ 55476.43 │
│ Aged 45 to 54 years    ┆ 55088.2    ┆ 55191.91   ┆ 55176.05                 ┆ … ┆ 54521.3  ┆ 55449.41   ┆ 55254.24   ┆ 54919.49 │
│ Aged 55 to 64 years    ┆ 55363.42   ┆ 54798.76   ┆ 54610.19                 ┆ … ┆ 54861.33 ┆ 55076.17   ┆ 55195.69   ┆ 55173.07 │
│ Aged 65 years and over ┆ 54201.06   ┆ 55779.16   ┆ 53755.73                 ┆ … ┆ 54197.31 ┆ 55123.64   ┆ 55186.29   ┆ 56589.58 │
└────────────────────────┴────────────┴────────────┴──────────────────────────┴───┴──────────┴────────────┴────────────┴──────────┘
```

## Pivot longer

We can do the inverse, by pivoting this "wide" `DataFrame` to "longer" (also known as "unpivot"). This will move the region columns into one region column and one value column. 

```rust
=== Rust 3_4_1_pivot block_3
```

This brings the `DataFrame` back to it's shape as prior to the pivot wider.

```
shape: (60, 3)
┌────────────────────────┬────────────┬──────────┐
│ age_group              ┆ variable   ┆ value    │
│ ---                    ┆ ---        ┆ ---      │
│ str                    ┆ str        ┆ f64      │
╞════════════════════════╪════════════╪══════════╡
│ Aged 16 to 24 years    ┆ North East ┆ 55149.43 │
│ Aged 25 to 34 years    ┆ North East ┆ 55871.1  │
│ Aged 35 to 44 years    ┆ North East ┆ 54567.28 │
│ Aged 45 to 54 years    ┆ North East ┆ 55088.2  │
│ Aged 55 to 64 years    ┆ North East ┆ 55363.42 │
│ …                      ┆ …          ┆ …        │
│ Aged 25 to 34 years    ┆ Wales      ┆ 54208.02 │
│ Aged 35 to 44 years    ┆ Wales      ┆ 55476.43 │
│ Aged 45 to 54 years    ┆ Wales      ┆ 54919.49 │
│ Aged 55 to 64 years    ┆ Wales      ┆ 55173.07 │
│ Aged 65 years and over ┆ Wales      ┆ 56589.58 │
└────────────────────────┴────────────┴──────────┘
```
