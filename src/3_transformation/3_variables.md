# Variables

This section will explore how to create new variables, derived from other variables and literals. You can run the examples in this chapter with `cargo run --example 3_3_1_variables`.

## Literals

To have access to data, lets connect to the parquet Census data:

```rust
=== Rust 3_3_1_variables imports
=== Rust 3_3_1_variables block_1
```

In the same way as selecting existing columns with `select()`, you can create variables from literals using `lit()` by giving them a name with `alias()`. As you can see in this example, you can create a new column from a literal with `lit(5).alias("five")` or a new variable from a formula of a mix of literals (e.g. `(lit(5) + lit(7) - lit(2)).alias("ten")`). You can use any of the arithmetic expressions (`-`, `+`, `*`, `/`, `%`).

```rust
=== Rust 3_3_1_variables block_2
```

```
shape: (5, 5)
┌───────────┬───────────┬────────┬──────┬─────┐
│ age_group ┆ region    ┆ income ┆ five ┆ ten │
│ ---       ┆ ---       ┆ ---    ┆ ---  ┆ --- │
│ i64       ┆ str       ┆ i64    ┆ i32  ┆ i32 │
╞═══════════╪═══════════╪════════╪══════╪═════╡
│ 1         ┆ E12000001 ┆ null   ┆ 5    ┆ 10  │
│ 1         ┆ E12000001 ┆ null   ┆ 5    ┆ 10  │
│ 1         ┆ E12000001 ┆ null   ┆ 5    ┆ 10  │
│ 1         ┆ E12000001 ┆ null   ┆ 5    ┆ 10  │
│ 1         ┆ E12000001 ┆ null   ┆ 5    ┆ 10  │
└───────────┴───────────┴────────┴──────┴─────┘
```

## Deriving from existing columns

Now that we have created the `five` and `ten` columns, we can derive the `fifteen` column by adding the two columns together with `col()`. In this example, we use `with_column()` instead of `select()` as it keeps the existing columns. For adding multiple columns `with_columns()` is available. A `select()` with an `all()` selection would do the same.

> [!NOTE]
> You can not use a newly created variable (or a renamed variable) in the same `select()`. For example, you would not be able to run `(col("five") + col("ten")).alias("fifteen")` in the same `select()` as you created the `five` and `ten` columns. 

```rust
=== Rust 3_3_1_variables block_3
```

```
shape: (5, 6)
┌───────────┬───────────┬────────┬──────┬─────┬─────────┐
│ age_group ┆ region    ┆ income ┆ five ┆ ten ┆ fifteen │
│ ---       ┆ ---       ┆ ---    ┆ ---  ┆ --- ┆ ---     │
│ i64       ┆ str       ┆ i64    ┆ i32  ┆ i32 ┆ i32     │
╞═══════════╪═══════════╪════════╪══════╪═════╪═════════╡
│ 1         ┆ E12000001 ┆ null   ┆ 5    ┆ 10  ┆ 15      │
│ 1         ┆ E12000001 ┆ null   ┆ 5    ┆ 10  ┆ 15      │
│ 1         ┆ E12000001 ┆ null   ┆ 5    ┆ 10  ┆ 15      │
│ 1         ┆ E12000001 ┆ null   ┆ 5    ┆ 10  ┆ 15      │
│ 1         ┆ E12000001 ┆ null   ┆ 5    ┆ 10  ┆ 15      │
└───────────┴───────────┴────────┴──────┴─────┴─────────┘
```

Now lets look at real data and how to modify it. As we can see, `income` is an `i64`. A 2% inflation increase can be added to this value, once it has been converted to an `f64` using `.cast(DataType::Float64)`, by multiplying by `1.02`:

```rust
=== Rust 3_3_1_variables block_4
```

```
shape: (5, 4)
┌───────────┬───────────┬────────┬─────────────┐
│ age_group ┆ region    ┆ income ┆ income_infl │
│ ---       ┆ ---       ┆ ---    ┆ ---         │
│ i64       ┆ str       ┆ i64    ┆ f64         │
╞═══════════╪═══════════╪════════╪═════════════╡
│ 2         ┆ E12000001 ┆ 50811  ┆ 51827.22    │
│ 2         ┆ E12000001 ┆ 70224  ┆ 71628.48    │
│ 2         ┆ E12000001 ┆ 89534  ┆ 91324.68    │
│ 2         ┆ E12000001 ┆ 93123  ┆ 94985.46    │
│ 2         ┆ E12000001 ┆ 82122  ┆ 83764.44    │
└───────────┴───────────┴────────┴─────────────┘
```

# Conditionally create values

You can also create conditional values for a column, using the `when()`, `then()` and `otherwise()` chain. This example below creates the `income_cat` column with `<= 30_000` = `Low`, `> 30_000 & <= 70_000` = `Medium` and `> 70_000` = `High`.

```rust
=== Rust 3_3_1_variables block_5
```

```
shape: (5, 5)
┌───────────┬───────────┬────────┬─────────────┬────────────┐
│ age_group ┆ region    ┆ income ┆ income_infl ┆ income_cat │
│ ---       ┆ ---       ┆ ---    ┆ ---         ┆ ---        │
│ i64       ┆ str       ┆ i64    ┆ f64         ┆ str        │
╞═══════════╪═══════════╪════════╪═════════════╪════════════╡
│ 2         ┆ E12000001 ┆ 50811  ┆ 51827.22    ┆ Medium     │
│ 2         ┆ E12000001 ┆ 70224  ┆ 71628.48    ┆ High       │
│ 2         ┆ E12000001 ┆ 89534  ┆ 91324.68    ┆ High       │
│ 2         ┆ E12000001 ┆ 93123  ┆ 94985.46    ┆ High       │
│ 2         ┆ E12000001 ┆ 82122  ┆ 83764.44    ┆ High       │
└───────────┴───────────┴────────┴─────────────┴────────────┘
```

## Replace data

You can replace the values of a column using the `replace_strict()` function. For example, to change the [GCC Geographic Code](https://en.wikipedia.org/wiki/GSS_coding_system) for the regions in the dataset, you can use a "from" `Series` and a "to" `Series` and the `region` variable will be replaced with Alpha codes.

```rust
=== Rust 3_3_1_variables block_6
```

```
shape: (5, 5)
┌───────────┬────────────┬────────┬─────────────┬────────────┐
│ age_group ┆ region     ┆ income ┆ income_infl ┆ income_cat │
│ ---       ┆ ---        ┆ ---    ┆ ---         ┆ ---        │
│ i64       ┆ str        ┆ i64    ┆ f64         ┆ str        │
╞═══════════╪════════════╪════════╪═════════════╪════════════╡
│ 2         ┆ North East ┆ 50811  ┆ 51827.22    ┆ Medium     │
│ 2         ┆ North East ┆ 70224  ┆ 71628.48    ┆ High       │
│ 2         ┆ North East ┆ 89534  ┆ 91324.68    ┆ High       │
│ 2         ┆ North East ┆ 93123  ┆ 94985.46    ┆ High       │
│ 2         ┆ North East ┆ 82122  ┆ 83764.44    ┆ High       │
└───────────┴────────────┴────────┴─────────────┴────────────┘
```