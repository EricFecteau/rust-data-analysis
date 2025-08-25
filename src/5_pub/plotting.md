# Plotting

Plots are the langauge of data analysts. There are mutiple dozens of types of plots, and the majority of them are possible in Rust. [Plotly](https://plotly.com/javascript/), the very popular JavaScript open source graphing library, has a [Rust interface](https://github.com/plotly/plotly.rs)! This chapter will use the [plotlars](https://github.com/alceal/plotlars), a wrapper around the Plotly library that takes polars dataframes as input. This is a great bridge between Polars and Plotly. Plotlars allows you to build X, Y, Z. In addition, similar to [ggplot2](https://ggplot2.tidyverse.org/), it follows the [grammar of graphics](https://ggplot2-book.org/mastery.html) approach to creating plots. Those familiar with `ggplot2` will quickly become proficient using `plotlars`.

## Bar Graph

### Setup

Lets get some summary statistics to output as a bar graph. Here is a table of the mean hourly wage by gender and province:

```rust
=== Rust 5_2_0_plots evcxr
=== Rust 5_2_0_plots imports
=== Rust 5_2_0_plots block_1
```

```
shape: (20, 3)
┌────────┬──────┬──────────────┐
│ gender ┆ prov ┆ hourly_wages │
│ ---    ┆ ---  ┆ ---          │
│ str    ┆ str  ┆ f64          │
╞════════╪══════╪══════════════╡
│ Men+   ┆ NL   ┆ 29.76        │
│ Men+   ┆ PE   ┆ 23.83        │
│ Men+   ┆ NS   ┆ 25.98        │
│ Men+   ┆ NB   ┆ 25.21        │
│ Men+   ┆ QC   ┆ 28.11        │
│ …      ┆ …    ┆ …            │
│ Women+ ┆ ON   ┆ 26.8         │
│ Women+ ┆ MB   ┆ 23.4         │
│ Women+ ┆ SK   ┆ 25.7         │
│ Women+ ┆ AB   ┆ 27.24        │
│ Women+ ┆ BC   ┆ 25.84        │
└────────┴──────┴──────────────┘
```

### Building the bar graph

To build a graph, you start with the data, you give it an `x` axis and a `y` axis, a `group` if you have groups and then various options, like `x_title`, `y_title`, `plot_title`, `size` for text, etc. Most functions are self-explanatory, but they are described in the documentation of the [bar graph](https://docs.rs/plotlars/latest/plotlars/struct.BarPlot.html).

```Rust
=== Rust 5_2_0_plots block_2
```

In this example, `to_html()` was used to imbed in this page to be interactive. You can save it to a `.html` file:

```Rust
=== Rust 5_2_0_plots block_3
```

Opening this file, will give you this interactive bar chart:

<div>
<script src="https://cdn.plot.ly/plotly-2.12.1.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"></script>
<script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.0/es5/tex-mml-chtml.js"></script>
    
<div id="plotly-html-element" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script type="module">
    const graph_div = document.getElementById("plotly-html-element");
    await Plotly.newPlot(graph_div, {"data":[{"type":"bar","x":["NL","PE","NS","NB","QC","ON","MB","SK","AB","BC"],"y":[29.76,23.83,25.98,25.21,28.11,30.86,26.45,30.04,34.56,31.0],"name":"Men+","orientation":"v","marker":{"color":"rgb(255, 127, 80)"}},{"type":"bar","x":["NL","PE","NS","NB","QC","ON","MB","SK","AB","BC"],"y":[24.91,22.96,23.13,22.66,25.25,26.8,23.4,25.7,27.24,25.84],"name":"Women+","orientation":"v","marker":{"color":"rgb(64, 224, 208)"}}],"layout":{"title":{"text":"Hourly wages by gender and province","font":{"family":"Arial","size":18,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9},"legend":{"orientation":"h","x":0.4,"y":1.0,"title":{"text":"Gender","font":{"family":"Arial","size":15,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9}},"xaxis":{"title":{"text":"Province","font":{"family":"Arial","size":15,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9}},"yaxis":{"title":{"text":"Mean hourly wage","font":{"family":"Arial","size":15,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9}},"barmode":"group"},"config":{},"frames":null});
</script>
</div>

Instead of the `to_html()`, you can also write an image using this syntax: `.write_image("./data/output/out.png", 800, 600, 1.0).unwrap()`

## Line Plot

### Setup

Lets also get some summary statistics to output as a scatter plot. Here is a table of the mean hourly wage by gender and job tenure (months), pivoted on gender:


```rust
=== Rust 5_2_0_plots block_4
```

```
shape: (240, 3)
┌────────┬───────┬────────┐
│ tenure ┆ Men+  ┆ Women+ │
│ ---    ┆ ---   ┆ ---    │
│ i64    ┆ f64   ┆ f64    │
╞════════╪═══════╪════════╡
│ 1      ┆ 21.23 ┆ 18.02  │
│ 2      ┆ 21.5  ┆ 18.27  │
│ 3      ┆ 21.88 ┆ 18.54  │
│ 4      ┆ 22.28 ┆ 18.95  │
│ 5      ┆ 22.73 ┆ 19.2   │
│ …      ┆ …     ┆ …      │
│ 236    ┆ 36.0  ┆ 32.06  │
│ 237    ┆ 36.29 ┆ 32.0   │
│ 238    ┆ 36.2  ┆ 31.97  │
│ 239    ┆ 36.26 ┆ 31.78  │
│ 240    ┆ 36.14 ┆ 32.14  │
└────────┴───────┴────────┘
```

### Building the line plot

Similar to the bar graph, for the line plot, you start with the data, you give it an `x` axis and a `y` axis (and here, you add another `y` axis with `additional_lines` to add a new line to the line plot), and then various options, like `x_title`, `y_title`, `plot_title`, `size` for text, etc. Most functions are self-explanatory, but they are described in the documentation of the [line plot](https://docs.rs/plotlars/latest/plotlars/struct.LinePlot.html).

```Rust
=== Rust 5_2_0_plots block_5
```

In this example, `to_html()` was used to imbed in this page to be interactive. You can save it to a `.html` file:

```Rust
=== Rust 5_2_0_plots block_6
```

Opening this file, will give you this interactive bar chart:


## EVCXR / Jupyter!