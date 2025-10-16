# Filter

This chapter will explore how to filter rows from your data. You can run these examples with `cargo run --example 3_1_1_filter`.

## Filters

### Simple filters

To have access to data, lets connect to the parquet Census data:

```Rust
=== Rust 3_1_1_filter imports
=== Rust 3_1_1_filter block_1
```

To filter this data, you can use `filter()` with expressions. In this example, `keep_type` is filtered to "usual resident", `region` is filtered to the region of London ("E12000007"), `age_group` is filtered to greater than or equal to 5 (45 years old and older) and records with null values in `income` are dropped:

```Rust
=== Rust 3_1_1_filter block_2
```

To reference a column in a filter, it must be called with the `col()` function. For values (i.e. literals), they must be referenced with the `lit()` function. They can bed compared using equality comparison such as:

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

The `and()` and `or()` functions are especially important when crafting more complex filters. For example, you can craft this filter to collect individuals 55 and over from the North East and individuals 54 and under from the North West: 

```Rust 
=== Rust 3_1_1_filter block_4
```

You can print the expression to see how it's being evaluated. This is especially useful when you use an IDE that can highlight bracket and parenthesis pairs.

```
[([([(col("region")) == ("E12000001")]) & ([(col("age_group")) >= (dyn int: 6)])]) | ([([(col("survyear")) == ("E12000002")]) & ([(col("age_group")) <= (dyn int: 6)])])]
```

You can then apply the expression with `.filter()`:

```Rust
=== Rust 3_1_1_filter block_5
```

### Value is in a list

With the `is_in` crate feature, you can see if a `col()` is within a list of `lit()`. The right side of the expression takes a `Polars::Series`, that can be built using `Series::from_iter(vec![<vals>])`. In this example, we see if `industry` is equal to Manufacturing (2), Construction (4), Transport and communication (6) or Public administration, education and health (8).

```Rust
=== Rust 3_1_1_filter block_6
```

## Lazy evaluation optimization

Filtering is a perfect example to show how `LazyFrame` use optimized queries, especially when using partitioned parquet files, as created in the [Parquet](../2_data/3_parquet.md#writing) chapter. This section can be run with `cargo run -r --example 3_1_2_filter_opt` (release mode is important for simple benchmarking).

> [!NOTE]
> Optimization also works when connecting to data on the Cloud.

First, lets connect to the `./data/large/census.parquet` file that contains over 60 million rows of Census data, in one extremely compressed parquet file (approximately 13 MB). Next, lets filter this file to the region of london (region = "E12000007"), for those aged 45 to 54 (age_group = 5), and non-null values for `income`. Remember, this code creates and execution plan, but does not yet execute it.

```Rust
=== Rust 3_1_2_filter_opt block_1
```

Second, lets connect to the `./data/large/partitioned` partitioned dataset, that was partitioned by `region` and by `age_group`. Overall, the files in this partitioned dataset folder will contain over 60 million rows of Census data. This data is split among 70 extremely compressed parquet files, equalling a total of approximately 40 MB. Similar to the large parquet file, this file will be filtered to the region of london (region = "E12000007"), for those aged 45 to 54 (age_group = 5), and non-null values for `income`. Again, nothing is executed at this point. 

```Rust
=== Rust 3_1_2_filter_opt block_2
```

With `LazyFrame`, you can see the execution plan with `.explain()`. Passing `false` gives the unoptimized plan and passing `true` gives the optimized plan. When the plan is executed, it always uses the optimized plan. We can see that the unoptimized execution plan for the single parquet file and partitioned parquet file are similar:

```Rust
=== Rust 3_1_2_filter_opt block_3
```

```
FILTER col("income").is_not_null()
FROM
  FILTER [(col("age_group")) == (5)]
  FROM
    FILTER [(col("region")) == ("E12000007")]
    FROM
      Parquet SCAN [./data/large/census.parquet]
      PROJECT */21 COLUMNS
```

```Rust
=== Rust 3_1_2_filter_opt block_4
```

```
FILTER col("income").is_not_null()
FROM
  FILTER [(col("age_group")) == (5)]
  FROM
    FILTER [(col("region")) == ("E12000007")]
    FROM
      Parquet SCAN [./data/large/partitioned/region=E12000001/age_group=1/00000000.parquet, ... 69 other sources]
      PROJECT 19/21 COLUMNS
```

In both cases, the filters are the same, and the SCAN in both cases touches all files (the large one or all 70 parquet file for the partitioned parquet file). The single file collects all 21 variables and the partitioned one selects 19 of the 21 variables, since `region` and `age_group` are removed from the file due to the hive partitioning structure.

> [!WARNING]
> As of Polars 0.46, query optimization on hive partitioned Parquet files is no longer working. You can follow the bug report [here](https://github.com/pola-rs/polars/issues/24909).

On the other hand, the optimized queries are quite different:

```Rust
=== Rust 3_1_2_filter_opt block_5
```

```
Parquet SCAN [./data/large/census.parquet]
PROJECT */21 COLUMNS
SELECTION: [([(col("income").is_not_null()) & ([(col("age_group")) == (5)])]) & ([(col("region")) == ("E12000007")])]
```

```Rust
=== Rust 3_1_2_filter_opt block_6
```

```
Parquet SCAN [./data/large/partitioned/region=E12000007/age_group=5/00000000.parquet]
PROJECT 19/21 COLUMNS
SELECTION: [([(col("income").is_not_null()) & ([(col("age_group")) == (5)])]) & ([(col("region")) == ("E12000007")])]
```

As you can see, the "selection" (filter) is essentially the same in both, but for the large file, the entirety of the file has to be scanned (e.g. each row has to be verified for all filters) but for the partitioned parquet file, only one file is scanned, and the filter is applied only to the rows in that one file. The partitioned parquet file allows for filters that are in the partitioned columns (e.g. `region` and `age_group`) to skip entire files.

This gives really great time improvements for queries that contain filters for those variables. For example, doing a mean of the income for the filtered population gives pretty significant time differences:

```Rust
=== Rust 3_1_2_filter_opt block_7
```

While the time differs, the one parquet file (between `20 ms` and `40 ms`) is about an order of magnitude slower than the partitioned parquet file (between `2 ms` and `4 ms`), when run in release mode (4-5 times slower when run in debug mode). For extremly large queries (billions of rows) this can have massive advantages.
