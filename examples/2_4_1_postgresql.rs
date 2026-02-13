// === imports
use postgres::{Client, NoTls};
use std::env;

// === main
fn main() {
    // === block_1

    // URL
    let postgres_url = "postgresql://postgres:postgres@localhost:5432/postgres";

    // Connect to postgresql
    let mut client = Client::connect(
        env::var("POSTGRES_URL")
            .unwrap_or(postgres_url.to_string())
            .as_str(),
        NoTls,
    )
    .unwrap();

    // Query the database, returns a vector of rows
    let data = client
        .query("select count(*) as count from census", &[])
        .unwrap();

    // Get the first data point from the first row
    let data_point: i64 = data[0].get(0);

    println!("{data_point}")

    // === end
}
