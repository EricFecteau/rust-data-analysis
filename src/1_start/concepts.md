# Concepts

This page goes over a few concepts that will significantly help understand how to do data analysis in Rust and to understand the rest of this book.

## Lazy vs Eager Polars

Like with [arrow](https://arrow.apache.org/), [duckdb](https://duckdb.org/), [dbplyr](https://dbplyr.tidyverse.org/) or other data analysis tools, Polars has the ability delay execution until the data is needed through `lazy evaluation`. This has 3 advantages (according to the [Polars docs](https://docs.pola.rs/user-guide/lazy/using/)):

* the lazy API allows Polars to apply automatic query optimization with the query optimizer
* the lazy API allows you to work with larger than memory datasets using streaming
* the lazy API can catch schema errors before processing the data

Polars uses the `DataFrame` API when using `eager evaluation` (e.g. bringing the data into memory) and uses the `LazyFrame` API when using `lazy evaluation` (e.g. connecting to the data, but not bringing it into memory). The available functions for both APIs are similar, but sufficiently different that code written for one will not work on the other. Generally, as is recommended by [Polar's DataFrame](https://docs.rs/polars/latest/polars/), because of the advantages of lazy evaluation, this book will be giving examples of `lazy evaluation`. In examples, this book will use `lf` for the name of the object (or the `lf_` prefix) when the object is a `LazyFrame` and `df` (or the `df_` prefix) for `DataFrame`.

To convert a `DataFrame` to a `LazyFrame` you can use `let lf = df.lazy();`. Note that nothing will be evaluated yet. It will only be evaluation once you convert the `LazyFrame` to a `DataFrame` with `collect()`: `let df = lf.collect().unwrap();`.

> [!CAUTION]
> Use caution when using `collect()`, as it will bring the results in memory. Make sure that the data being collected is memory-sized (e.g. small data frame, summary data, few columns, etc.).

## Arrow crates


## Polars versions

