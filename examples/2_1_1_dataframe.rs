// :dep polars = "0.48"

use polars::prelude::*;

fn main() {
    // Creating columns
    let year = Column::new("year".into(), [2010, 2010, 2011, 2011, 2011, 2011]);
    let month = Column::new(
        "month".into(),
        [
            "November", "December", "January", "February", "March", "April",
        ],
    );
    let value = Column::new("value".into(), [1.25, 2.50, 3.75, 4.00, 3.75, 4.25]);

    // Using columns to create a DataFrame
    let df = DataFrame::new(vec![year, month, value]).unwrap();

    println!("{}", df);

    // Use the df! macro to create DataFrame
    let df = df!("year" => [2008, 2008, 2008, 2008, 2009, 2009],
                            "month" => ["September", "October", "November", "December", "January", "February"],
                            "value" => [0.21, 0.22, 0.23, 0.25, 0.24, 0.25])
    .unwrap();

    println!("{}", df);
}
