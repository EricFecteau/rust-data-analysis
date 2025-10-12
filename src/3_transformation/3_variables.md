# Variables

This section will explore how to create new variables, derived from other variables and literals. You can run the examples in this chapter with `cargo run --example 3_3_1_variables`.

## Literals

To have access to data, lets connect to the parquet LFS data:

```rust
=== Rust 3_3_1_variables imports
=== Rust 3_3_1_variables block_1
```

In the same way as selecting existing columns with `select()`, you can create variables from literals using `lit()` by giving them a name with `alias()`. As you can see in this example, you can create a new column from a literal with `lit(5).alias("five")` or a new variable from a formula of a mix of literals (e.g. `(lit(5) + lit(7) - lit(2)).alias("ten")`). You can use any of the arithmetic expressions (`-`, `+`, `*`, `/`, `%`).

```rust
=== Rust 3_3_1_variables block_2
```

```
shape: (5, 6)
┌──────────┬──────────┬──────┬──────────────┬──────┬─────┐
│ survyear ┆ survmnth ┆ prov ┆ hourly_wages ┆ five ┆ ten │
│ ---      ┆ ---      ┆ ---  ┆ ---          ┆ ---  ┆ --- │
│ i64      ┆ i64      ┆ i64  ┆ i64          ┆ i32  ┆ i32 │
╞══════════╪══════════╪══════╪══════════════╪══════╪═════╡
│ 2011     ┆ 1        ┆ 59   ┆ null         ┆ 5    ┆ 10  │
│ 2011     ┆ 1        ┆ 48   ┆ null         ┆ 5    ┆ 10  │
│ 2011     ┆ 1        ┆ 47   ┆ null         ┆ 5    ┆ 10  │
│ 2011     ┆ 1        ┆ 35   ┆ null         ┆ 5    ┆ 10  │
│ 2011     ┆ 1        ┆ 12   ┆ 2462         ┆ 5    ┆ 10  │
└──────────┴──────────┴──────┴──────────────┴──────┴─────┘
```

## Deriving from existing columns

Now that we have created the `five` and `ten` columns, we can derive the `fifteen` column by adding the two columns together with `col()`. In this example, we use `with_column()` instead of `select()` as it keeps the existing columns. For adding multiple columns `with_columns()` is available. A `select()` with an `all()` selection would do the same.

> [!NOTE]
> You can not use a newly created variable (or a renamed variable) in the same `select()`. For example, you would not be able to run `(col("five") + col("ten")).alias("fifteen")` in the same `select()` as you created the `five` and `ten` columns. 

```rust
=== Rust 3_3_1_variables block_3
```

```
shape: (5, 7)
┌──────────┬──────────┬──────┬──────────────┬──────┬─────┬─────────┐
│ survyear ┆ survmnth ┆ prov ┆ hourly_wages ┆ five ┆ ten ┆ fifteen │
│ ---      ┆ ---      ┆ ---  ┆ ---          ┆ ---  ┆ --- ┆ ---     │
│ i64      ┆ i64      ┆ i64  ┆ i64          ┆ i32  ┆ i32 ┆ i32     │
╞══════════╪══════════╪══════╪══════════════╪══════╪═════╪═════════╡
│ 2011     ┆ 1        ┆ 59   ┆ null         ┆ 5    ┆ 10  ┆ 15      │
│ 2011     ┆ 1        ┆ 48   ┆ null         ┆ 5    ┆ 10  ┆ 15      │
│ 2011     ┆ 1        ┆ 47   ┆ null         ┆ 5    ┆ 10  ┆ 15      │
│ 2011     ┆ 1        ┆ 35   ┆ null         ┆ 5    ┆ 10  ┆ 15      │
│ 2011     ┆ 1        ┆ 12   ┆ 2462         ┆ 5    ┆ 10  ┆ 15      │
└──────────┴──────────┴──────┴──────────────┴──────┴─────┴─────────┘
```

Now lets look at real data and how to modify it. As we can see, `hourly_wages` is an `i64` in cents. We might want to convert it to dollars and cents (e.g. from `2462` to `24.62`). To do this, we have to convert he column to a `f64` using `.cast(DataType::Float64)`:

```rust
=== Rust 3_3_1_variables block_4
```

