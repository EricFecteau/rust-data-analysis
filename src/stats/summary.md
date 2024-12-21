# Summary statistics

(pl.DataFrame(data)
    .groupby("group")
    .agg([
        (pl.col("rating") * pl.col("weight")).sum() / pl.sum("weight")
    ])
)