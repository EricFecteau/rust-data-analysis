# Crates

Throughout this book, various crates are going to be used in the examples. Here are the creates, their versions at the time of writing and and the features that will be used. 




You will need to add these to your `Cargo.toml` file, when relevant.


```toml
[dependencies]

# Polars - open-source library for data manipulation, with `lazy` feature
#          enabled to support lazy evaluation 
polars = { version = "0.43", features = ["lazy"] }

# human_bytes - converts large byte numbers into a human-readable format
human_bytes = "0.4"
```