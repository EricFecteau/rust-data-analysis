// === imports
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect and process multiple census chunks
    let mut census_chunk = vec![];
    for c in 1..6 {
        let args = ScanArgsParquet::default();
        let lf = LazyFrame::scan_parquet(
            PlPath::from_string(format!("./data/parquet/census_{c}.parquet")),
            args,
        )
        .unwrap();

        census_chunk.push(lf);
    }

    // === block_2

    // Concatenate vertically two (or more) datasets
    let five_percent_sample = concat(
        [
            census_chunk[0].clone(), // Cloned, since we need it later
            census_chunk[1].clone(),
            census_chunk[2].clone(),
            census_chunk[3].clone(),
            census_chunk[4].clone(),
        ],
        UnionArgs::default(),
    )
    .unwrap();

    // === block_3

    // See `chunk` going from 1 to 5
    println!(
        "{}",
        five_percent_sample
            .filter(col("chunk").eq(1))
            .collect()
            .unwrap()
    );

    // === block_4

    // Update the census_chunk vector to remove variables and update values
    for c in 1..6 {
        census_chunk[c - 1] = census_chunk[c - 1]
            .clone()
            .filter(col("income").is_not_null())
            .select([col("id"), col("income").alias(format!("inc_{c}"))])
            .collect() // Collect necessary for sampling, but small database
            .unwrap()
            .sample_n_literal(150_000, false, false, Some(c as u64))
            .unwrap()
            .lazy();
    }

    // === block_5

    println!("{}", census_chunk[0].clone().collect().unwrap());

    // === block_6

    // Left join (creating a cohort)
    let cohort = census_chunk[0]
        .clone()
        .left_join(census_chunk[1].clone(), col("id"), col("id"))
        .left_join(census_chunk[2].clone(), col("id"), col("id"))
        .left_join(census_chunk[3].clone(), col("id"), col("id"))
        .left_join(census_chunk[4].clone(), col("id"), col("id"));

    println!("{}", cohort.collect().unwrap());

    // === block_7

    // Inner join (creating a "always earning" cohort)
    let longitudinal_all = census_chunk[0]
        .clone()
        .inner_join(census_chunk[1].clone(), col("id"), col("id"))
        .inner_join(census_chunk[2].clone(), col("id"), col("id"))
        .inner_join(census_chunk[3].clone(), col("id"), col("id"))
        .inner_join(census_chunk[4].clone(), col("id"), col("id"));

    println!("{}", longitudinal_all.collect().unwrap());

    // === block_8

    // More complex types of joins
    let fix_full_join_vars = [when(col("id").is_not_null())
        .then(col("id"))
        .otherwise(col("id_right"))
        .alias("id")];

    let longitudinal_any = census_chunk[0]
        .clone()
        .join(
            census_chunk[1].clone(),
            [col("id")],
            [col("id")],
            JoinArgs::new(JoinType::Full),
        )
        .with_columns(fix_full_join_vars.clone())
        .select([all().exclude_cols(["id_right"]).as_expr()])
        .join(
            census_chunk[2].clone(),
            [col("id")],
            [col("id")],
            JoinArgs::new(JoinType::Full),
        )
        .with_columns(fix_full_join_vars.clone())
        .select([all().exclude_cols(["id_right"]).as_expr()])
        .join(
            census_chunk[3].clone(),
            [col("id")],
            [col("id")],
            JoinArgs::new(JoinType::Full),
        )
        .with_columns(fix_full_join_vars.clone())
        .select([all().exclude_cols(["id_right"]).as_expr()])
        .join(
            census_chunk[4].clone(),
            [col("id")],
            [col("id")],
            JoinArgs::new(JoinType::Full),
        )
        .with_columns(fix_full_join_vars.clone())
        .select([all().exclude_cols(["id_right"]).as_expr()])
        .sort(["id"], Default::default());

    println!("{}", longitudinal_any.collect().unwrap());

    // === end
}
