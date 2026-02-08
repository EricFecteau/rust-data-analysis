use charton::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load dataset
    let df = load_dataset("iris")?;

    // Single layer chart
    Chart::build(&df)?
        .mark_point()
        .encode((
            x("sepal_length"),
            y("sepal_width"),
            color("species")
        ))?
        .into_layered()
        .save("single_layer.svg")?;

    // Multiple layers chart
    let df_subset = df.head(Some(4)).tail(Some(3));
    let line = Chart::build(&df_subset)?
        .mark_line()
        .encode((x("sepal_length"), y("sepal_width")))?;

    let points = Chart::build(&df_subset)?
        .mark_point()
        .encode((x("sepal_length"), y("sepal_width")))?;

    LayeredChart::new()
        .add_layer(line)
        .add_layer(points)
        .save("multiple_layers.svg")?;

    Ok(())
}