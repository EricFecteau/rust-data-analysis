use connectorx::prelude::*;
use std::convert::TryFrom;

fn main() {
    // Connect to PostgreSQL through the ConnectorX
    let source_conn =
        SourceConn::try_from("postgresql://postgres:postgres@localhost:5432").unwrap();

    // Prepare query
    let query = &[CXQuery::from("SELECT * FROM lfs")];

    // ConnectorX query PostgreSQL and return Polars object
    let df = get_arrow(&source_conn, None, query, None)
        .unwrap()
        .polars()
        .unwrap();

    // Print table
    println!("{df}");

    // Prepare query
    let query = &[CXQuery::from(
    "SELECT survmnth, avg(hrlyearn / 100) as avg_hourly FROM lfs where survyear = 2024 group by survmnth",
    )];

    // ConnectorX query PostgreSQL and return Polars object
    let df = get_arrow(&source_conn, None, query, None)
        .unwrap()
        .polars() // Polars 0.45
        .unwrap();

    // Print table
    println!("{df}");
}
