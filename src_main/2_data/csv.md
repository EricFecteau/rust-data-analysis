# CSV

You can read and write from CSVs using Polars.

## Reading

You can connect to a CSV file, like the Jan 2024 LFS file `./data/lfs_csv/pub0124.csv`, without bringing it in memory, with the `LazyCsvReader`. You can run this section using `cargo run -r --example 2_2_2_read_csv`.

```rust
:dep polars = { version = "0.49", features = ["lazy"] }

use polars::prelude::*;

let lf = LazyCsvReader::new("./data/lfs_large/lfs.csv")
    .with_has_header(true)
    .finish()
    .unwrap();
```

None of the data is brought into memory. You can't even visualize any of it, since `LazyFrame` does not implement `display`. To display it, you can subset it to a few rows and then convert it to a `DataFrame` for printing: 

```Rust
println!("{}", lf.limit(5).collect().unwrap());
```

```
shape: (5, 60)
┌─────────┬──────────┬──────────┬─────────┬───┬─────────┬──────────┬─────────┬─────────┐
│ rec_num ┆ survyear ┆ survmnth ┆ lfsstat ┆ … ┆ schooln ┆ efamtype ┆ agyownk ┆ finalwt │
│ ---     ┆ ---      ┆ ---      ┆ ---     ┆   ┆ ---     ┆ ---      ┆ ---     ┆ ---     │
│ i64     ┆ i64      ┆ i64      ┆ i64     ┆   ┆ i64     ┆ i64      ┆ i64     ┆ i64     │
╞═════════╪══════════╪══════════╪═════════╪═══╪═════════╪══════════╪═════════╪═════════╡
│ 1       ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 3        ┆ 1       ┆ 403     │
│ 2       ┆ 2024     ┆ 1        ┆ 4       ┆ … ┆ null    ┆ 11       ┆ null    ┆ 140     │
│ 3       ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 1        ┆ null    ┆ 378     │
│ 4       ┆ 2024     ┆ 1        ┆ 4       ┆ … ┆ 1       ┆ 18       ┆ null    ┆ 222     │
│ 5       ┆ 2024     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 4        ┆ null    ┆ 34      │
└─────────┴──────────┴──────────┴─────────┴───┴─────────┴──────────┴─────────┴─────────┘
```

## Writing

You can write to CSV any `DataFrame` you have in memory. For this example, we will bring one month of the LFS into memory:

```Rust
:dep polars = { version = "0.49", features = ["lazy"] }

use polars::prelude::*;

// Read `pub0124.csv` as LazyFrame
let lf = LazyCsvReader::new("./data/lfs_csv/pub0124.csv")
    .with_has_header(true)
    .finish()
    .unwrap();

// Bring it into memory (by converting it to DataFrame)
let mut df = lf.collect().unwrap();
```

In order to save it, you have to create a file and write to it:

```Rust
// Write `pub0124.csv`
let mut file = std::fs::File::create("./data/temp_data/pub0124.csv").unwrap();
CsvWriter::new(&mut file).finish(&mut df).unwrap();
```