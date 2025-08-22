# Plotting

Plots are the langauge of data analysts. There are mutiple dozens of types of plots, and the majority of them are possible in Rust. [Plotly](https://plotly.com/javascript/), the very popular JavaScript open source graphing library, has a [Rust interface](https://github.com/plotly/plotly.rs)! This chapter will use the [plotlars](https://github.com/alceal/plotlars), a wrapper around the Plotly library that takes polars dataframes as input. This is a great bridge between Polars and Plotly. Plotlars allows you to build X, Y, Z. In addition, similar to [ggplot2](https://ggplot2.tidyverse.org/), it follows the [grammar of graphics](https://ggplot2-book.org/mastery.html) approach to creating plots. Those familiar with `ggplot2` will quickly become proficient using `plotlars`.

## Bar Graph

### Setup

First, lets get some summary statistics to output as a plot. Here is a table of the mean hourly wage by gender and province:

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
│ Women+ ┆ SK   ┆ 25.7         │
│ Men+   ┆ NS   ┆ 25.98        │
│ Women+ ┆ NL   ┆ 24.91        │
│ Men+   ┆ BC   ┆ 31.0         │
│ Men+   ┆ NB   ┆ 25.21        │
│ …      ┆ …    ┆ …            │
│ Women+ ┆ MB   ┆ 23.4         │
│ Women+ ┆ ON   ┆ 26.8         │
│ Men+   ┆ PE   ┆ 23.83        │
│ Women+ ┆ AB   ┆ 27.24        │
│ Women+ ┆ QC   ┆ 25.25        │
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
    await Plotly.newPlot(graph_div, {"data":[{"type":"bar","x":["NS","BC","NB","ON","MB","QC","NL","SK","AB","PE"],"y":[25.98,31.0,25.21,30.86,26.45,28.11,29.76,30.04,34.56,23.83],"name":"Men+","orientation":"v","marker":{"color":"rgb(255, 127, 80)"}},{"type":"bar","x":["SK","NL","NB","BC","NS","PE","MB","ON","AB","QC"],"y":[25.7,24.91,22.66,25.84,23.13,22.96,23.4,26.8,27.24,25.25],"name":"Women+","orientation":"v","marker":{"color":"rgb(64, 224, 208)"}}],"layout":{"title":{"text":"Hourly wages by gender and province","font":{"family":"Arial","size":18,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9},"legend":{"orientation":"h","x":0.4,"y":1.0,"title":{"text":"gender","font":{"family":"Arial","size":15,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9}},"barmode":"group"},"config":{},"frames":null});
</script>
</div>

Instead of the `to_html()`, you can also write an image using this syntax: `.write_image("./data/output/out.png", 800, 600, 1.0).unwrap()`

## Scatter Plot

### Setup




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
│ Women+ ┆ SK   ┆ 25.7         │
│ Men+   ┆ NS   ┆ 25.98        │
│ Women+ ┆ NL   ┆ 24.91        │
│ Men+   ┆ BC   ┆ 31.0         │
│ Men+   ┆ NB   ┆ 25.21        │
│ …      ┆ …    ┆ …            │
│ Women+ ┆ MB   ┆ 23.4         │
│ Women+ ┆ ON   ┆ 26.8         │
│ Men+   ┆ PE   ┆ 23.83        │
│ Women+ ┆ AB   ┆ 27.24        │
│ Women+ ┆ QC   ┆ 25.25        │
└────────┴──────┴──────────────┘
```


## EVCXR / Jupyter!