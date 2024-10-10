# Useful functions


### Size of pandas dataframe

```rust
use human_bytes::human_bytes;

let size = human_bytes(df.collect().unwrap().estimated_size() as f64);

println!("", size);
```

### Rename to lower

#### Greedy

```rust
fn rename_tolower(df: &mut DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    let lower_cols: Vec<String> = df
        .get_column_names()
        .iter()
        .map(|c| c.to_owned().to_lowercase())
        .collect();

    df.set_column_names(lower_cols)?;

    Ok(())
}

rename_tolower(&mut df)?;
```

#### Lazy

```Rust
fn rename_tolower_lazy(mut df: LazyFrame) -> Result<LazyFrame, Box<dyn std::error::Error>> {
    let cols: Vec<String> = df
        .collect_schema()
        .unwrap()
        .iter_names()
        .map(|c| c.to_owned().to_string())
        .collect();

    let lower_cols: Vec<String> = df
        .collect_schema()
        .unwrap()
        .iter_names()
        .map(|c| c.to_owned().to_lowercase())
        .collect();

    Ok(df.rename(cols.iter(), lower_cols.iter()))
}

df = rename_tolower_lazy(df)?;
```