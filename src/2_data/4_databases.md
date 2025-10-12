# Database

This section will explore how to work with SQL databases in Rust. It relies on the optional `SQL` section in the [Data](../1_start/3_data.md#sql-optional) section of the setup, where a PostgreSQL server was set up and the Census data was loaded.

## Direct queries

You can direct query the data using the appropriate crate: [PostgreSQL](https://docs.rs/postgres/latest/postgres/), [MySql](https://docs.rs/mysql_common/latest/mysql_common/), [Sqlite](https://docs.rs/rusqlite/0.32.1/rusqlite/), [MSSQL](https://crates.io/crates/tiberius), [Oracle](https://docs.rs/tiberius/0.12.3/tiberius/). Other databases should also be available through these crates, such as Mariadb (MySql), ClickHouse (MySql), Redshift (PostgreSQL), Azure SQL Database (MSSql). You can run this section using `cargo run -r --example 2_4_1_postgresql`. 

```rust
=== Rust 2_4_1_postgresql imports
=== Rust 2_4_1_postgresql block_1
```

> [!NOTE]
> Note that these SQL libraries generally return row-oriented data, where Polars (using the Arrow memory model) uses column-oriented data. This makes moving data between SQL and Arrow complex.

Using this method, each type of databases will have their own special connection code and return their own specific data. It's useful for queries with simple outputs, but hard to work with large outputs. This is where the `ConnectorX` library comes in.

## SQL to Polars

Using [ConnectorX](https://docs.rs/connectorx/latest/connectorx/), you can move data from SQL servers to Polars with `get_arrow().polars()`. It will return a `DataFrame` (that can be converted to the latest version of Polars with `df-interchange` as explained in the [concepts](../1_start/4_concepts.md#polars-and-arrow-versions) section of the setup). You can run this section using `cargo run -r --example 2_4_2_sql_to_polars`.

```rust
=== Rust 2_4_2_sql_to_polars imports
=== Rust 2_4_2_sql_to_polars block_1
```

This example will move the "London" (region = 'E12000007') 0 to 15 years old (age_group = 1) data from the SQL server into memory as a `DataFrame`. Further manipulations or analysis can be done on this data using Polars. Loading the entirety of the server data into memory is generally not desired. As in this example, it is possible to pre-filter or to summarize the data using an SQL query. As a more complex example, you can collect the average income, by region, using the following query: `"SELECT region, avg(income)::float FROM census group by region"`. This can then be converted into a `DataFrame` with `.polars()`.

```rust
=== Rust 2_4_2_sql_to_polars block_2
```

This will return a `DataFrame`.

```
shape: (10, 2)
┌───────────┬──────────────┐
│ region    ┆ avg          │
│ ---       ┆ ---          │
│ str       ┆ f64          │
╞═══════════╪══════════════╡
│ E12000001 ┆ 55032.641723 │
│ E12000002 ┆ 54990.207499 │
│ E12000003 ┆ 55114.538138 │
│ E12000004 ┆ 54806.553317 │
│ E12000005 ┆ 55269.042885 │
│ E12000006 ┆ 55091.448925 │
│ E12000007 ┆ 54888.739957 │
│ E12000008 ┆ 55153.381614 │
│ E12000009 ┆ 55028.588204 │
│ W92000004 ┆ 55042.71009  │
└───────────┴──────────────┘
```