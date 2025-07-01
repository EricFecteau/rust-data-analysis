// :dep polars = { version = "0.49", features = ["lazy", "parquet", "regex", "replace"] }

use polars::prelude::*;

fn main() {
    // Connect and process multiple monthly parquet file
    let mut lfs_month = vec![];
    for m in 1..5 {
        let mm = format!("{:02}", m);

        let args = ScanArgsParquet::default();
        let lf = LazyFrame::scan_parquet(format!("./data/lfs_parquet/pub{mm}23.parquet"), args)
            .unwrap()
            .filter(col("hrlyearn").is_not_null())
            .select([
                col("rec_num"),
                col("survyear"),
                col("hrlyearn").alias(format!("earn_{mm}")),
            ]);

        lfs_month.push(lf.collect().unwrap());
    }

    println!("{:?}", lfs_month);

    // // Concatonate vertically two (or more) datasts
    // let lf_jan_to_apr = concat(
    //     [
    //         lf_jan.clone(), // Cloned, since we need it later
    //         lf_feb.clone(),
    //         lf_mar.clone(),
    //         lf_apr.clone(),
    //     ],
    //     UnionArgs::default(),
    // )
    // .unwrap();

    // // See `survmnth` going from 1 to 4 for 2023
    // println!("{}", lf_jan_to_apr.collect().unwrap());

    // // Get data ready for join
    // let lf_jan = lf_jan.filter(col("hrlyearn").is_not_null()).select([
    //     col("rec_num"),
    //     col("survyear"),
    //     col("hrlyearn").alias("earn_jan"),
    // ]);
    // let lf_feb = lf_feb.filter(col("hrlyearn").is_not_null()).select([
    //     col("rec_num"),
    //     col("survyear"),
    //     col("hrlyearn").alias("earn_feb"),
    // ]);
    // let lf_mar = lf_mar.filter(col("hrlyearn").is_not_null()).select([
    //     col("rec_num"),
    //     col("survyear"),
    //     col("hrlyearn").alias("earn_mar"),
    // ]);
    // let lf_apr = lf_apr.filter(col("hrlyearn").is_not_null()).select([
    //     col("rec_num"),
    //     col("survyear"),
    //     col("hrlyearn").alias("earn_apr"),
    // ]);

    // // Left join (creating a cohort)
    // let jan_cohort = lf_jan
    //     .clone()
    //     .drop([col("survyear")])
    //     .left_join(
    //         lf_feb.clone().drop([col("survyear")]),
    //         col("rec_num"),
    //         col("rec_num"),
    //     )
    //     .left_join(
    //         lf_mar.clone().drop([col("survyear")]),
    //         col("rec_num"),
    //         col("rec_num"),
    //     )
    //     .left_join(
    //         lf_apr.clone().drop([col("survyear")]),
    //         col("rec_num"),
    //         col("rec_num"),
    //     );

    // println!("{}", jan_cohort.collect().unwrap());

    // // Inner join (creating a "always earning" cohort)
    // let longitudinal_all = lf_jan
    //     .clone()
    //     .drop([col("survyear")])
    //     .inner_join(
    //         lf_feb.clone().drop([col("survyear")]),
    //         col("rec_num"),
    //         col("rec_num"),
    //     )
    //     .inner_join(
    //         lf_mar.clone().drop([col("survyear")]),
    //         col("rec_num"),
    //         col("rec_num"),
    //     )
    //     .inner_join(
    //         lf_apr.clone().drop([col("survyear")]),
    //         col("rec_num"),
    //         col("rec_num"),
    //     );

    // println!("{}", longitudinal_all.collect().unwrap());

    // // More complex types of joins (e.g. join on multiple variables)
    // let fix_full_join_vars = [
    //     when(col("rec_num").is_not_null())
    //         .then(col("rec_num"))
    //         .otherwise(col("rec_num_right"))
    //         .alias("rec_num"),
    //     when(col("survyear").is_not_null())
    //         .then(col("survyear"))
    //         .otherwise(col("survyear_right"))
    //         .alias("survyear"),
    // ];

    // let longitudinal_all = lf_jan
    //     .join(
    //         lf_feb,
    //         [col("rec_num"), col("survyear")],
    //         [col("rec_num"), col("survyear")],
    //         JoinArgs::new(JoinType::Full),
    //     )
    //     .with_columns(fix_full_join_vars.clone())
    //     .drop([col("rec_num_right"), col("survyear_right")])
    //     .join(
    //         lf_mar,
    //         [col("rec_num"), col("survyear")],
    //         [col("rec_num"), col("survyear")],
    //         JoinArgs::new(JoinType::Full),
    //     )
    //     .with_columns(fix_full_join_vars.clone())
    //     .drop([col("rec_num_right"), col("survyear_right")])
    //     .join(
    //         lf_apr,
    //         [col("rec_num"), col("survyear")],
    //         [col("rec_num"), col("survyear")],
    //         JoinArgs::new(JoinType::Full),
    //     )
    //     .with_columns(fix_full_join_vars.clone())
    //     .drop([col("rec_num_right"), col("survyear_right")])
    //     .sort(["rec_num", "survyear"], Default::default());

    // println!("{}", longitudinal_all.collect().unwrap());
}
