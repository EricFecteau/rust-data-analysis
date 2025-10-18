# Joins

This section explore how to join two datasets, either by stacking them one on top of the other (same columns) or by stacking them side by side (same rows).

## Concatenate

First, we create a vector containing five times the 1% Census sample. In `./data/parquet` we have 1% sample files of the UK Census. Each 1% sample is identical.

```rust
=== Rust 3_5_1_joins imports
=== Rust 3_5_1_joins block_1
```

To concatenate data of the same row-shape on top of each other, we can use the `concat` function by listing the LazyFrames we want to stack tegether. Here, we can concatenate all four 1% sample of the UK Census, to create a 5% sample of the Census from the `census_chunk` vector.

```rust
=== Rust 3_5_1_joins block_2
```

If we print the result, it shows that we have a DataFrame of over 3 million rows (5 x ~600,000 rows per 1% sample). It is also possible to see that `chunk` has multiple monthly values.

```rust
=== Rust 3_5_1_joins block_3
```

```
shape: (3_021_755, 22)
┌─────────────────┬────────┬───────┬──────┬───┬───────────┬────────┬────────┬───────┐
│ id              ┆ social ┆ birth ┆ econ ┆ … ┆ keep_type ┆ income ┆ weight ┆ chunk │
│ ---             ┆ ---    ┆ ---   ┆ ---  ┆   ┆ ---       ┆ ---    ┆ ---    ┆ ---   │
│ str             ┆ i64    ┆ i64   ┆ i64  ┆   ┆ i64       ┆ i64    ┆ i64    ┆ i64   │
╞═════════════════╪════════╪═══════╪══════╪═══╪═══════════╪════════╪════════╪═══════╡
│ PTS000000588097 ┆ 4      ┆ 1     ┆ 1    ┆ … ┆ 1         ┆ 46223  ┆ 109    ┆ 1     │
│ PTS000000000320 ┆ -8     ┆ 1     ┆ 5    ┆ … ┆ 1         ┆ null   ┆ 97     ┆ 1     │
│ PTS000000397448 ┆ -8     ┆ 2     ┆ 5    ┆ … ┆ 1         ┆ null   ┆ 90     ┆ 1     │
│ PTS000000082442 ┆ -8     ┆ 1     ┆ 5    ┆ … ┆ 1         ┆ null   ┆ 108    ┆ 1     │
│ PTS000000016066 ┆ 4      ┆ 1     ┆ 8    ┆ … ┆ 1         ┆ null   ┆ 75     ┆ 1     │
│ …               ┆ …      ┆ …     ┆ …    ┆ … ┆ …         ┆ …      ┆ …      ┆ …     │
│ PTS000000166524 ┆ -8     ┆ 2     ┆ 6    ┆ … ┆ 1         ┆ null   ┆ 110    ┆ 5     │
│ PTS000000246489 ┆ 2      ┆ 1     ┆ 1    ┆ … ┆ 1         ┆ 26757  ┆ 107    ┆ 5     │
│ PTS000000177162 ┆ -8     ┆ 1     ┆ 9    ┆ … ┆ 1         ┆ null   ┆ 75     ┆ 5     │
│ PTS000000377217 ┆ -8     ┆ 1     ┆ 6    ┆ … ┆ 1         ┆ null   ┆ 113    ┆ 5     │
│ PTS000000377192 ┆ 1      ┆ 2     ┆ 2    ┆ … ┆ 1         ┆ 36811  ┆ 121    ┆ 5     │
└─────────────────┴────────┴───────┴──────┴───┴───────────┴────────┴────────┴───────┘
```

## Joins

Polars has multiple options for joining data by row. To make the data visualization simpler, the below code processes the data found in the `census_chunk` vector, keeps only a few variables, removes those without any income and renames the `income` to reflect the chunk number of the data. Note that we will use `id` as a linkage key, but each chunk is identical (the same 1% sample of the UK Census), so we will drop random 50% of the columns from each to better show how joins work.

```rust
=== Rust 3_5_1_joins block_4
```

