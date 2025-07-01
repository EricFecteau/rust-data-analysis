# Joins

This section explore how to join two datasets, either by stacking them one on top of the other (same columns) or by stacking them side by side (same rows).

## Setup

Lets first create 4 small datasets

```Rust
// Connect to parquet
let args = ScanArgsParquet::default();
let lf_jan =
    LazyFrame::scan_parquet("./data/lfs_parquet/pub0123.parquet", args.clone()).unwrap();
let lf_feb =
    LazyFrame::scan_parquet("./data/lfs_parquet/pub0223.parquet", args.clone()).unwrap();
let lf_mar =
    LazyFrame::scan_parquet("./data/lfs_parquet/pub0323.parquet", args.clone()).unwrap();
let lf_apr =
    LazyFrame::scan_parquet("./data/lfs_parquet/pub0423.parquet", args.clone()).unwrap();
```