# Excel

In the data analysis world, Excel is still the universal aggregate statistics exchange format and quick analysis tool. It is much simpler to send a fellow researcher an Excel file with some summary statistics or a few hundred rows of data than it is to send pretty much any other data format. With the following code, you can export any data from Polars to Excel, format it as needed and even add plots.

## Setup

First, lets create some summary statistics to throw into the excel file. We will create a table of mean hourly earnings by year and province, in a long format (e.g. 3 columns: "survyear", "prov" and "hourly_wages") and wide format (e.g. "survyear" as row and "prov" as column). Both `df_long` and `df_wide` are brought into memory as data frames.

```rust
=== Rust 5_1_1_excel imports
=== Rust 5_1_1_excel block_1
```

Long:

```
shape: (140, 3)
┌──────────┬──────┬──────────────┐
│ survyear ┆ prov ┆ hourly_wages │
│ ---      ┆ ---  ┆ ---          │
│ i64      ┆ str  ┆ f64          │
╞══════════╪══════╪══════════════╡
│ 2011     ┆ NL   ┆ 22.31        │
│ 2011     ┆ PE   ┆ 19.6         │
│ 2011     ┆ NS   ┆ 20.79        │
│ 2011     ┆ NB   ┆ 19.96        │
│ 2011     ┆ QC   ┆ 21.89        │
│ …        ┆ …    ┆ …            │
│ 2024     ┆ ON   ┆ 36.64        │
│ 2024     ┆ MB   ┆ 30.49        │
│ 2024     ┆ SK   ┆ 32.88        │
│ 2024     ┆ AB   ┆ 37.06        │
│ 2024     ┆ BC   ┆ 36.65        │
└──────────┴──────┴──────────────┘
```

Wide:

```
shape: (14, 11)
┌──────────┬───────┬───────┬───────┬───┬───────┬───────┬───────┬───────┐
│ survyear ┆ NL    ┆ PE    ┆ NS    ┆ … ┆ MB    ┆ SK    ┆ AB    ┆ BC    │
│ ---      ┆ ---   ┆ ---   ┆ ---   ┆   ┆ ---   ┆ ---   ┆ ---   ┆ ---   │
│ i64      ┆ f64   ┆ f64   ┆ f64   ┆   ┆ f64   ┆ f64   ┆ f64   ┆ f64   │
╞══════════╪═══════╪═══════╪═══════╪═══╪═══════╪═══════╪═══════╪═══════╡
│ 2011     ┆ 22.31 ┆ 19.6  ┆ 20.79 ┆ … ┆ 21.07 ┆ 23.54 ┆ 26.05 ┆ 23.52 │
│ 2012     ┆ 23.85 ┆ 20.37 ┆ 21.37 ┆ … ┆ 21.55 ┆ 24.43 ┆ 27.44 ┆ 23.95 │
│ 2013     ┆ 24.64 ┆ 20.8  ┆ 22.08 ┆ … ┆ 21.97 ┆ 25.23 ┆ 28.45 ┆ 24.67 │
│ 2014     ┆ 25.32 ┆ 21.16 ┆ 22.73 ┆ … ┆ 22.58 ┆ 25.95 ┆ 28.78 ┆ 24.86 │
│ 2015     ┆ 24.85 ┆ 21.6  ┆ 22.7  ┆ … ┆ 23.35 ┆ 26.61 ┆ 30.29 ┆ 25.64 │
│ …        ┆ …     ┆ …     ┆ …     ┆ … ┆ …     ┆ …     ┆ …     ┆ …     │
│ 2020     ┆ 27.76 ┆ 24.8  ┆ 25.93 ┆ … ┆ 27.04 ┆ 29.79 ┆ 33.69 ┆ 30.29 │
│ 2021     ┆ 28.53 ┆ 25.65 ┆ 26.32 ┆ … ┆ 27.42 ┆ 30.12 ┆ 33.48 ┆ 31.33 │
│ 2022     ┆ 30.55 ┆ 27.45 ┆ 27.42 ┆ … ┆ 28.15 ┆ 30.97 ┆ 33.84 ┆ 32.74 │
│ 2023     ┆ 32.08 ┆ 28.18 ┆ 28.63 ┆ … ┆ 29.42 ┆ 32.06 ┆ 35.49 ┆ 35.05 │
│ 2024     ┆ 33.92 ┆ 29.73 ┆ 30.67 ┆ … ┆ 30.49 ┆ 32.88 ┆ 37.06 ┆ 36.65 │
└──────────┴───────┴───────┴───────┴───┴───────┴───────┴───────┴───────┘
```

## Excel

To write this data to Excel, we can use the [polars_excel_writer](https://docs.rs/polars_excel_writer/latest/polars_excel_writer/) crate to write Polars data from a `DataFrame` into Excel. This crate uses the [rust_xlsxwriter](https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/) crate for this, and we can use the other options in the `rust_xlsxwriter` crate to do anything you can do in excel. 

First, lets create an Excel workbook and write our `df_long` to the "long" worksheet. Note that nothing has been saved yet, but the screenshots are taken as if it had been saved. The workbook is currently in-memory and will be written at the end of this section.

```rust
=== Rust 5_1_1_excel block_2
```

![Long Excel data](images/excel/long.png)

Next, we can add a second worksheet, called "wide" with the wide data from `df_wide`:

```rust
=== Rust 5_1_1_excel block_3
```

![Wide Excel data](images/excel/wide.png)

Now that we have the data into excel, we can use `rust_xlsxwriter` to manipulate the worksheet and add anything. Here, we add a line chart based on the data from `df_wide`, found in the "wide" worksheet:

```rust
=== Rust 5_1_1_excel block_4
```

![Excel graph based on wide data](images/excel/graph.png)

Lastly, we can save all of this to the `./data/output/mean_hourly_wages.xlsx` folder:

```rust
=== Rust 5_1_1_excel block_5
```