# Reading

Reading data using Polars can be done either by bringing all the data into memory (called `eager evaluation`) as a `DataFrame` or by using a `LazyFrame` and building a logical plan that delays execution to when the output is requested (called `lazy evaluation`). The `DataFrame` has more available options for analysis, but the `LazyFrame` can evaluate data that is larger than memory and can optimize the logical plan prior to executing, making it more efficient if possible.

## CSV

You can read `.csv` files using Polars with the `CSVReadOptions`, for eager evaluation, or the `LazyCsvReader`, for lazy evaluation.

### Eager evaluation

```rust
let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some("./data/lfs_csv/pub0824.csv".into()))
    .unwrap()
    .finish()
    .unwrap();
```

You can use `estimate_size()` to estimate the size in memory of the `DataFrame`. As you can see, this eager evaluation brings the whole `.csv` in memory.

```rust
println!("{}", human_bytes(df.estimated_size() as f64));
```

```
51.1 MiB
```

You can bring in quite large data, depending on your available memory (see [writing](writing.md) for the source of this `.csv` file).

```rust
let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some("./data/large/all_years.csv".into()))
    .unwrap()
    .finish()
    .unwrap();
```

If you run out of RAM, you will get a `` error.


### Lazy evaluation

```rust
let df = LazyCsvReader::new("./data/pub0824.csv")
    .with_has_header(true)
    .finish()?;
```

