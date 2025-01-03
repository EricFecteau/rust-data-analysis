# Database

This section will explore how to work with SQL databases in Rust. In the [Data]() section of the setup, a PostgreSQL server was set up and the lfs data was loaded.

## Direct queries

You can direct query the data using the appropriate crate: [PostgreSQL](https://docs.rs/postgres/latest/postgres/), [MySql](https://docs.rs/mysql_common/latest/mysql_common/), [Sqlite](https://docs.rs/rusqlite/0.32.1/rusqlite/), [MSSQL](https://crates.io/crates/tiberius), [Oracle](https://docs.rs/tiberius/0.12.3/tiberius/). Other databases should also be available through these crates, such as Mariadb (MySql), ClickHouse (MySql), Redshift (PostgreSQL), Azure SQL Database (MSSql). You can run this section using `cargo run -r --example 2_3_1_postgresql`. 

```Rust
// Connect to postgresql
let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

// Query the database, returns a vector of rows
let data = client
    .query("select count(*) as count from lfs", &[])
    .unwrap();

// Get the first data point from the first row
let data_point: i64 = data[0].get(0);
```

> [!NOTE]
> Note that these SQL libraries generally return row-oriented data, when Polars (using the Arrow memory model) uses column-oriented data. This makes moving data between SQL and Arrow complex.

Using this method, each type of databases will have their own special connection code and return their own specific data. It's useful for queries with simple outputs, but hard to work with large outputs. This is where the `ConnectorX` library comes in.

## SQL to Polars

Using [ConnectorX](https://github.com/sfu-db/connector-x), you can move data from SQL servers to Polars with `.polars()`. It will return a `DataFrame`. You can run this section using `cargo run -r --example 2_3_2_sql_to_polars`.

```Rust
// Connect to PostgreSQL through the ConnectorX
let source_conn =
    SourceConn::try_from("postgresql://postgres:postgres@localhost:5432").unwrap();

// Prepare query
let query = &[CXQuery::from("SELECT * FROM lfs")];

// ConnectorX query PostgreSQL and return Polars object
let arrow_obj = get_arrow(&source_conn, None, query)
    .unwrap()
    .polars()
    .unwrap();
```

This example will move the entirety of the SQL server into memory as a `DataFrame`. Further manipulations or analysis can be done on this data using Polars. This may or not be desirable, since the SQL server may contain too much data. It may be preferable to pre-filter or to summarize the data using a SQL query. For example, you can collect the unweighted mean hourly wages in 2010, by month (using the following query: `"SELECT survmnth, avg(hrlyearn / 100) as avg_hourly FROM lfs where survyear = 2010 group by survmnth"`). This can then be converted into a `DataFrame` with `.polars()`.

```Rust
// Connect to PostgreSQL through the ConnectorX
let source_conn =
    SourceConn::try_from("postgresql://postgres:postgres@localhost:5432").unwrap();

// Prepare query
let query = &[CXQuery::from(
    "SELECT survmnth, avg(hrlyearn / 100) as avg_hourly FROM lfs where survyear = 2010 group by survmnth",
)];

// ConnectorX query PostgreSQL and return Polars object
let df = get_arrow(&source_conn, None, query)
    .unwrap()
    .polars()
    .unwrap();
```

This will return a `DataFrame`.

```
shape: (12, 2)
┌──────────┬────────────┐
│ survmnth ┆ avg_hourly │
│ ---      ┆ ---        │
│ i64      ┆ f64        │
╞══════════╪════════════╡
│ 1        ┆ 22.215189  │
│ 2        ┆ 22.190046  │
│ 3        ┆ 22.209012  │
│ 4        ┆ 22.149379  │
│ 5        ┆ 21.986003  │
│ …        ┆ …          │
│ 8        ┆ 21.900589  │
│ 9        ┆ 22.341671  │
│ 10       ┆ 22.44213   │
│ 11       ┆ 22.524596  │
│ 12       ┆ 22.523934  │
└──────────┴────────────┘
```