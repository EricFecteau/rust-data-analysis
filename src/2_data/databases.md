# Database

This section will explore how to work with SQL databases in Rust. This section relies on the optional `SQL` section in the [Data](../1_start/data.md#sql-optional) section of the setup, where a PostgreSQL server was set up and the lfs data was loaded.

## Direct queries

You can direct query the data using the appropriate crate: [PostgreSQL](https://docs.rs/postgres/latest/postgres/), [MySql](https://docs.rs/mysql_common/latest/mysql_common/), [Sqlite](https://docs.rs/rusqlite/0.32.1/rusqlite/), [MSSQL](https://crates.io/crates/tiberius), [Oracle](https://docs.rs/tiberius/0.12.3/tiberius/). Other databases should also be available through these crates, such as Mariadb (MySql), ClickHouse (MySql), Redshift (PostgreSQL), Azure SQL Database (MSSql). You can run this section using `cargo run -r --example 2_4_1_postgresql`. 

```rust
=== Rust 2_4_1_postgresql evcxr
=== Rust 2_4_1_postgresql imports
=== Rust 2_4_1_postgresql block_1
```

> [!NOTE]
> Note that these SQL libraries generally return row-oriented data, when Polars (using the Arrow memory model) uses column-oriented data. This makes moving data between SQL and Arrow complex.

Using this method, each type of databases will have their own special connection code and return their own specific data. It's useful for queries with simple outputs, but hard to work with large outputs. This is where the `ConnectorX` library comes in.

## SQL to Polars

Using [ConnectorX](https://github.com/sfu-db/connector-x), you can move data from SQL servers to Polars with `.polars()`. It will return a `DataFrame` (currently `Polars 0.45`, but can be converted to the latest version of Polars with `df-interchange` as explained in the [concepts](../1_start/concepts.md#polars-versions) section of the setup). You can run this section using `cargo run -r --example 2_4_2_sql_to_polars`.

```rust
=== Rust 2_4_2_sql_to_polars evcxr
=== Rust 2_4_2_sql_to_polars imports
=== Rust 2_4_2_sql_to_polars block_1
```

This example will move the entirety of the SQL server into memory as a `DataFrame`. Further manipulations or analysis can be done on this data using Polars. This may or not be desirable, since the SQL server may contain too much data and will be slow. It may be preferable to pre-filter or to summarize the data using an SQL query. For example, you can collect the unweighted mean hourly wages in 2024, by month (using the following query: `"SELECT survmnth, avg(hrlyearn / 100) as avg_hourly FROM lfs where survyear = 2024 group by survmnth"`). This can then be converted into a `DataFrame` with `.polars()`.

```rust
=== Rust 2_4_2_sql_to_polars block_1
```

This will return a `DataFrame`.

```
shape: (12, 2)
┌──────────┬────────────┐
│ survmnth ┆ avg_hourly │
│ ---      ┆ ---        │
│ i64      ┆ f64        │
╞══════════╪════════════╡
│ 1        ┆ 33.932579  │
│ 2        ┆ 34.013028  │
│ 3        ┆ 34.049152  │
│ 4        ┆ 34.193399  │
│ 5        ┆ 34.085427  │
│ …        ┆ …          │
│ 8        ┆ 34.23867   │
│ 9        ┆ 34.738232  │
│ 10       ┆ 34.846987  │
│ 11       ┆ 34.791847  │
│ 12       ┆ 34.872347  │
└──────────┴────────────┘
```