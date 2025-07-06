# Excel

In the data analysis world, Excel is still the universal aggregate statistics exchange format and quick analysis tool. It is much simpler to send a fellow researcher an Excel file with some summary statistics or a few hundred rows of data than it is to send pretty much any other data format. With the 

## Setup

First, lets create some summary statistics to throw into the excel file. We will create a table of mean oft he hearly earnings by year and province, in a long format (e.g. 3 columns: "survyear", "prov" and "hourly_wages") and wide format (e.g. "survyear" as row and "prov" as column). Both `df_long` and `df_wide` are brought into memory as data frames.

```Rust
// Connect to LazyFrame
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

// Modify var
let lf: LazyFrame = lf
    .filter(col("hrlyearn").is_not_null())
    .with_column((col("hrlyearn").cast(DataType::Float64) / lit(100)).alias("hourly_wages"));

// Mean by year and province
let df_long = lf
    .clone()
    .group_by([col("survyear"), col("prov")])
    .agg([col("hourly_wages")
        .mean()
        .round(2, RoundMode::HalfAwayFromZero)])
    .sort(["survyear", "prov"], Default::default())
    .with_column(col("prov").replace_strict(
        lit(Series::from_iter(vec![
            "10", "11", "12", "13", "24", "35", "46", "47", "48", "59",
        ])),
        lit(Series::from_iter(vec![
            "NL", "PE", "NS", "NB", "QC", "ON", "MB", "SK", "AB", "BC",
        ])),
        None,
        Some(DataType::String),
    ))
    .collect()
    .unwrap();

// Pivot by province
let df_wide = pivot_stable(
    &df_long,
    ["prov"],
    Some(["survyear"]),
    Some(["hourly_wages"]),
    false,
    None,
    None,
)
.unwrap();

println!("Long:\n{df_long}");

println!("Wide:\n{df_wide}");
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

```Rust
// Create a new Excel writer.
let mut excel_writer = PolarsExcelWriter::new();

// Create workbook
let mut workbook = Workbook::new();

// Write long table to "long" worksheet
let ws_long = workbook.add_worksheet().set_name("long").unwrap();
excel_writer
    .write_dataframe_to_worksheet(&df_long, ws_long, 0, 0)
    .unwrap();
```


![Long Excel data](images/excel/long.png)

Next, we can add a second worksheet, called "wide" with the wide data from `df_wide`:

```Rust
// Write wide table to "wide" worksheet
let ws_wide = workbook.add_worksheet().set_name("wide").unwrap();
excel_writer
    .write_dataframe_to_worksheet(&df_wide, ws_wide, 0, 0)
    .unwrap();
```

![Wide Excel data](images/excel/wide.png)

Now that we have the data into excel, we can use `rust_xlsxwriter` to manipulate the worksheet and add anything. Here, we add a line chart based on the data from `df_wide`, found in the "wide" worksheet:

```Rust
// Get some info to limit the size and shape of the graph (e.g. rows, min/max values)
let row_num = df_wide.shape().0;
let col_num = df_wide.shape().1;
let min_val: f64 = df_long
    .column("hourly_wages")
    .unwrap()
    .as_series()
    .unwrap()
    .min()
    .unwrap()
    .unwrap();
let max_val: f64 = df_long
    .column("hourly_wages")
    .unwrap()
    .as_series()
    .unwrap()
    .max()
    .unwrap()
    .unwrap();

// Iterate throw rows to create multiple lines
let mut chart = Chart::new(ChartType::Line);
for i in 1..col_num {
    chart
        .add_series()
        .set_name(("wide", 0, i as u16)) // Name of province found in row 1, column i
        .set_categories(("wide", 1, 0, row_num as u32, 0)) // Category (year) found in column 1, row 1-year_count
        .set_values(("wide", 1, i as u16, row_num as u32, i as u16)); // FR, FC, LR, LC
}
chart.x_axis().set_name("Year");
chart
    .y_axis()
    .set_name("Hourly wages")
    .set_min((min_val - 1.0).round())
    .set_max((max_val + 1.0).round());
chart.legend().set_position(ChartLegendPosition::Bottom);
ws_wide.insert_chart(1, 12, &chart).unwrap();
```

![Excel graph based on wide data](images/excel/graph.png)

Lastly, we can save all of this to the `./data/output/mean_hourly_wages.xlsx` folder:

```Rust
// Save the file to disk.
workbook
    .save("./data/output/mean_hourly_wages.xlsx")
    .unwrap();
```