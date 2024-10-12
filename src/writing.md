# Reading


## Lazy

## CSV

### Greedy

```rust
let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some("./data/pub0824.csv".into()))
    .unwrap()
    .finish()
    .unwrap();

// Print df
println!("{}", df);
```

### Lazy

```rust
let df = LazyCsvReader::new("./data/pub0824.csv")
    .with_has_header(true)
    .finish()?;

// Collect and print df
println!("{}", df.collect().unwrap());
```