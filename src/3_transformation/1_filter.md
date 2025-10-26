# Filter

This chapter will explore how to filter rows from your data. You can run these examples with `cargo run --example 3_1_1_filter`.

## Filters

### Simple filters

To have access to data, lets connect to the 100% sample Census data parquet file:

```Rust
=== Rust 3_1_1_filter imports
=== Rust 3_1_1_filter block_1
```

To filter this data, you can use `filter()` with expressions. In this example, `keep_type` is filtered to "usual resident", `region` is filtered to the region of London ("E12000007"), `age_group` is filtered to 45 years old and older (>= 5) and records with null values in `income` are dropped:

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

With the `is_in` crate feature, you can see if a `col()` is within a list of `lit()`. The right side of the expression takes a `Polars::Series`, that can be built using `Series::from_iter(vec![<vals>])`. In this example, `industry` is subset to Manufacturing (2), Construction (4), Transport and communication (6) or Public administration, education and health (8).

```Rust
=== Rust 3_1_1_filter block_6
```

## Lazy evaluation optimization

Filtering is a perfect example to show how `LazyFrame` use optimized queries, especially when using partitioned parquet files, as created in the [Parquet](../2_data/3_parquet.md#writing) chapter. This example can be run with `cargo run -r --example 3_1_2_filter_opt` (release mode is important for simple benchmarking).

> [!NOTE]
> Optimization also works when connecting to data on the Cloud.

First, lets enable verbose output to understand what Polars is doing.

```Rust
=== Rust 3_1_2_filter_opt block_1
```

> [!NOTE]
> Note that this code is wrapped in Rust's `unsafe {}`. This is not uncommon in Rust, as it's a way to explicitly force the user to acknowledge that the code they are using could potentially cause memory issues or not be thread-safe. As explained in the documentation of [set_var](https://doc.rust-lang.org/std/env/fn.set_var.html#safety), "This function is safe to call in a single-threaded program." and "This function is also always safe to call on Windows, in single-threaded and multi-threaded programs.". Therefore, this function is only unsafe in multi-treaded programs, on Linux or MacOS. This program is single threaded, so no concerns here! I recommend exploring the [unsafe rust](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html) documentation a bit to familiarize yourself with safety in Rust. 

Next lets connect to the `./data/large/census.parquet` file that contains over 60 million rows of Census data, in one extremely compressed parquet file (approximately 13 MB). Lets filter this file to the region of london (region = "E12000007"), for those aged 45 to 54 (age_group = 5), and non-null values for `income`. Remember, this code creates and execution plan, but does not yet execute it.

```Rust
=== Rust 3_1_2_filter_opt block_2
```

Second, lets connect to the `./data/large/partitioned` partitioned dataset, that was partitioned by `region` and by `age_group`. Overall, the files in this partitioned dataset folder will contain over 60 million rows of Census data. This data is split among 70 extremely compressed parquet files, equalling a total of approximately 40 MB. Similar to the large parquet file, this file will be filtered to the region of London (region = "E12000007"), for those aged 45 to 54 (age_group = 5), and non-null values for `income`. Again, nothing is executed at this point. 

```Rust
=== Rust 3_1_2_filter_opt block_3
```

Now, lets collect both files into memory and get info about execution time:

```Rust
=== Rust 3_1_2_filter_opt block_4
```

This is when the lazy query is executed and, since verbose was enabled, we can understand what steps are being taken.

For the verbose output of the `./data/large/census.parquet` file you will see that there is 1 source, and the entirety of the source needs to be read:

```
[MultiScanTaskInit]: 1 sources, reader name: parquet, ReaderCapabilities(ROW_INDEX | PRE_SLICE | NEGATIVE_PRE_SLICE | PARTIAL_FILTER | FULL_FILTER | MAPPED_COLUMN_PROJECTION), n_readers_pre_init: 1, max_concurrent_scans: 1
[MultiScanTaskInit]: predicate: Some("<predicate>"), skip files mask: None, predicate to reader: Some("<predicate>")
```

For the verbose output of the `./data/large/partitioned` files you will see that while there are 70 files, 69 of them can be skipped before being read:

```
[MultiScanTaskInit]: 70 sources, reader name: parquet, ReaderCapabilities(ROW_INDEX | PRE_SLICE | NEGATIVE_PRE_SLICE | PARTIAL_FILTER | FULL_FILTER | MAPPED_COLUMN_PROJECTION), n_readers_pre_init: 19, max_concurrent_scans: 16
[MultiScan]: Predicate pushdown allows skipping 69 / 70 files
```

For the large file, the entirety of the file has to be scanned (e.g. each row has to be verified for all filters) but for the partitioned parquet file, only one file is scanned, and the filter is applied only to the rows in that one smaller file. The partitioned parquet file allows for filters that are in the partitioned columns (e.g. `region` and `age_group`) to skip entire files.

This gives really great time improvements for queries that contain filters for those variables. In the above example, collecting the data from both files can show some significant time differences. While the time differs, you can see an improvment of between 2x and 5x the speed. For extremly large queries (billions of rows) this can have massive advantages.
