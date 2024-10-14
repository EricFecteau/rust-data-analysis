# Writing



## CVS

Assuming you have a mutable `DataFrame` in memory called `df`, you can write it to `.csv`:

```Rust
let mut file = std::fs::File::create("./data/df.csv").unwrap();
CsvWriter::new(&mut file).finish(&mut df).unwrap();
```

With this knowledge (and some knowledge from the [reading section](reading.md)), it is now possible to make a larger file using the monthly LFS data. This script creates a ~2.1 GB CSV, but requires ~16 GB of RAM (or more) to process. You can reduce the "current_year" to get a smaller file.

```Rust
let current_year = 2024;
let current_month = 9;

// Load first CSV
let mut df = CsvReadOptions::default()
    .with_infer_schema_length(Some(10_000)) // Default 100, missing = String
    .try_into_reader_with_file_path(Some("./data/lfs_csv/pub0106.csv".into()))
    .unwrap()
    .finish()
    .unwrap();

// For all years
for y in 2006..current_year + 1 {
    // For all months in each year
    for m in 1..12 + 1 {
        // skip the first one, since already loaded
        if y == 2006 && m == 1 {
            continue;
        }

        // Format name for the CSV file
        let csv_file = format!("./data/lfs_csv/pub{:02}{:02}.csv", m, y % 100);

        // stack new csv files on top if the first one
        df.vstack_mut(
            &CsvReadOptions::default()
                .with_infer_schema_length(Some(10_000)) // Default 100, missing = String
                .try_into_reader_with_file_path(Some(csv_file.clone().into()))
                .unwrap()
                .finish()
                .unwrap(),
        )?;

        // Print to keep track + size
        println!(
            "File: {} - Size: {}",
            csv_file,
            human_bytes(df.estimated_size() as f64)
        );

        // Recommended in the docs
        df.align_chunks();

        if y == current_year && m == current_month {
            break;
        }
    }
}

// Write large file as `lfs_large.csv`
let mut file = std::fs::File::create("./data/lfs_large.csv").unwrap();
CsvWriter::new(&mut file).finish(&mut df).unwrap();
```

## Parquet

```Rust
let mut df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("./data/lfs_csv/pub0824.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    let stats = StatisticsOptions {
        min_value: true,
        max_value: true,
        distinct_count: true,
        null_count: true,
    };

    let write_options = ParquetWriteOptions {
        compression: ParquetCompression::Zstd(Some(ZstdLevel::try_new(10)?)),
        statistics: stats,
        row_group_size: Some(512_usize.pow(2)),
        data_page_size: Some(1024_usize.pow(2)),
        maintain_order: true,
    };

    // This functionality is unstable according to the docs
    write_partitioned_dataset(
        &mut df,
        Path::new("./data/part/"),
        vec!["PROV", "SEX"],
        &write_options,
        4294967296,
    )?;

    Ok(())
```