Here is what each chunk of data looks like at this point:

```rust
=== Rust 3_5_1_joins block_5
```

```
shape: (150_000, 2)
┌─────────────────┬───────┐
│ id              ┆ inc_1 │
│ ---             ┆ ---   │
│ str             ┆ i64   │
╞═════════════════╪═══════╡
│ PTS000000407151 ┆ 38619 │
│ PTS000000127436 ┆ 62296 │
│ PTS000000499339 ┆ 13414 │
│ PTS000000397534 ┆ 19402 │
│ PTS000000490519 ┆ 63475 │
│ …               ┆ …     │
│ PTS000000403316 ┆ 67194 │
│ PTS000000150748 ┆ 85943 │
│ PTS000000022816 ┆ 88017 │
│ PTS000000140902 ┆ 66287 │
│ PTS000000078467 ┆ 36567 │
└─────────────────┴───────┘
```

Now that we have simpler data, we can join these. In this example, we are doing multiple left joins in a row with the `left_join` function, always keeping the original population (akin to creating a cohort). To do these joins, all we need is the data and the left and right key (`id` in this example).

```rust
=== Rust 3_5_1_joins block_6
```

This gives us a longitudinal cohort, keeping the population from the first dataset (january):

```
shape: (150_000, 6)
┌─────────────────┬───────┬───────┬───────┬───────┬───────┐
│ id              ┆ inc_1 ┆ inc_2 ┆ inc_3 ┆ inc_4 ┆ inc_5 │
│ ---             ┆ ---   ┆ ---   ┆ ---   ┆ ---   ┆ ---   │
│ str             ┆ i64   ┆ i64   ┆ i64   ┆ i64   ┆ i64   │
╞═════════════════╪═══════╪═══════╪═══════╪═══════╪═══════╡
│ PTS000000407151 ┆ 38619 ┆ 38619 ┆ null  ┆ null  ┆ null  │
│ PTS000000127436 ┆ 62296 ┆ 62296 ┆ null  ┆ null  ┆ 62296 │
│ PTS000000499339 ┆ 13414 ┆ null  ┆ 13414 ┆ 13414 ┆ 13414 │
│ PTS000000397534 ┆ 19402 ┆ null  ┆ 19402 ┆ null  ┆ 19402 │
│ PTS000000490519 ┆ 63475 ┆ null  ┆ 63475 ┆ null  ┆ 63475 │
│ …               ┆ …     ┆ …     ┆ …     ┆ …     ┆ …     │
│ PTS000000403316 ┆ 67194 ┆ null  ┆ null  ┆ 67194 ┆ 67194 │
│ PTS000000150748 ┆ 85943 ┆ 85943 ┆ null  ┆ null  ┆ 85943 │
│ PTS000000022816 ┆ 88017 ┆ null  ┆ null  ┆ 88017 ┆ null  │
│ PTS000000140902 ┆ 66287 ┆ null  ┆ 66287 ┆ null  ┆ null  │
│ PTS000000078467 ┆ 36567 ┆ 36567 ┆ 36567 ┆ null  ┆ null  │
└─────────────────┴───────┴───────┴───────┴───────┴───────┘
```
In the same way, we can also use other types of joins, like the `inner_join`:

```rust
=== Rust 3_5_1_joins block_7
```

This creates a cohort of those who are in every dataset.