```
shape: (5, 5)
┌──────────┬──────────┬──────┬──────────────┬───────────────┐
│ survyear ┆ survmnth ┆ prov ┆ hourly_wages ┆ wages_dollars │
│ ---      ┆ ---      ┆ ---  ┆ ---          ┆ ---           │
│ i64      ┆ i64      ┆ i64  ┆ i64          ┆ f64           │
╞══════════╪══════════╪══════╪══════════════╪═══════════════╡
│ 2011     ┆ 1        ┆ 12   ┆ 2462         ┆ 24.62         │
│ 2011     ┆ 1        ┆ 13   ┆ 3769         ┆ 37.69         │
│ 2011     ┆ 1        ┆ 35   ┆ 2706         ┆ 27.06         │
│ 2011     ┆ 1        ┆ 35   ┆ 2644         ┆ 26.44         │
│ 2011     ┆ 1        ┆ 35   ┆ 1724         ┆ 17.24         │
└──────────┴──────────┴──────┴──────────────┴───────────────┘
```

# Conditionally create values

You can also create conditional values for a column, using the `when()`, `then()` and `otherwise()` chain. This example below creates the `wage_cat` column with `<= 20` = `Low`, `> 20 & <= 50` = `Medium` and `> 50` = `High`.

```rust
=== Rust 3_3_1_variables block_5
```

```
shape: (5, 6)
┌──────────┬──────────┬──────┬──────────────┬───────────────┬──────────┐
│ survyear ┆ survmnth ┆ prov ┆ hourly_wages ┆ wages_dollars ┆ wage_cat │
│ ---      ┆ ---      ┆ ---  ┆ ---          ┆ ---           ┆ ---      │
│ i64      ┆ i64      ┆ i64  ┆ i64          ┆ f64           ┆ str      │
╞══════════╪══════════╪══════╪══════════════╪═══════════════╪══════════╡
│ 2011     ┆ 1        ┆ 12   ┆ 2462         ┆ 24.62         ┆ Medium   │
│ 2011     ┆ 1        ┆ 13   ┆ 3769         ┆ 37.69         ┆ Medium   │
│ 2011     ┆ 1        ┆ 35   ┆ 2706         ┆ 27.06         ┆ Medium   │
│ 2011     ┆ 1        ┆ 35   ┆ 2644         ┆ 26.44         ┆ Medium   │
│ 2011     ┆ 1        ┆ 35   ┆ 1724         ┆ 17.24         ┆ Low      │
└──────────┴──────────┴──────┴──────────────┴───────────────┴──────────┘
```

## Replace data

You can replace the values of a column using the `replace_strict()` function. For example, to change the [SGC provincial code to an Alpha code](https://www12.statcan.gc.ca/census-recensement/2021/ref/dict/tab/index-eng.cfm?ID=t1_8) for the provinces in the dataset, you can use a "from" `Series` and a "to" `Series` and the `prov` variable will be replaced with Alpha codes.

```rust
=== Rust 3_3_1_variables block_6
```

```
shape: (5, 6)
┌──────────┬──────────┬──────┬──────────────┬───────────────┬──────────┐
│ survyear ┆ survmnth ┆ prov ┆ hourly_wages ┆ wages_dollars ┆ wage_cat │
│ ---      ┆ ---      ┆ ---  ┆ ---          ┆ ---           ┆ ---      │
│ i64      ┆ i64      ┆ str  ┆ i64          ┆ f64           ┆ str      │
╞══════════╪══════════╪══════╪══════════════╪═══════════════╪══════════╡
│ 2011     ┆ 1        ┆ NS   ┆ 2462         ┆ 24.62         ┆ Medium   │
│ 2011     ┆ 1        ┆ NB   ┆ 3769         ┆ 37.69         ┆ Medium   │
│ 2011     ┆ 1        ┆ ON   ┆ 2706         ┆ 27.06         ┆ Medium   │
│ 2011     ┆ 1        ┆ ON   ┆ 2644         ┆ 26.44         ┆ Medium   │
│ 2011     ┆ 1        ┆ ON   ┆ 1724         ┆ 17.24         ┆ Low      │
└──────────┴──────────┴──────┴──────────────┴───────────────┴──────────┘
```