# Plotting

Plots are the language of data analysts. There are dozens of different plot types, and the majority of them are possible in Rust today.

This chapter introduces **two complementary approaches to data visualization in Rust**.

The first approach builds on [Plotly](https://plotly.com/javascript/), the very popular JavaScript open source graphing library, has a [Rust interface](https://github.com/plotly/plotly.rs). We will use the [plotlars](https://github.com/alceal/plotlars) crate, a wrapper around the Plotly library that takes polars dataframes as input. This is a great bridge between Polars and Plotly. Plotlars allows you to build all types of graphs, like bar plots, box plots, line plots, pie charts and sankey diagrams.

The second approach explores [Charton](https://github.com/wangjiawen2013/charton), a declarative plotting library designed for Rust. Charton emphasizes a grammar-of-graphics style API and first-class Polars support, and can render plots natively in Rust or delegate rendering to external visualization backends when appropriate. This model is particularly well suited for exploratory data analysis, composable chart construction, and web-oriented workflows.

## Plotting with Plotly and plotlars

Run the Plotly-based examples in this section using:

```bash
cargo run -r --example 5_2_1_plots
```

### Bar Graph

#### Setup

Lets get some summary statistics to output as a bar graph. Here is a table of the mean income by sex and region:

```rust
=== Rust 5_2_1_plots imports
=== Rust 5_2_1_plots block_1
```

```
shape: (20, 3)
┌────────┬──────────────────────────┬──────────┐
│ sex    ┆ region                   ┆ income   │
│ ---    ┆ ---                      ┆ ---      │
│ str    ┆ str                      ┆ f64      │
╞════════╪══════════════════════════╪══════════╡
│ Female ┆ North East               ┆ 54834.9  │
│ Female ┆ North West               ┆ 55016.59 │
│ Female ┆ Yorkshire and The Humber ┆ 55089.42 │
│ Female ┆ East Midlands            ┆ 54911.42 │
│ Female ┆ West Midlands            ┆ 55141.59 │
│ …      ┆ …                        ┆ …        │
│ Male   ┆ East of England          ┆ 55189.26 │
│ Male   ┆ London                   ┆ 54936.82 │
│ Male   ┆ South East               ┆ 55201.77 │
│ Male   ┆ South West               ┆ 55239.98 │
│ Male   ┆ Wales                    ┆ 54865.14 │
└────────┴──────────────────────────┴──────────┘
```

#### Building the bar graph

To build a graph, you start with the data, you give it an `x` axis and a `y` axis, a `group` if you have groups and then various options, like `x_title`, `y_title`, `plot_title`, `size` for text, etc. Most functions are self-explanatory, but they are described in the documentation of the [bar graph](https://docs.rs/plotlars/latest/plotlars/struct.BarPlot.html).

```Rust
=== Rust 5_2_1_plots block_2
```

In this example, `to_html()` was used to imbed in this page to be interactive. You can save it to a `.html` file:

```Rust
=== Rust 5_2_1_plots block_3
```

Opening this file, will give you this interactive bar chart:

<div>
<meta charset="utf-8" />
<script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"></script>
<script src="https://cdn.plot.ly/plotly-3.0.1.min.js"></script>

<div id="plotly-html-element" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script type="module">
    const graph_div = document.getElementById("plotly-html-element");
    await Plotly.newPlot(graph_div, {"data":[{"type":"bar","x":["North East","North West","Yorkshire and The Humber","East Midlands","West Midlands","East of England","London","South East","South West","Wales"],"y":[54834.9,55016.59,55089.42,54911.42,55141.59,54741.55,54775.1,54993.65,54964.28,55296.26],"name":"Female","orientation":"v","marker":{"color":"rgb(255, 127, 80)"}},{"type":"bar","x":["North East","North West","Yorkshire and The Humber","East Midlands","West Midlands","East of England","London","South East","South West","Wales"],"y":[55500.69,55106.8,54866.75,54502.34,55477.21,55189.26,54936.82,55201.77,55239.98,54865.14],"name":"Male","orientation":"v","marker":{"color":"rgb(64, 224, 208)"}}],"layout":{"title":{"text":"Income by sex and region","font":{"family":"Arial","size":18,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9},"legend":{"orientation":"h","x":0.37,"y":1.1,"title":{"text":"Sex","font":{"family":"Arial","size":15,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9}},"xaxis":{"title":{"text":"Region","font":{"family":"Arial","size":15,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9}},"yaxis":{"title":{"text":"Mean Income","font":{"family":"Arial","size":15,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9}},"barmode":"group"},"config":{},"frames":null});
</script>
</div>

Instead of the `to_html()`, you can also write an image using this syntax: `.write_image("./data/output/out.png", 800, 600, 1.0).unwrap()`

### Line Plot

#### Setup

Lets also get some summary statistics to output as a line plot. Here is a table of the mean income by sex and ours worked (groupped), pivoted on sex:


```rust
=== Rust 5_2_1_plots block_4
```

```
shape: (4, 3)
┌──────────────────┬──────────┬──────────┐
│ hours_worked     ┆ Female   ┆ Male     │
│ ---              ┆ ---      ┆ ---      │
│ str              ┆ f64      ┆ f64      │
╞══════════════════╪══════════╪══════════╡
│ 15 hours or less ┆ 54997.22 ┆ 54990.49 │
│ 16 to 30 hours   ┆ 54949.22 ┆ 55026.07 │
│ 31 to 48 hours   ┆ 54904.08 ┆ 55022.64 │
│ 49 or more hours ┆ 55195.33 ┆ 55353.39 │
└──────────────────┴──────────┴──────────┘
```

#### Building the line plot

Similar to the bar graph, for the line plot, you start with the data, you give it an `x` axis and a `y` axis (and here, you add another `y` axis with `additional_lines` to add a new line to the line plot), and then various options, like `x_title`, `y_title`, `plot_title`, `size` for text, etc. Most functions are self-explanatory, but they are described in the documentation of the [line plot](https://docs.rs/plotlars/latest/plotlars/struct.LinePlot.html).

```Rust
=== Rust 5_2_1_plots block_5
```

In this example, `to_html()` was used to imbed in this page to be interactive. You can save it to a `.html` file:

```Rust
=== Rust 5_2_1_plots block_6
```

Opening this file, will give you this interactive bar chart:

<div>
<meta charset="utf-8" />
<script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"></script>
<script src="https://cdn.plot.ly/plotly-3.0.1.min.js"></script>
    
<div id="plotly-html-element-2" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script type="module">
    const graph_div = document.getElementById("plotly-html-element-2");
    await Plotly.newPlot(graph_div, {"data":[{"type":"scatter","name":"Female","x":[1.0,2.0,3.0,4.0],"y":[54997.22,54949.22,54904.08,55195.33],"marker":{"size":12,"color":"rgb(178, 34, 34)"},"line":{}},{"type":"scatter","name":"Male","x":[1.0,2.0,3.0,4.0],"y":[54990.49,55026.07,55022.64,55353.39],"marker":{"size":12,"color":"rgb(65, 105, 225)"},"line":{}}],"layout":{"title":{"text":"Mean income by hours worked (groupped) and sex","font":{"family":"","size":0,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9},"legend":{"title":{"text":"Sex","font":{"family":"","size":0,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9}},"xaxis":{"title":{"text":"Hours worked (groupped)","font":{"family":"","size":0,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9},"tickvals":[1.0,2.0,3.0,4.0],"ticktext":["15 hours or less","16 to 30 hours","31 to 48 hours","49 or more hours"]},"yaxis":{"title":{"text":"Mean income","font":{"family":"","size":0,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9}}},"config":{},"frames":null});
</script>
</div>

Instead of the `to_html()`, you can also write an image using this syntax: `.write_image("./data/output/out.png", 800, 600, 1.0).unwrap()`

## Declarative Plotting in Rust with Charton

While Plotly and plotlars demonstrate that high-quality, interactive visualization is entirely possible in Rust, they also represent a design where Rust primarily acts as a data preparation layer, delegating rendering to an external visualization runtime.

In some workflows, however, it can be desirable to treat Rust itself as the primary language for visualization: avoiding external runtimes, temporary files, or language context switches, while keeping a concise, declarative API.

**Charton** explores this alternative design space.

Run the Charton-based examples in this section using:

```bash
cargo run -r --example 5_2_2_plots
```
or evaluated interactively in a Jupyter notebook via `evcxr`.

### Design Philosophy

Charton is a Rust-native plotting library with first-class support for Polars. Its API is inspired by declarative visualization systems such as Altair and Vega-Lite, allowing users to describe *what* should be plotted rather than how to construct individual plot components.

Rather than directly manipulating traces, axes, and layouts, users define mappings between data columns and visual encodings (for example, x- and y-axes), and Charton handles the rest.

This approach is particularly well-suited for exploratory data analysis, where readability, composability, and iteration speed are more important than fine-grained control over rendering primitives.

### A Simple Example

The following example creates a scatter plot directly from a Polars `DataFrame`, rendered entirely in Rust as an SVG file:
```rust
use charton::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the iris dataset (a polars dataframe)
    let df = load_dataset("iris")?;
    println!("{}", df);

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

    Ok(())
}
```
```text
shape: (150, 5)
┌──────────────┬─────────────┬──────────────┬─────────────┬───────────┐
│ sepal_length ┆ sepal_width ┆ petal_length ┆ petal_width ┆ species   │
│ ---          ┆ ---         ┆ ---          ┆ ---         ┆ ---       │
│ f64          ┆ f64         ┆ f64          ┆ f64         ┆ str       │
╞══════════════╪═════════════╪══════════════╪═════════════╪═══════════╡
│ 5.1          ┆ 3.5         ┆ 1.4          ┆ 0.2         ┆ setosa    │
│ 4.9          ┆ 3.0         ┆ 1.4          ┆ 0.2         ┆ setosa    │
│ 4.7          ┆ 3.2         ┆ 1.3          ┆ 0.2         ┆ setosa    │
│ 4.6          ┆ 3.1         ┆ 1.5          ┆ 0.2         ┆ setosa    │
│ 5.0          ┆ 3.6         ┆ 1.4          ┆ 0.2         ┆ setosa    │
│ …            ┆ …           ┆ …            ┆ …           ┆ …         │
│ 6.7          ┆ 3.0         ┆ 5.2          ┆ 2.3         ┆ virginica │
│ 6.3          ┆ 2.5         ┆ 5.0          ┆ 1.9         ┆ virginica │
│ 6.5          ┆ 3.0         ┆ 5.2          ┆ 2.0         ┆ virginica │
│ 6.2          ┆ 3.4         ┆ 5.4          ┆ 2.3         ┆ virginica │
│ 5.9          ┆ 3.0         ┆ 5.1          ┆ 1.8         ┆ virginica │
└──────────────┴─────────────┴──────────────┴─────────────┴───────────┘
```
Open the `single_layer.svg` file in a web browser to see the plot:

<svg width="500" height="400" viewBox="0 0 500 400" xmlns="http://www.w3.org/2000/svg"><rect width="100%" height="100%" fill="white" /><circle cx="168.86874999999998" cy="165" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="154.17625000000004" cy="220" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="139.48375000000001" cy="197.99999999999997" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="132.1375" cy="209" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="161.5225" cy="154" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="190.90750000000003" cy="121" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="132.1375" cy="176.00000000000003" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="161.5225" cy="176.00000000000003" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="117.44500000000004" cy="231" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="154.17625000000004" cy="209" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="190.90750000000003" cy="143" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="146.82999999999998" cy="176.00000000000003" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="146.82999999999998" cy="220" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="110.09875" cy="220" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="220.2925" cy="110" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="212.94625000000002" cy="65.99999999999994" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="190.90750000000003" cy="121" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="168.86874999999998" cy="165" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="212.94625000000002" cy="132" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="168.86874999999998" cy="132" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="190.90750000000003" cy="176.00000000000003" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="168.86874999999998" cy="143" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="132.1375" cy="154" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="168.86874999999998" cy="187.00000000000003" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="146.82999999999998" cy="176.00000000000003" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="161.5225" cy="220" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="161.5225" cy="176.00000000000003" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="176.21500000000003" cy="165" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="176.21500000000003" cy="176.00000000000003" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="139.48375000000001" cy="197.99999999999997" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="146.82999999999998" cy="209" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="190.90750000000003" cy="176.00000000000003" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="176.21500000000003" cy="99.00000000000003" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="198.25375000000003" cy="87.99999999999997" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="154.17625000000004" cy="209" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="161.5225" cy="197.99999999999997" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="198.25375000000003" cy="165" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="154.17625000000004" cy="209" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="117.44500000000004" cy="220" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="168.86874999999998" cy="176.00000000000003" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="161.5225" cy="165" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="124.79125" cy="297" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="117.44500000000004" cy="197.99999999999997" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="161.5225" cy="165" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="168.86874999999998" cy="132" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="146.82999999999998" cy="220" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="168.86874999999998" cy="132" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="132.1375" cy="197.99999999999997" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="183.56125" cy="143" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="161.5225" cy="187.00000000000003" r="3" fill="#1f77b4" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="308.4475" cy="197.99999999999997" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="264.37" cy="197.99999999999997" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="301.10125000000005" cy="209" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="198.25375000000003" cy="297" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="271.71625" cy="242" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="212.94625000000002" cy="242" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="257.02375" cy="187.00000000000003" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="154.17625000000004" cy="286" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="279.0625" cy="231" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="176.21500000000003" cy="252.99999999999997" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="161.5225" cy="330" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="227.63875000000004" cy="220" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="234.985" cy="308" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="242.33124999999998" cy="231" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="205.59999999999997" cy="231" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="286.40875000000005" cy="209" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="205.59999999999997" cy="220" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="220.2925" cy="252.99999999999997" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="249.67750000000004" cy="308" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="205.59999999999997" cy="275" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="227.63875000000004" cy="197.99999999999997" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="242.33124999999998" cy="242" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="257.02375" cy="275" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="242.33124999999998" cy="242" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="264.37" cy="231" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="279.0625" cy="220" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="293.755" cy="242" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="286.40875000000005" cy="220" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="234.985" cy="231" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="212.94625000000002" cy="264" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="198.25375000000003" cy="286" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="198.25375000000003" cy="286" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="220.2925" cy="252.99999999999997" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="234.985" cy="252.99999999999997" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="190.90750000000003" cy="220" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="234.985" cy="176.00000000000003" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="286.40875000000005" cy="209" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="257.02375" cy="297" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="205.59999999999997" cy="220" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="198.25375000000003" cy="275" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="198.25375000000003" cy="264" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="242.33124999999998" cy="220" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="220.2925" cy="264" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="161.5225" cy="297" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="205.59999999999997" cy="252.99999999999997" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="212.94625000000002" cy="220" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="212.94625000000002" cy="231" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="249.67750000000004" cy="231" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="168.86874999999998" cy="275" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="212.94625000000002" cy="242" r="3" fill="#ff7f0e" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="257.02375" cy="187.00000000000003" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="220.2925" cy="252.99999999999997" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="315.79375" cy="220" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="257.02375" cy="231" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="271.71625" cy="220" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="352.525" cy="220" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="154.17625000000004" cy="275" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="330.48625000000004" cy="231" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="286.40875000000005" cy="275" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="323.14000000000004" cy="154" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="271.71625" cy="197.99999999999997" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="264.37" cy="252.99999999999997" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="293.755" cy="220" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="212.94625000000002" cy="275" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="220.2925" cy="242" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="264.37" cy="197.99999999999997" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="271.71625" cy="220" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="359.87125000000003" cy="132" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="359.87125000000003" cy="264" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="234.985" cy="308" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="301.10125000000005" cy="197.99999999999997" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="205.59999999999997" cy="242" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="359.87125000000003" cy="242" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="257.02375" cy="252.99999999999997" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="286.40875000000005" cy="187.00000000000003" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="323.14000000000004" cy="197.99999999999997" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="249.67750000000004" cy="242" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="242.33124999999998" cy="220" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="264.37" cy="242" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="323.14000000000004" cy="220" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="337.83250000000004" cy="242" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="374.56375" cy="132" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="264.37" cy="242" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="257.02375" cy="242" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="242.33124999999998" cy="264" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="359.87125000000003" cy="220" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="257.02375" cy="176.00000000000003" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="264.37" cy="209" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="234.985" cy="220" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="301.10125000000005" cy="209" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="286.40875000000005" cy="209" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="301.10125000000005" cy="209" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="220.2925" cy="252.99999999999997" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="293.755" cy="197.99999999999997" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="286.40875000000005" cy="187.00000000000003" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="286.40875000000005" cy="220" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="257.02375" cy="275" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="271.71625" cy="220" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="249.67750000000004" cy="176.00000000000003" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="227.63875000000004" cy="220" r="3" fill="#2ca02c" stroke="none" stroke-width="0" opacity="1"/>
<line x1="75" y1="340" x2="401.5" y2="340" stroke="black" stroke-width="1"/>
<line x1="88.06" y1="340" x2="88.06" y2="345" stroke="black" stroke-width="1"/>
<text x="88.06" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">4</text>
<line x1="161.5225" y1="340" x2="161.5225" y2="345" stroke="black" stroke-width="1"/>
<text x="161.5225" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">5</text>
<line x1="234.985" y1="340" x2="234.985" y2="345" stroke="black" stroke-width="1"/>
<text x="234.985" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">6</text>
<line x1="308.4475" y1="340" x2="308.4475" y2="345" stroke="black" stroke-width="1"/>
<text x="308.4475" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">7</text>
<line x1="381.91" y1="340" x2="381.91" y2="345" stroke="black" stroke-width="1"/>
<text x="381.91" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">8</text>
<text x="238.25" y="363" font-size="15" font-family="sans-serif" fill="#333" text-anchor="middle" dominant-baseline="text-before-edge">sepal_length</text>
<line x1="75" y1="40" x2="75" y2="340" stroke="black" stroke-width="1"/>
<line x1="70" y1="330" x2="75" y2="330" stroke="black" stroke-width="1"/>
<text x="67" y="330" font-size="13" font-family="sans-serif" fill="#333" text-anchor="end" dominant-baseline="middle">2.0</text>
<line x1="70" y1="275" x2="75" y2="275" stroke="black" stroke-width="1"/>
<text x="67" y="275" font-size="13" font-family="sans-serif" fill="#333" text-anchor="end" dominant-baseline="middle">2.5</text>
<line x1="70" y1="220" x2="75" y2="220" stroke="black" stroke-width="1"/>
<text x="67" y="220" font-size="13" font-family="sans-serif" fill="#333" text-anchor="end" dominant-baseline="middle">3.0</text>
<line x1="70" y1="165" x2="75" y2="165" stroke="black" stroke-width="1"/>
<text x="67" y="165" font-size="13" font-family="sans-serif" fill="#333" text-anchor="end" dominant-baseline="middle">3.5</text>
<line x1="70" y1="110" x2="75" y2="110" stroke="black" stroke-width="1"/>
<text x="67" y="110" font-size="13" font-family="sans-serif" fill="#333" text-anchor="end" dominant-baseline="middle">4.0</text>
<line x1="70" y1="55" x2="75" y2="55" stroke="black" stroke-width="1"/>
<text x="67" y="55" font-size="13" font-family="sans-serif" fill="#333" text-anchor="end" dominant-baseline="middle">4.5</text>
<text x="36.8" y="190" font-size="15" font-family="sans-serif" fill="#333" text-anchor="middle" transform="rotate(-90, 36.8, 190)">sepal_width</text>
<text x="416.5" y="55" font-size="15" font-family="sans-serif" text-anchor="start" font-weight="bold">species</text>
<circle cx="424" cy="75" r="7.5" fill="#1f77b4" stroke="black" stroke-width="0.0" fill-opacity="1"/>
<text x="436.5" y="75" font-size="13" font-family="sans-serif" dominant-baseline="middle">setosa</text>
<circle cx="424" cy="95" r="7.5" fill="#ff7f0e" stroke="black" stroke-width="0.0" fill-opacity="1"/>
<text x="436.5" y="95" font-size="13" font-family="sans-serif" dominant-baseline="middle">versicolor</text>
<circle cx="424" cy="115" r="7.5" fill="#2ca02c" stroke="black" stroke-width="0.0" fill-opacity="1"/>
<text x="436.5" y="115" font-size="13" font-family="sans-serif" dominant-baseline="middle">virginica</text>
</svg>

In contrast to Plotly-based approaches, this example requires no JavaScript runtime or browser engine. The chart is rendered natively and written directly to disk.

### Layered Charts

Charton uses a layered chart model, where multiple visual layers can share a common coordinate system. This makes it straightforward to combine different plot types—such as lines and points—within the same figure.
```rust
let df = df.head(Some(4)).tail(Some(3));
let line = Chart::build(&df)?
    .mark_line()
    .encode((x("sepal_length"), y("sepal_width")))?;

let points = Chart::build(&df)?
    .mark_point()
    .encode((x("sepal_length"), y("sepal_width")))?;

LayeredChart::new()
    .add_layer(line)
    .add_layer(points)
    .save("multiple_layers.svg")?;

Ok(())
```
Open the `multiple_layers.svg` file in a web browser to see the plot:

<svg width="500" height="400" viewBox="0 0 500 400" xmlns="http://www.w3.org/2000/svg"><rect width="100%" height="100%" fill="white" /><path d="M 435.9375 328 L 235.0446428571 58 L 134.5982142857 193" fill="none" stroke="black" stroke-width="2" opacity="1" stroke-linejoin="round" stroke-linecap="round"/>
<circle cx="435.9374999999991" cy="328.0000000000006" r="3" fill="black" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="235.0446428571421" cy="58.000000000000625" r="3" fill="black" stroke="none" stroke-width="0" opacity="1"/>
<circle cx="134.59821428571314" cy="193.0000000000006" r="3" fill="black" stroke="none" stroke-width="0" opacity="1"/>
<line x1="75" y1="340" x2="450" y2="340" stroke="black" stroke-width="1"/>
<line x1="84.375" y1="340" x2="84.375" y2="345" stroke="black" stroke-width="1"/>
<text x="84.375" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">4.55</text>
<line x1="134.59821428571402" y1="340" x2="134.59821428571402" y2="345" stroke="black" stroke-width="1"/>
<text x="134.59821428571402" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">4.60</text>
<line x1="184.82142857142804" y1="340" x2="184.82142857142804" y2="345" stroke="black" stroke-width="1"/>
<text x="184.82142857142804" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">4.65</text>
<line x1="235.04464285714297" y1="340" x2="235.04464285714297" y2="345" stroke="black" stroke-width="1"/>
<text x="235.04464285714297" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">4.70</text>
<line x1="285.267857142857" y1="340" x2="285.267857142857" y2="345" stroke="black" stroke-width="1"/>
<text x="285.267857142857" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">4.75</text>
<line x1="335.49107142857105" y1="340" x2="335.49107142857105" y2="345" stroke="black" stroke-width="1"/>
<text x="335.49107142857105" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">4.80</text>
<line x1="385.7142857142851" y1="340" x2="385.7142857142851" y2="345" stroke="black" stroke-width="1"/>
<text x="385.7142857142851" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">4.85</text>
<line x1="435.9375" y1="340" x2="435.9375" y2="345" stroke="black" stroke-width="1"/>
<text x="435.9375" y="358" font-size="13" font-family="sans-serif" fill="#333" text-anchor="middle">4.90</text>
<text x="262.5" y="363" font-size="15" font-family="sans-serif" fill="#333" text-anchor="middle" dominant-baseline="text-before-edge">sepal_length</text>
<line x1="75" y1="40" x2="75" y2="340" stroke="black" stroke-width="1"/>
<line x1="70" y1="328" x2="75" y2="328" stroke="black" stroke-width="1"/>
<text x="67" y="328" font-size="13" font-family="sans-serif" fill="#333" text-anchor="end" dominant-baseline="middle">3.00</text>
<line x1="70" y1="260.4999999999997" x2="75" y2="260.4999999999997" stroke="black" stroke-width="1"/>
<text x="67" y="260.4999999999997" font-size="13" font-family="sans-serif" fill="#333" text-anchor="end" dominant-baseline="middle">3.05</text>
<line x1="70" y1="193" x2="75" y2="193" stroke="black" stroke-width="1"/>
<text x="67" y="193" font-size="13" font-family="sans-serif" fill="#333" text-anchor="end" dominant-baseline="middle">3.10</text>
<line x1="70" y1="125.50000000000031" x2="75" y2="125.50000000000031" stroke="black" stroke-width="1"/>
<text x="67" y="125.50000000000031" font-size="13" font-family="sans-serif" fill="#333" text-anchor="end" dominant-baseline="middle">3.15</text>
<line x1="70" y1="58" x2="75" y2="58" stroke="black" stroke-width="1"/>
<text x="67" y="58" font-size="13" font-family="sans-serif" fill="#333" text-anchor="end" dominant-baseline="middle">3.20</text>
<text x="29.65" y="190" font-size="15" font-family="sans-serif" fill="#333" text-anchor="middle" transform="rotate(-90, 29.65, 190)">sepal_width</text>
</svg>

This compositional approach mirrors concepts found in grammar-of-graphics systems, while remaining idiomatic to Rust.

### Interactive Use and Web Integration

Charton integrates with the `evcxr` Jupyter kernel, allowing charts to be displayed inline during interactive analysis sessions. The same declarative workflow used for static output can be reused without modification.

In addition, Charton can emit Vega-Lite–compatible JSON specifications. These specifications can be consumed by modern frontend visualization libraries, making it possible to decouple data processing (in Rust) from rendering (in the browser or a web application).

This makes Charton particularly suitable for WebAssembly-based workflows, where Polars data processing and visualization logic can be executed directly in the browser.

### When to Use Charton

Charton is best suited for:
- Rust-first data analysis workflows built around Polars
- Environments where external runtimes are undesirable
- Declarative, composable visualization pipelines
- WebAssembly and frontend-integrated visualization use cases

For users who require the full breadth of Plotly’s interactive feature set, plotlars remains an excellent choice. Charton complements this ecosystem by offering a Rust-native alternative that prioritizes simplicity, performance, and portability.