# Pivots

Pivoting a dataframe allows you to make wide data longer or long data wider. This is done by increasing the number of columns and decreasing the number of rows, or vice versa. 

As explained by Polars, "lazy does not implement a pivot because it is impossible to know the schema without materializing the whole dataset". In other words, if, for example, you wanted to pivot wider on the province variable (e.g. make a column for each province), until Polars reads every single row in your dataset it can not know how many columns it would create. Therefore, it can not move forward lazily and continue optimizing the query, without materializing the dataframe. Polars does not allow you to provide a schema to solve this issue lazily. Caution should be taken when pivoting large dataframes as it will have to be done eagerly.

## Pivot wider

`unstable pivot not yet supported, using stable pivot`


## Pivot longer