# Joins

This section explore how to join two datasets, either by stacking them one on top of the other (same columns) or by stacking them side by side (same rows).

## Concatonate

Lets first create a vector containing four months (Jan to April) of LFS data, pulled form the single-month versions of the Parquet files.

```Rust
// Connect and process multiple monthly parquet file
let mut lfs_month = vec![];
for m in 1..5 {
    let mm = format!("{:02}", m);

    let args = ScanArgsParquet::default();
    let lf =
        LazyFrame::scan_parquet(format!("./data/lfs_parquet/pub{mm}23.parquet"), args).unwrap();

    lfs_month.push(lf);
}
```

To concatenate data of the same row-shape on top of eachother, we can use the `concat` function by listing the LazyFrames we want to stack. Here, we can concatonate all four months of the LFS found in the `lfs_month` vector.

```Rust
// Concatonate vertically two (or more) datasts
let lf_jan_to_apr = concat(
    [
        lfs_month[0].clone(), // Cloned, since we need it later
        lfs_month[1].clone(),
        lfs_month[2].clone(),
        lfs_month[3].clone(),
    ],
    UnionArgs::default(),
)
.unwrap();
```

If we print the result, it shows that we have a DataFrame of over 400,000 rows (4 x ~100,000 rows per monthly file). It is also possible to see that `survmnth` has multiple monthly values.

```Rust
println!("{}", lf_jan_to_apr.collect().unwrap());
```

```
shape: (413_982, 60)
┌─────────┬──────────┬──────────┬─────────┬───┬─────────┬──────────┬─────────┬─────────┐
│ rec_num ┆ survyear ┆ survmnth ┆ lfsstat ┆ … ┆ schooln ┆ efamtype ┆ agyownk ┆ finalwt │
│ ---     ┆ ---      ┆ ---      ┆ ---     ┆   ┆ ---     ┆ ---      ┆ ---     ┆ ---     │
│ i64     ┆ i64      ┆ i64      ┆ i64     ┆   ┆ i64     ┆ i64      ┆ i64     ┆ i64     │
╞═════════╪══════════╪══════════╪═════════╪═══╪═════════╪══════════╪═════════╪═════════╡
│ 1       ┆ 2023     ┆ 1        ┆ 4       ┆ … ┆ null    ┆ 11       ┆ null    ┆ 248     │
│ 2       ┆ 2023     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 2        ┆ null    ┆ 70      │
│ 3       ┆ 2023     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 8        ┆ null    ┆ 168     │
│ 4       ┆ 2023     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 1        ┆ null    ┆ 1434    │
│ 5       ┆ 2023     ┆ 1        ┆ 1       ┆ … ┆ 1       ┆ 2        ┆ null    ┆ 119     │
│ …       ┆ …        ┆ …        ┆ …       ┆ … ┆ …       ┆ …        ┆ …       ┆ …       │
│ 100020  ┆ 2023     ┆ 4        ┆ 1       ┆ … ┆ 1       ┆ 3        ┆ 1       ┆ 236     │
│ 100021  ┆ 2023     ┆ 4        ┆ 1       ┆ … ┆ 1       ┆ 18       ┆ null    ┆ 258     │
│ 100022  ┆ 2023     ┆ 4        ┆ 4       ┆ … ┆ null    ┆ 18       ┆ null    ┆ 105     │
│ 100023  ┆ 2023     ┆ 4        ┆ 1       ┆ … ┆ 1       ┆ 1        ┆ null    ┆ 237     │
│ 100024  ┆ 2023     ┆ 4        ┆ 1       ┆ … ┆ 1       ┆ 2        ┆ null    ┆ 1082    │
└─────────┴──────────┴──────────┴─────────┴───┴─────────┴──────────┴─────────┴─────────┘
```

## Joins

Polars has multiple options for joining data by row. To make the data visualization simpler, the below code processes the data found in the `lfs_month` vector to keep only a few variables, remove those without any hourly earnings and renames the `hrlyearn` to reflect the month of the data. Note that `rec_num` is a row number, but will be used as linkage key in this example.

```Rust
for m in 1..5 {
    let mm = format!("{:02}", m);

    lfs_month[m - 1] = lfs_month[m - 1]
        .clone()
        .filter(col("hrlyearn").is_not_null())
        .select([
            col("rec_num"),
            col("survyear"),
            col("hrlyearn").alias(format!("earn_{mm}")),
        ]);
}
```

