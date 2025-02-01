# Select & Filter

This section will explore how to filter rows and select columns. 

## Filter rows

You can filter the rows in the data using `filter()`. You can run this code with `cargo r -r --example 3_`.

First, lets load the partitioned parquet file:

```Rust
:dep polars = { version = "0.46", features = ["lazy", "parquet", "is_in"] }

use polars::prelude::*;

// Connect to LazyFrame (no data is brought into memory)
let args = ScanArgsParquet::default();
let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();
```

You can then filter using expressions. In this example, we have multiple filter filtering the data on the `survyear` (2010), `survmnth` (>6) and `hrlyearn` (not null):

```Rust
let lf_filtered = lf
    .filter(col("survyear").eq(lit(2010)))
    .filter(col("survmnth").gt(lit(6)))
    .filter(col("hrlyearn").is_not_null());
```

As you can see, to reference a column you have to use `col()` and to reference a literal you have to use `lit()`. You can compare these using equality comparison such as:

| Equality        | Definition                                               |
|-----------------|----------------------------------------------------------|
| `eq()`          | `==`                                                     |
| `eq_missing()`  | `==` & `None == None`                                    |
| `neq()`         | `!=`                                                     |
| `neq_missing()` | `!=` (including when one side of the equation is `None`) |
| `lt()`          | `<`                                                      |
| `gt()`          | `>`                                                      |
| `gt_eq()`       | `>=`                                                     |
| `lt_eq()`       | `<=`                                                     |
| `not()`         | `!`                                                      |
| `is_null()`     | `== None`                                                |
| `is_not_null()` | `!= None`                                                |

Instead of the multiple `.filter()` you can use one `.filter()` and chain the commands with `.and()`, `.or()` and `.xor()`. The above example can be created using this filter:

```Rust
// Filtering the data in one step
let lf_filt = lf.clone().filter(
    col("survyear")
        .eq(lit(2010))
        .and(col("survmnth").gt(lit(6)))
        .and(col("hrlyearn").is_not_null()),
);
```

This is especially important when crafting more complex filters. For example, you can craft this filter to collect the second half of 2010 and first half of 2011: 

```Rust 
// ((survyear == 2010 & survmnt > 6) | (survyear == 2011 & survmnt <= 6))
let expr = (col("survyear")
    .eq(lit(2010))
    .and(col("survmnth").gt(lit(6))))
.or(col("survyear")
    .eq(lit(2011))
    .and(col("survmnth").lt_eq(lit(6))));
```

You can print the expression to see how it's evaluated. This is especially useful when you use an IDE that can highlight bracket and parenthesis pairs.

```
[([([(col("survyear")) == (dyn int: 2010)]) & ([(col("survmnth")) > (dyn int: 6)])]) | ([([(col("survyear")) == (dyn int: 2011)]) & ([(col("survmnth")) <= (dyn int: 6)])])]
```

You can then apply the expression with `.filter()`:

```Rust
// Apply the expression to a LazyFrame
let lf_filt = lf.clone().filter(expr);
```
With the `is_in` crate feature, you can see if a `col()` is within a list of `lit()`. The right side of the expression takes a `Polars::Series`, that can be built using `Series::from_iter(vec![<vals>])`. In this example, we see if `survyear` is equal to 2009, 2010 or 2011.

```Rust
// Using `is_in` crate feature with literals
let lf_filt = lf
    .clone()
    .filter(col("survyear").is_in(lit(Series::from_iter(vec![2009, 2010, 2011]))));
```

### Lazy evaluation optimization

Filtering is a perfect example to show how `LazyFrame` use optimized queries, especially when using partitioned parquet files, as created in the [Writing](../rw/writing.md) section. 

> ![NOTE]
> This also works when connecting to data on the Cloud.

First, lets connect to the `./data/lfs_large/lfs.parquet` file that contains nearly 20 years of monthly LFS data, 23 million rows, in one parquet file (approximately 400 MB), and filter it to the records in the second half of 2010, and non-null values for `hrlyearn` (hourly wages). Remember, this code creates and execution plan, but does not yet execute it.

```Rust
// Connect to LazyFrame (one large parquet file)
let args = ScanArgsParquet::default();
let lf_one = LazyFrame::scan_parquet("./data/lfs_large/lfs.parquet", args).unwrap();

// Filter it
let lf_one = lf_one
    .filter(col("survyear").eq(lit(2010)))
    .filter(col("survmnth").gt(lit(6)))
    .filter(col("hrlyearn").is_not_null());
```

Second, lets connect to the `./data/lfs_large/part` partitioned dataset, that was partitioned by `survyear` and by `survmnth`. All the files in this partitioned dataset folder will contain nearly 20 years of monthly LFS data, 23 million rows, and over 200 parquet files, equalling a total of approximately 400 MB. Similar to the large parquet file, we will filter it to the records in the second half of 2010, and non-null values for `hrlyearn`. Again, nothing is executed at this point. 

```Rust
// Connect to LazyFrame (partitioned parquet file)
let args = ScanArgsParquet::default();
let lf_part = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

// Filter it
let lf_part = lf_part
    .filter(col("survyear").eq(lit(2010)))
    .filter(col("survmnth").gt(lit(6)))
    .filter(col("hrlyearn").is_not_null());
```

