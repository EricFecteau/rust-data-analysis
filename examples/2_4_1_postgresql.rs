// === evcxr
// :dep postgres = "0.19"

// === imports
use postgres::{Client, NoTls};

// === main
fn main() {
    // === block_1

    // Connect to postgresql
    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

    // Query the database, returns a vector of rows
    let data = client
        .query("select count(*) as count from lfs", &[])
        .unwrap();

    // Get the first data point from the first row
    let data_point: i64 = data[0].get(0);

    println!("{data_point}")

    // === end
}
