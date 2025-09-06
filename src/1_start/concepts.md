# Concepts

This page goes over a few concepts that will significantly help understand how to do data analysis in Rust and to understand the rest of this book.

## Lazy vs Eager Polars

Like with [arrow](https://arrow.apache.org/), [duckdb](https://duckdb.org/), [dbplyr](https://dbplyr.tidyverse.org/) or other modern data analysis tools, Polars has the ability delay execution until the data is needed through `lazy evaluation`. This has 3 advantages (according to the [Polars docs](https://docs.pola.rs/user-guide/lazy/using/)):

* the lazy API allows Polars to apply automatic query optimization with the query optimizer
* the lazy API allows you to work with larger than memory datasets using streaming
* the lazy API can catch schema errors before processing the data

Polars uses the `DataFrame` API when using `eager evaluation` (e.g. bringing the data into memory) and uses the `LazyFrame` API when using `lazy evaluation` (e.g. connecting to the data, but not bringing it into memory). The available functions for both APIs are similar, but sufficiently different that code written for one will not work on the other. Generally, as is recommended by [Polar's DataFrame](https://docs.rs/polars/latest/polars/) because of the advantages of lazy evaluation, this book will be giving examples of `lazy evaluation`. In examples, this book will use `lf` for the name of the object (or the `lf_` prefix) when the object is a `LazyFrame` and `df` (or the `df_` prefix) for `DataFrame`.

To convert a `DataFrame` to a `LazyFrame` you can use `let lf = df.lazy();`. Note that nothing will be evaluated yet. It will only be evaluation once you convert the `LazyFrame` to a `DataFrame` with `collect()`: `let df = lf.collect().unwrap();`.

> [!CAUTION]
> Use caution when using `collect()`, as it will bring the results in memory. Make sure that the data being collected is memory-sized (e.g. small data frame, summary data, only a few columns, etc.).

## Arrow crates

Arrow is a language-independent columnar memory format for flat and nested data, organized for efficient analytic operations on modern hardware. In Rust, there are three crates that can manipulate [Apache Arrow](https://arrow.apache.org/) data. This book generally uses `polars_arrow`, through `polars`. Since the data ecosystem in Rust has not standardized, it's useful to be aware of the other crates: 

* [arrow](https://docs.rs/arrow/latest/arrow/): A complete, safe, native Rust implementation of Apache Arrow, a cross-language development platform for in-memory data. This crate is the official implementation of Arrow in Rust, by Apache.
* [polars_arrow](https://docs.rs/polars-arrow/latest/polars_arrow/): An Arrow implementation by Polars. Polars uses the Apache Arrow memory model for their DataFrame.
* [arrow2](https://docs.rs/arrow2/latest/arrow2/): A previously popular Rust crate to work with Apache Arrow. This crate is still used by some older tools, but is no longer maintained and should be avoided.

See the [Polars and Arrow versions](#polars-and-arrow-versions) section for information on how to convert data from `arrow` to `polars_arrow`.

## Polars and Arrow versions

Since both Polars (pre-1.0) and Arrow have SemVer breaking API updates in Rust every few months, the Rust ecosystem that depend on these crates update at a different rates and are consistently incompatible with each other (e.g. one crate outputs Polars 0.45 and another crate takes Polars 0.43 as input). For crates who take these as input or provide these as output, updating should be considered an API break, and require a major bump in version. This has a cascading effect over the whole ecosystem.

For example, attempting to pass Polars 0.45 to a crate that uses Polars 0.43, or vice versa, will give a error[E0308]: mismatched types error with the note “‘DataFrame’ and ‘DataFrame’ have similar names, but are actually distinct types”.

The [df_interchange](https://docs.rs/df-interchange/latest/df_interchange/) crate solves the interoperability issue and prevent the need for the entirety of the ecosystem to update at the same speed. It solves this by allowing for seamless interoperability between any version of Polars (>=0.40) and any version of Arrow (>=54), including between versions of the same crate (e.g. Polars 0.40 to Polars 0.46), using the Arrow C Data Interchange format.

Various chapters in this book may use `df_interchange` depending on the current status and update cycle between crates. Generally, the text won't discuss in detail the use of `df_interchange` as it is likely a temporary fix to lagging crates.

If you encounter incompatibility between version of Arrow or version of Polars, you can use `df_interchange`:

```yaml
[dependencies]
polars = "0.43"
arrow = "54"
df-interchange = { version = "0.2", features = ["polars_0_43", "polars_0_46", "arrow_54"] }
```

```Rust
use std::sync::Arc;
use arrow::{array::{ArrayRef, Int32Array, Int64Array}, record_batch::RecordBatch}; // Arrow 54
use polars::prelude::*; // Polars 0.43
use df_interchange::Interchange;

// Create Polars 0.43 data
let polars_0_43 = DataFrame::new(vec![
    Series::new("test_i32".into(), [-1i32, 0, 1]),
    Series::new("test_i64".into(), [-1i64, 0, 1]),
])
.unwrap();

// Create arrow 54 data
let arrow_54: Vec<_> = vec![RecordBatch::try_from_iter(vec![
    ("test_i32", Arc::new(Int32Array::from(vec![-1i32, 0, 1])) as ArrayRef),
    ("test_i64", Arc::new(Int64Array::from(vec![-1i64, 0, 1])) as ArrayRef),
])
.unwrap()];

// Convert Polars 0.43 to Polars 0.46
let df_polars = Interchange::from_polars_0_43(polars_0_43)?.to_polars_0_46()?;

// Convert Arrow 54 to Polars 0.46
let df_arrow = Interchange::from_arrow_54(arrow_54)?.to_polars_0_46()?;

// Compare the two DataFrames (not possible prior to conversion to Polars 0.46)
assert!(df_polars.equals_missing(&df_arrow));
```

## Rust error handling

Rust has [extensive abilities](https://doc.rust-lang.org/book/ch09-00-error-handling.html) to handle errors and recover from them, that this book completely ignores. When something may fail (e.g. the cloud connection string is incorrect or a variable is misspelled), instead of handling it, this book simply allows the program to crash (with `.unwrap()`). This is generally how most data analysts work when doing data analysis and data exploration: let the program crash if the code is incorrect. 

Once your code is "production ready", handling errors can be added for every `.unwrap()` (generally as simple as replacing `.unwrap()` with `?` and handling the returned error). Crates like [thiserror](https://docs.rs/thiserror/latest/thiserror/) and [anyhow](https://docs.rs/anyhow/latest/anyhow/) provide great tools to deal with this.