With `LazyFrame`, you can see the execution plan with `.explain()`. Passing `false` gives the unoptimized plan and passing `true` gives the optimized plan. When the plan is executed, it always uses the optimized plan. We can see that the unoptimized execution plan for the single parquet file and partitioned parquet file are similar:

```Rust
println!(lf_one.explain(false).unwrap());
```

```
FILTER col("hrlyearn").is_not_null() FROM
  FILTER [(col("survmnth")) > (6)] FROM
    FILTER [(col("survyear")) == (2010)] FROM
      Parquet SCAN [./data/lfs_large/lfs.parquet]
      PROJECT */60 COLUMNS
```


```Rust
println!(lf_part.explain(false).unwrap());
```

```
FILTER col("hrlyearn").is_not_null() FROM
  FILTER [(col("survmnth")) > (6)] FROM
    FILTER [(col("survyear")) == (2010)] FROM
      Parquet SCAN [./data/lfs_large/part/survyear=2006/survmnth=1/00000000.parquet, ... 224 other sources]
      PROJECT 58/60 COLUMNS
```

In both cases, the filters are the same, and the SCAN in both cases touches all files (the large one or all 225 parquet file for the partitioned parquet file). The single file collects all 60 variables and the partitioned one selects 58 of the 60 variables, likely because `survyear` and `survmnth` are known by it's folder structure.

On the other hand, the optimized query is quite different:

```Rust
println!(lf_one.explain(true).unwrap());
```

```
Parquet SCAN [./data/lfs_large/lfs.parquet]
PROJECT */60 COLUMNS
SELECTION: [([([(col("survmnth")) > (6)]) & ([(col("survyear")) == (2010)])]) & (col("hrlyearn").is_not_null())]
```


```Rust
println!(lf_part.explain(true).unwrap());
```

```
Parquet SCAN [./data/lfs_large/part/survyear=2010/survmnth=10/00000000.parquet, ... 5 other sources]
PROJECT 58/60 COLUMNS
SELECTION: [([(col("hrlyearn").is_not_null()) & ([(col("survmnth")) > (6)])]) & ([(col("survyear")) == (2010)])]
```

As you can see, the "selection" (filter) is essentially the same in both, but for the large file, the entirety of the file has to be scanned (e.g. each row has to be verified for all filters) but for the partitioned parquet file, only 6 files are scanned, and the filter is applied only to the rows in those 6 files. The partitioned parquet file allows for filters that are in the partitioned columns (e.g. `survyear` and `survmnth`) to skip entire files.

This gives really great time improvements for queries that contain filters for those variables. For example, 

```Rust
let before = Instant::now();
let _ = lf_one.select([col("hrlyearn")]).mean().collect().unwrap();
println!("One parquet file elapsed time: {:.2?}", before.elapsed());

let before = Instant::now();
let _ = lf_part.select([col("hrlyearn")]).mean().collect().unwrap();
println!("Partitioned parquet file elapsed time: {:.2?}", before.elapsed());
```

While the time differs, the one parquet file (between `25 ms` and `50 ms`) is about an order of magnitude slower than the partitioned parquet file (between `2 ms` and `4 ms`).

## Select columns

With Polars, you can select a few columns using `select()`.

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
Vector of the 60 variables in the LazyFrame: ["rec_num", "survyear", "survmnth", "lfsstat", "prov", "cma", "age_12", "age_6", "sex", "marstat", "educ", "mjh", "everwork", "ftptlast", "cowmain", "immig", "naics_21", "noc_10", "noc_43", "yabsent", "wksaway", "payaway", "uhrsmain", "ahrsmain", "ftptmain", "utothrs", "atothrs", "hrsaway", "yaway", "paidot", "unpaidot", "xtrahrs", "whypt", "tenure", "prevten", "hrlyearn", "union", "permtemp", "estsize", "firmsize", "durunemp", "flowunem", "unemftpt", "whylefto", "whyleftn", "durjless", "availabl", "lkpubag", "lkemploy", "lkrels", "lkatads", "lkansads", "lkothern", "prioract", "ynolook", "tlolook", "schooln", "efamtype", "agyownk", "finalwt"]
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
│ 2006     ┆ 1        ┆ 24   ┆ null         ┆ 119     │
│ 2006     ┆ 1        ┆ 24   ┆ null         ┆ 94      │
│ 2006     ┆ 1        ┆ 13   ┆ null         ┆ 121     │
│ 2006     ┆ 1        ┆ 46   ┆ null         ┆ 154     │
│ 2006     ┆ 1        ┆ 48   ┆ 1300         ┆ 489     │
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
│ 2006     ┆ 1        ┆ 119     │
│ 2006     ┆ 1        ┆ 94      │
│ 2006     ┆ 1        ┆ 121     │
│ 2006     ┆ 1        ┆ 154     │
│ 2006     ┆ 1        ┆ 489     │
└──────────┴──────────┴─────────┘
```

/// MIGHT NOT BE NECESSARY TO SELECT AND DROP -- let optimization figure it out for you according to your last table