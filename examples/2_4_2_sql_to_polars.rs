// === imports
use connectorx::prelude::*;
use std::{convert::TryFrom, env};

// === main
fn main() {
    // === block_1

    // Connect to PostgreSQL through the ConnectorX

    // Get standard URL or from env
    let postgres_url = env::var("POSTGRES_URL")
        .unwrap_or("postgresql://postgres:postgres@localhost:5432/postgres".to_string());

    // Connect to postgresql
    let source_conn = SourceConn::try_from(postgres_url.as_str()).unwrap();

    // Prepare query (london, aged 15 years and under)
    let query = &[CXQuery::from(
        "SELECT * FROM census WHERE region = 'E12000007' and age_group = 1",
    )];

    // ConnectorX query PostgreSQL and return Polars object
    let df = get_arrow(&source_conn, None, query, None)
        .unwrap()
        .polars()
        .unwrap();

    // Print table
    println!("{df}");

    // === block_2

    // Prepare query
    let query = &[CXQuery::from(
        "SELECT region, avg(income)::float FROM census group by region",
    )];

    // ConnectorX query PostgreSQL and return Polars object
    let df = get_arrow(&source_conn, None, query, None)
        .unwrap()
        .polars()
        .unwrap();

    // Print table
    println!("{df}");

    // === end
}