```
shape: (9_981, 6)
┌─────────────────┬───────┬───────┬───────┬───────┬───────┐
│ id              ┆ inc_1 ┆ inc_2 ┆ inc_3 ┆ inc_4 ┆ inc_5 │
│ ---             ┆ ---   ┆ ---   ┆ ---   ┆ ---   ┆ ---   │
│ str             ┆ i64   ┆ i64   ┆ i64   ┆ i64   ┆ i64   │
╞═════════════════╪═══════╪═══════╪═══════╪═══════╪═══════╡
│ PTS000000282273 ┆ 78586 ┆ 78586 ┆ 78586 ┆ 78586 ┆ 78586 │
│ PTS000000221209 ┆ 19179 ┆ 19179 ┆ 19179 ┆ 19179 ┆ 19179 │
│ PTS000000098103 ┆ 41441 ┆ 41441 ┆ 41441 ┆ 41441 ┆ 41441 │
│ PTS000000538070 ┆ 74324 ┆ 74324 ┆ 74324 ┆ 74324 ┆ 74324 │
│ PTS000000364383 ┆ 72375 ┆ 72375 ┆ 72375 ┆ 72375 ┆ 72375 │
│ …               ┆ …     ┆ …     ┆ …     ┆ …     ┆ …     │
│ PTS000000182678 ┆ 45183 ┆ 45183 ┆ 45183 ┆ 45183 ┆ 45183 │
│ PTS000000122168 ┆ 30325 ┆ 30325 ┆ 30325 ┆ 30325 ┆ 30325 │
│ PTS000000079312 ┆ 74982 ┆ 74982 ┆ 74982 ┆ 74982 ┆ 74982 │
│ PTS000000461362 ┆ 96417 ┆ 96417 ┆ 96417 ┆ 96417 ┆ 96417 │
│ PTS000000345005 ┆ 31422 ┆ 31422 ┆ 31422 ┆ 31422 ┆ 31422 │
└─────────────────┴───────┴───────┴───────┴───────┴───────┘
```

Polars has multiple of these "simple" joins, including `left_join`, `semi_join`, `full_join`, `inner_join`, `anti_join` and `cross_join`. But you can create significantly more complex joins by building the `join` yourself with the `join` function and all of it's options. For example, here is a `full join` on multiple variables:

> [!NOTE]
> For some reason, Polars does not reconcile the values of the keys in a `full join`, both in the `join` and `full_join` functions. This means that any keys not found in the left create `nulls` in the original key name and any key not found in the right creates `nulls` in the key with an `_right` suffix (e.g. rec_num_right). This can be fixed with an expression that applies to all five joins (called `fix_full_join_vars` below).

```rust
=== Rust 3_5_1_joins block_8
```

This full join keeps a superpopulation of all the five datasets:

```
shape: (286_293, 6)
┌─────────────────┬───────┬───────┬───────┬───────┬───────┐
│ id              ┆ inc_1 ┆ inc_2 ┆ inc_3 ┆ inc_4 ┆ inc_5 │
│ ---             ┆ ---   ┆ ---   ┆ ---   ┆ ---   ┆ ---   │
│ str             ┆ i64   ┆ i64   ┆ i64   ┆ i64   ┆ i64   │
╞═════════════════╪═══════╪═══════╪═══════╪═══════╪═══════╡
│ PTS000000000002 ┆ 19163 ┆ null  ┆ null  ┆ 19163 ┆ null  │
│ PTS000000000005 ┆ 43481 ┆ 43481 ┆ 43481 ┆ null  ┆ 43481 │
│ PTS000000000008 ┆ 70614 ┆ 70614 ┆ 70614 ┆ null  ┆ null  │
│ PTS000000000010 ┆ 62499 ┆ 62499 ┆ 62499 ┆ null  ┆ null  │
│ PTS000000000011 ┆ null  ┆ 30589 ┆ 30589 ┆ null  ┆ null  │
│ …               ┆ …     ┆ …     ┆ …     ┆ …     ┆ …     │
│ PTS000000604345 ┆ null  ┆ 20666 ┆ 20666 ┆ null  ┆ null  │
│ PTS000000604348 ┆ 54153 ┆ 54153 ┆ null  ┆ 54153 ┆ null  │
│ PTS000000604349 ┆ 74670 ┆ null  ┆ 74670 ┆ 74670 ┆ null  │
│ PTS000000604350 ┆ 22652 ┆ 22652 ┆ 22652 ┆ 22652 ┆ null  │
│ PTS000000604351 ┆ null  ┆ 73487 ┆ 73487 ┆ null  ┆ 73487 │
└─────────────────┴───────┴───────┴───────┴───────┴───────┘
```