Now that we have simpler data, we can join these. In this example, we are doing multiple left joins in a row with the `left_join` function, always keeping the original population (akin to creating a cohort). To do these joins, all we need is the data and the left and right key (`rec_num` in this example).

```Rust
let jan_cohort = lfs_month[0]
    .clone()
    .drop([col("survyear")])
    .left_join(
        lfs_month[1].clone().drop([col("survyear")]),
        col("rec_num"),
        col("rec_num"),
    )
    .left_join(
        lfs_month[2].clone().drop([col("survyear")]),
        col("rec_num"),
        col("rec_num"),
    )
    .left_join(
        lfs_month[3].clone().drop([col("survyear")]),
        col("rec_num"),
        col("rec_num"),
    );

println!("{}", jan_cohort.collect().unwrap());
```

This gives us a longitudinal cohort, keeping the population from the first dataset (january):

```
shape: (54_207, 5)
┌─────────┬─────────┬─────────┬─────────┬─────────┐
│ rec_num ┆ earn_01 ┆ earn_02 ┆ earn_03 ┆ earn_04 │
│ ---     ┆ ---     ┆ ---     ┆ ---     ┆ ---     │
│ i64     ┆ i64     ┆ i64     ┆ i64     ┆ i64     │
╞═════════╪═════════╪═════════╪═════════╪═════════╡
│ 3       ┆ 2900    ┆ null    ┆ null    ┆ 1920    │
│ 4       ┆ 2000    ┆ 2650    ┆ null    ┆ null    │
│ 6       ┆ 4238    ┆ null    ┆ null    ┆ null    │
│ 10      ┆ 3590    ┆ 3000    ┆ 2950    ┆ null    │
│ 12      ┆ 1900    ┆ null    ┆ null    ┆ 9375    │
│ …       ┆ …       ┆ …       ┆ …       ┆ …       │
│ 108056  ┆ 1827    ┆ null    ┆ null    ┆ null    │
│ 108058  ┆ 2897    ┆ null    ┆ null    ┆ null    │
│ 108059  ┆ 3300    ┆ null    ┆ null    ┆ null    │
│ 108062  ┆ 1825    ┆ null    ┆ null    ┆ null    │
│ 108063  ┆ 1750    ┆ null    ┆ null    ┆ null    │
└─────────┴─────────┴─────────┴─────────┴─────────┘
```
In the same way, we can also use other types of joins, like the `inner_join`:

```Rust
let longitudinal_all = lfs_month[0]
    .clone()
    .drop([col("survyear")])
    .inner_join(
        lfs_month[1].clone().drop([col("survyear")]),
        col("rec_num"),
        col("rec_num"),
    )
    .inner_join(
        lfs_month[2].clone().drop([col("survyear")]),
        col("rec_num"),
        col("rec_num"),
    )
    .inner_join(
        lfs_month[3].clone().drop([col("survyear")]),
        col("rec_num"),
        col("rec_num"),
    );

println!("{}", longitudinal_all.collect().unwrap());

```

This creates a cohort of those who are in every dataset.

```
shape: (6_494, 5)
┌─────────┬─────────┬─────────┬─────────┬─────────┐
│ rec_num ┆ earn_01 ┆ earn_02 ┆ earn_03 ┆ earn_04 │
│ ---     ┆ ---     ┆ ---     ┆ ---     ┆ ---     │
│ i64     ┆ i64     ┆ i64     ┆ i64     ┆ i64     │
╞═════════╪═════════╪═════════╪═════════╪═════════╡
│ 38      ┆ 3077    ┆ 1442    ┆ 1600    ┆ 1600    │
│ 76      ┆ 4423    ┆ 5300    ┆ 3510    ┆ 1700    │
│ 82      ┆ 4952    ┆ 5282    ┆ 2578    ┆ 1475    │
│ 102     ┆ 4087    ┆ 3436    ┆ 4769    ┆ 4423    │
│ 109     ┆ 4712    ┆ 7692    ┆ 4500    ┆ 1440    │
│ …       ┆ …       ┆ …       ┆ …       ┆ …       │
│ 99968   ┆ 2646    ┆ 2400    ┆ 3500    ┆ 1550    │
│ 99971   ┆ 4615    ┆ 3990    ┆ 2000    ┆ 9000    │
│ 99974   ┆ 6319    ┆ 8077    ┆ 3250    ┆ 4667    │
│ 99999   ┆ 1500    ┆ 2400    ┆ 1950    ┆ 6357    │
│ 100016  ┆ 2308    ┆ 3000    ┆ 6200    ┆ 5000    │
└─────────┴─────────┴─────────┴─────────┴─────────┘
```

