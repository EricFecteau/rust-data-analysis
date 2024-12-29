# Database

This section will explore how to work with SQL databases in Rust. In the [Data]() section of the setup, a PostgreSQL server was set up and the lfs data was loaded. This section should work with various other databases, using their respective crates: [PostgreSQL](https://docs.rs/postgres/latest/postgres/), [MySql](https://docs.rs/mysql_common/latest/mysql_common/), [Sqlite](https://docs.rs/rusqlite/0.32.1/rusqlite/), [MSSQL](https://crates.io/crates/tiberius), [Oracle](https://docs.rs/tiberius/0.12.3/tiberius/). Other databases should also be available through these libraries, such as Mariadb (MySql), ClickHouse (MySql), Redshift (PostgreSQL), Azure SQL Database (MSSql). 

## Queries



## SQL to Polars

Moving data from SQL to Polars is an area that requires significant improvement in Rust, but it's possible to implement on-the-fly conversion methods.

### Existing solutions

The [ConnectorX](https://github.com/sfu-db/connector-x) crate, widely used in the Python version of Polars to move data from SQL to Polars, works very well, but provides an ancient version of a Polars object (version 0.32, released in August 2023) that does not have many features used in the examples in this book. It also uses the [Arrow2](https://github.com/jorgecarleitao/arrow2?tab=readme-ov-file#this-crate-is-unmaintained), an archived and no longer maintained arrow package, for transport between SQL and Polars. 


Since [Polars 0.44](https://github.com/pola-rs/polars/pull/19312), it is no longer possible to convert `arrow-rs` data to `polars-arrow`, and therefore import this data from ConnectorX to a current version of Polars.

### Custom solutions

