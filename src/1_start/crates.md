# Crates

Throughout this book, various crates are going to be used in the examples. Here are the creates, their versions at the time of writing and and the features that will be used.

You will need to add these to your [Cargo.toml](https://github.com/EricFecteau/rust-data-analysis/blob/main/Cargo.toml) file, when relevant.

```toml
[dependencies]

# Download files from the internet (blocking)
reqwest = { version = "0.12", features = ["blocking"] }

# Extract ZIP files
zip = "2"

# Polars - open-source library for data manipulation, with `lazy` feature
#          enabled to support lazy evaluation 
polars = { version = "0.48", features = ["lazy"] }
polars_arrow = "0.48"
```

## Polars


## 