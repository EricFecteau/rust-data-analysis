use polars::prelude::*;

use hypors::chi_square::independence;

fn main() {
    let df = LazyCsvReader::new("/home/eric/R/large_csv/housetask.csv")
        .with_has_header(true)
        .finish()
        .unwrap()
        .select([
            col("Wife").cast(DataType::Float64),
            col("Alternating").cast(DataType::Float64),
            col("Husband").cast(DataType::Float64),
            col("Jointly").cast(DataType::Float64),
        ])
        .collect()
        .unwrap();

    let cols = df
        .get_columns()
        .iter()
        .map(|c| {
            c.as_materialized_series()
                .f64()
                .unwrap()
                .to_vec_null_aware()
                .left()
                .unwrap()
        })
        .collect::<Vec<Vec<f64>>>();

    // Perform Chi-Square Test for Independence
    let result = independence(&cols, 0.01).unwrap();
    println!("{:?}", result);
}

// file_path <- "https://www.sthda.com/sthda/RDoc/data/housetasks.txt"
// housetasks <- read.delim(file_path, row.names = 1)

// write.csv(housetasks, "/home/eric/R/large_csv/housetask.csv")

// chisq <- chisq.test(housetasks)
