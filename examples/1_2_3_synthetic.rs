// === imports
use polars::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

// === main
fn main() {
    // === program

    // Read CSV (into memory)
    let mut df = LazyCsvReader::new(PlPath::from_string("./data/raw/census.csv".to_string()))
        .with_infer_schema_length(Some(10_000)) // Default 100, missing = String
        .with_has_header(true)
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    // Start up seeded RNG
    let mut rng = ChaCha8Rng::seed_from_u64(1);

    // Create random income vector of correct size
    let random_income: Vec<i64> = (0..df.height())
        .map(|_| rng.random_range(10_000..100_000))
        .collect();

    // Create random weight vector of correct size
    let randome_weight: Vec<i64> = (0..df.height())
        .map(|_| rng.random_range(75..125))
        .collect();

    // Make them "Series" as required by Polars
    let income = Series::new("income".into(), random_income);
    let weight = Series::new("weight".into(), randome_weight);

    // Add them as columns to the data
    let df = df.with_column(income).unwrap().with_column(weight).unwrap();

    // If economically non-active, put income as `Null`
    let mut df = df
        .clone()
        .lazy()
        .with_column(
            when(col("econ").is_in(
                lit(Series::from_iter(vec![-8, 5, 6, 7, 8, 9])).implode(),
                false,
            ))
            .then(Null {}.lit())
            .otherwise(col("income"))
            .alias("income"),
        )
        .collect()
        .unwrap();

    // Write output to CSV
    let mut file = std::fs::File::create("./data/raw/census.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();

    // === end
}