Polars has multiple of these "simple" joins, incliding `left_join`, `semi_join`, `full_join`, `inner_join`, `anti_join` and `cross_join`. But you can create singnificantly more complex joins by building the `join` yourself with the `join` function and all of it's options. For example, here is a `full join` on multiple variables:

> [!NOTE]
> For some reason, Polars does not reconsile the values of the keys in a `full join`, both in the `join` and `full_join` functions. This means that any keys not found in the left create `nulls` in the original key name and any key not found in the right creates `nulls` in the key with an `_right` suffix (e.g. rec_num_right). I fix this in the example below with an expression that applies to all four joins.

```Rust
let fix_full_join_vars = [
    when(col("rec_num").is_not_null())
        .then(col("rec_num"))
        .otherwise(col("rec_num_right"))
        .alias("rec_num"),
    when(col("survyear").is_not_null())
        .then(col("survyear"))
        .otherwise(col("survyear_right"))
        .alias("survyear"),
];

let longitudinal_all = lfs_month[0]
    .clone()
    .join(
        lfs_month[1].clone(),
        [col("rec_num"), col("survyear")],
        [col("rec_num"), col("survyear")],
        JoinArgs::new(JoinType::Full),
    )
    .with_columns(fix_full_join_vars.clone())
    .drop([col("rec_num_right"), col("survyear_right")])
    .join(
        lfs_month[2].clone(),
        [col("rec_num"), col("survyear")],
        [col("rec_num"), col("survyear")],
        JoinArgs::new(JoinType::Full),
    )
    .with_columns(fix_full_join_vars.clone())
    .drop([col("rec_num_right"), col("survyear_right")])
    .join(
        lfs_month[3].clone(),
        [col("rec_num"), col("survyear")],
        [col("rec_num"), col("survyear")],
        JoinArgs::new(JoinType::Full),
    )
    .with_columns(fix_full_join_vars.clone())
    .drop([col("rec_num_right"), col("survyear_right")])
    .sort(["rec_num", "survyear"], Default::default());

println!("{}", longitudinal_all.collect().unwrap());
```

This full join keeps a superpopulation of all the four datasets:

```
shape: (99_250, 6)
┌─────────┬──────────┬─────────┬─────────┬─────────┬─────────┐
│ rec_num ┆ survyear ┆ earn_01 ┆ earn_02 ┆ earn_03 ┆ earn_04 │
│ ---     ┆ ---      ┆ ---     ┆ ---     ┆ ---     ┆ ---     │
│ i64     ┆ i64      ┆ i64     ┆ i64     ┆ i64     ┆ i64     │
╞═════════╪══════════╪═════════╪═════════╪═════════╪═════════╡
│ 1       ┆ 2023     ┆ null    ┆ 2500    ┆ null    ┆ 6000    │
│ 2       ┆ 2023     ┆ null    ┆ 2500    ┆ 3534    ┆ 6731    │
│ 3       ┆ 2023     ┆ 2900    ┆ null    ┆ null    ┆ 1920    │
│ 4       ┆ 2023     ┆ 2000    ┆ 2650    ┆ null    ┆ null    │
│ 5       ┆ 2023     ┆ null    ┆ null    ┆ null    ┆ 1750    │
│ …       ┆ …        ┆ …       ┆ …       ┆ …       ┆ …       │
│ 108056  ┆ 2023     ┆ 1827    ┆ null    ┆ null    ┆ null    │
│ 108058  ┆ 2023     ┆ 2897    ┆ null    ┆ null    ┆ null    │
│ 108059  ┆ 2023     ┆ 3300    ┆ null    ┆ null    ┆ null    │
│ 108062  ┆ 2023     ┆ 1825    ┆ null    ┆ null    ┆ null    │
│ 108063  ┆ 2023     ┆ 1750    ┆ null    ┆ null    ┆ null    │
└─────────┴──────────┴─────────┴─────────┴─────────┴─────────┘
```