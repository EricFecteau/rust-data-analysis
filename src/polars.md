# Polars

```rust
fn main() {
    use polars::prelude::*;

    let df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("./data/path.csv".into()))
        .unwrap()
        .finish()
        .unwrap();
}
```
