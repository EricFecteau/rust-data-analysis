# Reading data

## Lazy

## CSV

### Eager

```rust
let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some("./data/pub0824.csv".into()))
    .unwrap()
    .finish()
    .unwrap();

// Print df
println!("{}", df);
```

### Lazy

```rust
let df = LazyCsvReader::new("./data/pub0824.csv")
    .with_has_header(true)
    .finish()?;

// Collect and print df
println!("{}", df.collect().unwrap());
```

```
┌─────────┬──────────┬──────────┬─────────┬───┬─────────┬──────────┬─────────┬─────────┐
│ REC_NUM ┆ SURVYEAR ┆ SURVMNTH ┆ LFSSTAT ┆ … ┆ SCHOOLN ┆ EFAMTYPE ┆ AGYOWNK ┆ FINALWT │
│ ---     ┆ ---      ┆ ---      ┆ ---     ┆   ┆ ---     ┆ ---      ┆ ---     ┆ ---     │
│ i64     ┆ i64      ┆ i64      ┆ i64     ┆   ┆ i64     ┆ i64      ┆ i64     ┆ i64     │
╞═════════╪══════════╪══════════╪═════════╪═══╪═════════╪══════════╪═════════╪═════════╡
│ 1       ┆ 2024     ┆ 8        ┆ 1       ┆ … ┆ 1       ┆ 3        ┆ 2       ┆ 33      │
│ 2       ┆ 2024     ┆ 8        ┆ 1       ┆ … ┆ 1       ┆ 6        ┆ 2       ┆ 171     │
│ 3       ┆ 2024     ┆ 8        ┆ 4       ┆ … ┆ null    ┆ 8        ┆ null    ┆ 181     │
│ 4       ┆ 2024     ┆ 8        ┆ 1       ┆ … ┆ 1       ┆ 6        ┆ 1       ┆ 312     │
│ 5       ┆ 2024     ┆ 8        ┆ 4       ┆ … ┆ 1       ┆ 11       ┆ null    ┆ 443     │
│ …       ┆ …        ┆ …        ┆ …       ┆ … ┆ …       ┆ …        ┆ …       ┆ …       │
│ 112139  ┆ 2024     ┆ 8        ┆ 4       ┆ … ┆ null    ┆ 11       ┆ null    ┆ 322     │
│ 112140  ┆ 2024     ┆ 8        ┆ 1       ┆ … ┆ 1       ┆ 1        ┆ null    ┆ 968     │
│ 112141  ┆ 2024     ┆ 8        ┆ 4       ┆ … ┆ null    ┆ 11       ┆ null    ┆ 183     │
│ 112142  ┆ 2024     ┆ 8        ┆ 1       ┆ … ┆ 1       ┆ 3        ┆ 1       ┆ 140     │
│ 112143  ┆ 2024     ┆ 8        ┆ 4       ┆ … ┆ null    ┆ 1        ┆ null    ┆ 269     │
└─────────┴──────────┴──────────┴─────────┴───┴─────────┴──────────┴─────────┴─────────┘
```