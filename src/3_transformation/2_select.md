# Select

This chapter will explore how to keep or drop columns from your data. You can run the examples with `cargo run --example 3_2_1_select`.

## Select

To have access to data, lets connect to the parquet Census data:

```rust
=== Rust 3_2_1_select imports
=== Rust 3_2_1_select block_1
```

Using `collect_schema`, we can collect the names of the columns in the `LazyFrame`. Here is code to collect a vector of variable names: 

```rust
=== Rust 3_2_1_select block_2
```

```
Vector of the 21 variables in the LazyFrame: ["id", "social", "birth", "econ", "ethnic", "health", "fam_type", "hours_worked", "education", "industry", "london", "mar_stat", "occupation", "region", "religion", "residence_type", "age_group", "sex", "keep_type", "income", "chunk"]
```

Now, using `select()` you can select (i.e. keep) various columns using the `col()` function. With the `regex` Polars crate feature, you can also use regular expressions to identify columns following a pattern. This pattern must start with `^` and end with `$`. In this example, we are keeping `age_group`, `region` and `income`. With `alias` we are renaming `income` to `yearly_income`.

```rust
=== Rust 3_2_1_select block_3
```

```
shape: (5, 3)
┌───────────┬───────────┬───────────────┐
│ age_group ┆ region    ┆ yearly_income │
│ ---       ┆ ---       ┆ ---           │
│ i64       ┆ str       ┆ i64           │
╞═══════════╪═══════════╪═══════════════╡
│ 1         ┆ E12000001 ┆ null          │
│ 1         ┆ E12000001 ┆ null          │
│ 1         ┆ E12000001 ┆ null          │
│ 1         ┆ E12000001 ┆ null          │
│ 1         ┆ E12000001 ┆ null          │
└───────────┴───────────┴───────────────┘
```

## Remove

You can also drop variables by selecting `all()` variables and providing a vector of variables to drop to `exclude_cols()`.

```rust
=== Rust 3_2_1_select block_4
```

```
shape: (5, 1)
┌───────────┐
│ age_group │
│ ---       │
│ i64       │
╞═══════════╡
│ 1         │
│ 1         │
│ 1         │
│ 1         │
│ 1         │
└───────────┘
```

The `exclude_cols()` should be used sparingly by letting your query optimization (e.g. summary of data on requested variables only) do the work for you. In other words, an analytical pipeline will naturally ignore some columns and Polars will automatically drop them when no longer relevant. 