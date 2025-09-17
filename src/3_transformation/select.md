# Select

This chapter will explore how to keep or drop columns from your data. You can run the examples with `cargo run --example 3_2_1_select`.

## Select

To have access to data, lets connect to the parquet LFS data:

```rust
=== Rust 3_2_1_select evcxr
=== Rust 3_2_1_select imports
=== Rust 3_2_1_select block_1
```

Using `collect_schema`, we can collect the names of the columns in the `LazyFrame`. Here is code to collect a vector of variable names: 

```rust
=== Rust 3_2_1_select block_2
```

```
Vector of the 60 variables in the LazyFrame: ["rec_num", "survyear", "survmnth", "lfsstat", "prov", "cma", "age_12", "age_6", "gender", "marstat", "educ", "mjh", "everwork", "ftptlast", "cowmain", "immig", "naics_21", "noc_10", "noc_43", "yabsent", "wksaway", "payaway", "uhrsmain", "ahrsmain", "ftptmain", "utothrs", "atothrs", "hrsaway", "yaway", "paidot", "unpaidot", "xtrahrs", "whypt", "tenure", "prevten", "hrlyearn", "union", "permtemp", "estsize", "firmsize", "durunemp", "flowunem", "unemftpt", "whylefto", "whyleftn", "durjless", "availabl", "lkpubag", "lkemploy", "lkrels", "lkatads", "lkansads", "lkothern", "prioract", "ynolook", "tlolook", "schooln", "efamtype", "agyownk", "finalwt"]
```

Now, using `select()` you can select (i.e. keep) various columns using the `col()` function. With the `regex` Polars crate feature, you can also use regular expressions to identify columns following a pattern. This pattern must start with `^` and end with `$`. In this example, we are keeping `survyear` (Survey year), `survmnth` (Survey month), `prov` (Province), `hrlyearn` (Usual hourly wages) and `finalwt` (Standard final weight). With `alias` we are renaming `hrlyearn` to `hourly_wages`.

```rust
=== Rust 3_2_1_select block_3
```

```
shape: (5, 5)
┌──────────┬──────────┬──────┬──────────────┬─────────┐
│ survyear ┆ survmnth ┆ prov ┆ hourly_wages ┆ finalwt │
│ ---      ┆ ---      ┆ ---  ┆ ---          ┆ ---     │
│ i64      ┆ i64      ┆ i64  ┆ i64          ┆ i64     │
╞══════════╪══════════╪══════╪══════════════╪═════════╡
│ 2011     ┆ 1        ┆ 59   ┆ null         ┆ 109     │
│ 2011     ┆ 1        ┆ 48   ┆ null         ┆ 62      │
│ 2011     ┆ 1        ┆ 47   ┆ null         ┆ 71      │
│ 2011     ┆ 1        ┆ 35   ┆ null         ┆ 345     │
│ 2011     ┆ 1        ┆ 12   ┆ 2462         ┆ 105     │
└──────────┴──────────┴──────┴──────────────┴─────────┘
```

## Remove

You can also drop variables by selecting `all()` variables and providing a vector of variables to drop to `exclude_cols()`.

```rust
=== Rust 3_2_1_select block_4
```

```
shape: (5, 3)
┌──────────┬──────────┬─────────┐
│ survyear ┆ survmnth ┆ finalwt │
│ ---      ┆ ---      ┆ ---     │
│ i64      ┆ i64      ┆ i64     │
╞══════════╪══════════╪═════════╡
│ 2011     ┆ 1        ┆ 109     │
│ 2011     ┆ 1        ┆ 62      │
│ 2011     ┆ 1        ┆ 71      │
│ 2011     ┆ 1        ┆ 345     │
│ 2011     ┆ 1        ┆ 105     │
└──────────┴──────────┴─────────┘
```

The `exclude_cols()` should be used sparingly by letting your query optimization (e.g. summary of data on requested variables only) do the work for you. In other words, an analytical pipeline will naturally ignore some columns and Polars will automatically drop them when no longer relevant. 