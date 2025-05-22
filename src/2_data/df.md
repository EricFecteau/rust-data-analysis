# DataFrame

Creating a DataFrame from data stored in code is generally not very useful. Data is more frequently stored in csv files, parquet files or databases. It is still very useful to know how to create these small DataFrames for sharing reproducible examples, showcasing a feature or identifying a bug. 

To create a DataFrame, you first have to create `Columns`. You can create columns with `Column::new()` and passing a name and an vector of values. Here we create the "year" (i32), "month" (str) and value (f64) variables.

Run this script using `cargo run -r --example 2_1_1_dataframe`.

```Rust
:dep polars = "0.48"

use polars::prelude::*;

// Creating columns
let year = Column::new("year".into(), [2010, 2010, 2011, 2011, 2011, 2011]);
let month = Column::new(
    "month".into(),
    [
        "November", "December", "January", "February", "March", "April",
    ],
);
let value = Column::new("value".into(), [1.25, 2.50, 3.75, 4.00, 3.75, 4.25]);
```

Once you have created columns of the same length, you can create a DataFrame using `DataFrame::new()`.

```Rust
// Using columns to create a DataFrame
let df = DataFrame::new(vec![year, month, value]).unwrap();
```

This creates a `DataFrame` with 3 columns and 6 rows:

```
shape: (6, 3)
┌──────┬──────────┬───────┐
│ year ┆ month    ┆ value │
│ ---  ┆ ---      ┆ ---   │
│ i32  ┆ str      ┆ f64   │
╞══════╪══════════╪═══════╡
│ 2010 ┆ November ┆ 1.25  │
│ 2010 ┆ December ┆ 2.5   │
│ 2011 ┆ January  ┆ 3.75  │
│ 2011 ┆ February ┆ 4.0   │
│ 2011 ┆ March    ┆ 3.75  │
│ 2011 ┆ April    ┆ 4.25  │
└──────┴──────────┴───────┘
```

The `df!()` macro can simplify this! Instead of creating a DataFrame from `Columns`, you can do it all in one step. Here we create the "year" (i32), "month" (str) and value (f64) variables.

```Rust
// Use the df! macro to create DataFrame
let df = df!("year" => [2008, 2008, 2008, 2008, 2009, 2009],
             "month" => ["September", "October", "November", "December", "January", "February"],
             "value" => [0.21, 0.22, 0.23, 0.25, 0.24, 0.25])
.unwrap();
```

Similar to above, this creates a `DataFrame` with 3 columns and 6 rows:

```
shape: (6, 3)
┌──────┬───────────┬───────┐
│ year ┆ month     ┆ value │
│ ---  ┆ ---       ┆ ---   │
│ i32  ┆ str       ┆ f64   │
╞══════╪═══════════╪═══════╡
│ 2008 ┆ September ┆ 0.21  │
│ 2008 ┆ October   ┆ 0.22  │
│ 2008 ┆ November  ┆ 0.23  │
│ 2008 ┆ December  ┆ 0.25  │
│ 2009 ┆ January   ┆ 0.24  │
│ 2009 ┆ February  ┆ 0.25  │
└──────┴───────────┴───────┘
```

