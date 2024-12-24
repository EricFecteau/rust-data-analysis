use polars::df;
use polars::prelude::*;

fn main() {
    let df = df! [
        "val1" => [Some(1), Some(2), None],
        "val2" => [Some(1), None, None],
    ]
    .unwrap()
    .lazy();

    let df2 = df.filter(col("val2").is_null().xor(col("val1").is_null()));

    println!("{}", df2.collect().unwrap());
}
