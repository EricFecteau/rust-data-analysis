# Plotting

Plots are the langauge of data analysts. There are mutiple dozens of types of plots, and the majority of them are possible in Rust. [Plotly](https://plotly.com/javascript/), the very popular JavaScript open source graphing library, has a [Rust interface](https://github.com/plotly/plotly.rs)! This chapter will use the [plotlars](https://github.com/alceal/plotlars) crate, a wrapper around the Plotly library that takes polars dataframes as input. This is a great bridge between Polars and Plotly. Plotlars allows you to build all types of graphs, like bar plots, box plots, line plots, pie charts and sankey diagrams. In addition, similar to [ggplot2](https://ggplot2.tidyverse.org/), it follows the [grammar of graphics](https://ggplot2-book.org/mastery.html) approach to creating plots. Those familiar with `ggplot2` will quickly become proficient at using `plotlars`.

Run this code using `cargo run -r --example 5_2_1_plots`.

## Bar Graph

### Setup

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

### Building the bar graph

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

## Line Plot

### Setup

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

### Building the line plot

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