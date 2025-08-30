# Filter

This chapter will explore how to filter rows from your data. You can run these examples with `cargo run --example 3_1_1_filter`.

## Filters

### Simple filters

You can filter the rows in the data using `filter()`. 

First, lets load the partitioned parquet file:

```Rust
=== Rust 3_1_1_filter evcxr
=== Rust 3_1_1_filter imports
=== Rust 3_1_1_filter block_1
```

You can then filter using expressions. In this example, we have multiple filter filtering the data on the `survyear` (2023), `survmnth` (> 6) and `hrlyearn` (not null):

```Rust
=== Rust 3_1_1_filter block_2
```

As you can see, to reference a column in a filter you have to use `col()` function and to reference a literal you have to use `lit()` function. You can compare these using equality comparison such as:

| Equality        | Definition                                               |
|-----------------|----------------------------------------------------------|
| `eq()`          | `==`                                                     |
| `eq_missing()`  | `==` & `None == None`                                    |
| `neq()`         | `!=`                                                     |
| `neq_missing()` | `!=` (including when one side of the equation is `None`) |
| `lt()`          | `<`                                                      |
| `gt()`          | `>`                                                      |
| `gt_eq()`       | `>=`                                                     |
| `lt_eq()`       | `<=`                                                     |
| `not()`         | `!`                                                      |
| `is_null()`     | `== None`                                                |
| `is_not_null()` | `!= None`                                                |

Instead of the multiple `.filter()` you can use one `.filter()` and chain the commands with `.and()`, `.or()` and `.xor()`. The above example can be created using this filter:

```Rust
=== Rust 3_1_1_filter block_3
```

### Complex filters

The `and()` and `or()` options are especially important when crafting more complex filters. For example, you can craft this filter to collect the second half of 2023 and first half of 2024: 

```Rust 
=== Rust 3_1_1_filter block_4
```

You can print the expression to see how it's being evaluated. This is especially useful when you use an IDE that can highlight bracket and parenthesis pairs.

```
[([([(col("survyear")) == (dyn int: 2010)]) & ([(col("survmnth")) > (dyn int: 6)])]) | ([([(col("survyear")) == (dyn int: 2011)]) & ([(col("survmnth")) <= (dyn int: 6)])])]
```

You can then apply the expression with `.filter()`:

```Rust
=== Rust 3_1_1_filter block_5
```

### Value is in a list

With the `is_in` crate feature, you can see if a `col()` is within a list of `lit()`. The right side of the expression takes a `Polars::Series`, that can be built using `Series::from_iter(vec![<vals>])`. oIn this example, we see if `survyear` is equal t 2022, 2023 or 2024.

```Rust
=== Rust 3_1_1_filter block_6
```

TODO: from 2020 to 2024 (not needing all values)

## Lazy evaluation optimization

Filtering is a perfect example to show how `LazyFrame` use optimized queries, especially when using partitioned parquet files, as created in the [Parquet](../2_data/parquet.md#writing) chapter. This section can be run with `cargo run -r --example 3_1_2_filter_opt` (release mode is important for benchmarking).

> [!NOTE]
> This also works when connecting to data on the Cloud.

First, lets connect to the `./data/lfs_large/lfs.parquet` file that contains nearly 15 years of monthly LFS data, 17 million rows, in one parquet file (approximately 300 MB), and filter it to the records in the second half of 2023, and non-null values for `hrlyearn` (hourly wages). Remember, this code creates and execution plan, but does not yet execute it.

```Rust
=== Rust 3_1_2_filter_opt block_1
```

Second, lets connect to the `./data/lfs_large/part` partitioned dataset, that was partitioned by `survyear` and by `survmnth`. All the files in this partitioned dataset folder will contain nearly 15 years of monthly LFS data, 17 million rows, and over 150 parquet files, equalling a total of approximately 300 MB. Similar to the large parquet file, we will filter it to the records in the second half of 2023, and non-null values for `hrlyearn`. Again, nothing is executed at this point. 

```Rust
=== Rust 3_1_2_filter_opt block_2
```

With `LazyFrame`, you can see the execution plan with `.explain()`. Passing `false` gives the unoptimized plan and passing `true` gives the optimized plan. When the plan is executed, it always uses the optimized plan. We can see that the unoptimized execution plan for the single parquet file and partitioned parquet file are similar:

```Rust
=== Rust 3_1_2_filter_opt block_3
```

```
FILTER col("hrlyearn").is_not_null() FROM
  FILTER [(col("survmnth")) > (6)] FROM
    FILTER [(col("survyear")) == (2023)] FROM
      Parquet SCAN [./data/lfs_large/lfs.parquet]
      PROJECT */60 COLUMNS
```

```Rust
=== Rust 3_1_2_filter_opt block_4
```

```
FILTER col("hrlyearn").is_not_null() FROM
  FILTER [(col("survmnth")) > (6)] FROM
    FILTER [(col("survyear")) == (2023)] FROM
      Parquet SCAN [./data/lfs_large/part/survyear=2011/survmnth=1/00000000.parquet, ... 167 other sources]
      PROJECT 58/60 COLUMNS
```

In both cases, the filters are the same, and the SCAN in both cases touches all files (the large one or all 167 parquet file for the partitioned parquet file). The single file collects all 60 variables and the partitioned one selects 58 of the 60 variables, since `survyear` and `survmnth` are removed from the file due to the hive partitioning structure.

On the other hand, the optimized queries are quite different:

```Rust
=== Rust 3_1_2_filter_opt block_5
```

```
Parquet SCAN [./data/lfs_large/lfs.parquet]
PROJECT */60 COLUMNS
SELECTION: [([(col("hrlyearn").is_not_null()) & ([(col("survmnth")) > (6)])]) & ([(col("survyear")) == (2023)])]
```

```Rust
=== Rust 3_1_2_filter_opt block_6
```

```
Parquet SCAN [./data/lfs_large/part/survyear=2023/survmnth=10/00000000.parquet, ... 5 other sources]
PROJECT 58/60 COLUMNS
SELECTION: [([([(col("survmnth")) > (6)]) & ([(col("survyear")) == (2023)])]) & (col("hrlyearn").is_not_null())]
```

As you can see, the "selection" (filter) is essentially the same in both, but for the large file, the entirety of the file has to be scanned (e.g. each row has to be verified for all filters) but for the partitioned parquet file, only 6 files are scanned, and the filter is applied only to the rows in those 6 files. The partitioned parquet file allows for filters that are in the partitioned columns (e.g. `survyear` and `survmnth`) to skip entire files.

This gives really great time improvements for queries that contain filters for those variables. For example, 

```Rust
=== Rust 3_1_2_filter_opt block_7
```

While the time differs, the one parquet file (between `20 ms` and `40 ms`) is about an order of magnitude slower than the partitioned parquet file (between `2 ms` and `4 ms`), when run in release mode (4-5 times slower when run in debug mode).
