# Select

This chapter will explore how to select columns from your data. 

With Polars, you can select a few columns using `select()`. You can run this code with `cargo run --example 3_2_1_select`.

```Rust
// Connect to LazyFrame (no data is brought into memory)
let args = ScanArgsParquet::default();
let mut lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();
```

You can get a vector of all the variables in the connected data with: 

```Rust
// Get names of columns
let cols: Vec<String> = lf
    .collect_schema()
    .unwrap()
    .iter_names()
    .map(|c| c.to_owned().to_string())
    .collect();

println!(
    "Vector of the {} variables in the LazyFrame: {:?}",
    cols.len(),
    cols
);
```

```
Vector of the 60 variables in the LazyFrame: ["rec_num", "survyear", "survmnth", "lfsstat", "prov", "cma", "age_12", "age_6", "gender", "marstat", "educ", "mjh", "everwork", "ftptlast", "cowmain", "immig", "naics_21", "noc_10", "noc_43", "yabsent", "wksaway", "payaway", "uhrsmain", "ahrsmain", "ftptmain", "utothrs", "atothrs", "hrsaway", "yaway", "paidot", "unpaidot", "xtrahrs", "whypt", "tenure", "prevten", "hrlyearn", "union", "permtemp", "estsize", "firmsize", "durunemp", "flowunem", "unemftpt", "whylefto", "whyleftn", "durjless", "availabl", "lkpubag", "lkemploy", "lkrels", "lkatads", "lkansads", "lkothern", "prioract", "ynolook", "tlolook", "schooln", "efamtype", "agyownk", "finalwt"]
```

Using `select()` you can select various columns using the `col()` function. With the `regex` Polars crate feature, you can also use regular expressions to identify columns following a pattern. This pattern must start with `^` and end with `$`. In this example, we are keeping `survyear`, `survmnth`, `prov`, `hrlyearn` and `finalwt`. With `alias` we are renaming `hrlyearn` to `hourly_wages`.

```Rust
// Select some columns by name & with regex & with rename
let lf = lf.select([
    col("^surv.*$"), // survyear, survmnth
    col("prov"),
    col("hrlyearn").alias("hourly_wages"),
    col("finalwt"),
]);

// Print selected column (top 5 values)
println!("{}", lf.clone().limit(5).collect().unwrap());
```

```
shape: (5, 5)
┌──────────┬──────────┬──────┬──────────────┬─────────┐
│ survyear ┆ survmnth ┆ prov ┆ hourly_wages ┆ finalwt │
│ ---      ┆ ---      ┆ ---  ┆ ---          ┆ ---     │
│ i64      ┆ i64      ┆ i64  ┆ i64          ┆ i64     │
╞══════════╪══════════╪══════╪══════════════╪═════════╡
│ 2011     ┆ 1        ┆ 59   ┆ null         ┆ 109     │
│ 2011     ┆ 1        ┆ 48   ┆ null         ┆ 62      │
│ 2011     ┆ 1        ┆ 47   ┆ null         ┆ 71      │
│ 2011     ┆ 1        ┆ 35   ┆ null         ┆ 345     │
│ 2011     ┆ 1        ┆ 12   ┆ 2462         ┆ 105     │
└──────────┴──────────┴──────┴──────────────┴─────────┘
```

You can also drop variables with `drop()`:

```Rust
// Drop variables
let lf = lf.drop([col("prov"), col("hourly_wages")]);

// Print selected column (top 5 values)
println!("{}", lf.clone().limit(5).collect().unwrap());
```

```
shape: (5, 3)
┌──────────┬──────────┬─────────┐
│ survyear ┆ survmnth ┆ finalwt │
│ ---      ┆ ---      ┆ ---     │
│ i64      ┆ i64      ┆ i64     │
╞══════════╪══════════╪═════════╡
│ 2011     ┆ 1        ┆ 109     │
│ 2011     ┆ 1        ┆ 62      │
│ 2011     ┆ 1        ┆ 71      │
│ 2011     ┆ 1        ┆ 345     │
│ 2011     ┆ 1        ┆ 105     │
└──────────┴──────────┴─────────┘
```

The `drop()` should be used sparingly by letting your query optimization (e.g. summary of data on requested variables only) do the work for you.