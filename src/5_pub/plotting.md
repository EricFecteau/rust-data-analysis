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

<div id="plotly-html-element-1" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script type="module">
const graph_div = document.getElementById("plotly-html-element-1");
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

<div>
<meta charset="utf-8" />
<script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"></script>
<script src="https://cdn.plot.ly/plotly-3.0.1.min.js"></script>
    
<div id="plotly-html-element-2" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script type="module">
const graph_div = document.getElementById("plotly-html-element-2");
await Plotly.newPlot(graph_div, {"data":[{"type":"scatter","name":"Men+","x":[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0,20.0,21.0,22.0,23.0,24.0,25.0,26.0,27.0,28.0,29.0,30.0,31.0,32.0,33.0,34.0,35.0,36.0,37.0,38.0,39.0,40.0,41.0,42.0,43.0,44.0,45.0,46.0,47.0,48.0,49.0,50.0,51.0,52.0,53.0,54.0,55.0,56.0,57.0,58.0,59.0,60.0,61.0,62.0,63.0,64.0,65.0,66.0,67.0,68.0,69.0,70.0,71.0,72.0,73.0,74.0,75.0,76.0,77.0,78.0,79.0,80.0,81.0,82.0,83.0,84.0,85.0,86.0,87.0,88.0,89.0,90.0,91.0,92.0,93.0,94.0,95.0,96.0,97.0,98.0,99.0,100.0,101.0,102.0,103.0,104.0,105.0,106.0,107.0,108.0,109.0,110.0,111.0,112.0,113.0,114.0,115.0,116.0,117.0,118.0,119.0,120.0,121.0,122.0,123.0,124.0,125.0,126.0,127.0,128.0,129.0,130.0,131.0,132.0,133.0,134.0,135.0,136.0,137.0,138.0,139.0,140.0,141.0,142.0,143.0,144.0,145.0,146.0,147.0,148.0,149.0,150.0,151.0,152.0,153.0,154.0,155.0,156.0,157.0,158.0,159.0,160.0,161.0,162.0,163.0,164.0,165.0,166.0,167.0,168.0,169.0,170.0,171.0,172.0,173.0,174.0,175.0,176.0,177.0,178.0,179.0,180.0,181.0,182.0,183.0,184.0,185.0,186.0,187.0,188.0,189.0,190.0,191.0,192.0,193.0,194.0,195.0,196.0,197.0,198.0,199.0,200.0,201.0,202.0,203.0,204.0,205.0,206.0,207.0,208.0,209.0,210.0,211.0,212.0,213.0,214.0,215.0,216.0,217.0,218.0,219.0,220.0,221.0,222.0,223.0,224.0,225.0,226.0,227.0,228.0,229.0,230.0,231.0,232.0,233.0,234.0,235.0,236.0,237.0,238.0,239.0,240.0],"y":[21.23,21.5,21.88,22.28,22.73,22.99,23.24,23.5,23.8,24.05,24.35,24.47,24.23,24.21,24.46,24.81,25.19,25.36,25.7,25.93,26.18,26.3,26.5,26.6,26.45,26.55,26.61,26.83,26.89,26.99,27.11,27.34,27.49,27.61,27.62,27.81,27.58,27.56,27.73,27.82,27.9,27.91,28.17,28.48,28.66,28.73,28.86,29.05,28.74,28.8,28.73,28.9,29.09,29.38,29.47,29.82,29.83,30.08,30.05,30.0,29.9,29.88,30.04,30.08,30.15,30.21,30.64,30.97,31.07,31.0,31.25,31.23,31.02,31.06,31.11,31.38,31.57,31.66,31.88,32.0,32.08,31.98,32.17,32.27,32.2,32.25,31.95,32.18,32.01,32.16,32.46,32.54,32.73,32.89,32.85,32.66,32.59,32.33,32.32,32.54,32.78,32.76,32.81,33.06,33.0,32.92,33.24,33.29,33.23,33.23,33.49,33.5,33.46,33.49,33.35,33.5,33.71,33.54,33.52,33.51,33.29,33.06,32.9,33.02,32.96,33.02,33.34,33.38,33.71,33.87,33.95,34.23,34.2,34.04,34.31,34.38,34.52,34.79,34.88,35.1,35.22,34.82,34.71,34.86,34.57,34.68,34.68,34.88,35.03,34.97,35.12,35.21,35.33,35.29,35.51,35.53,35.33,35.12,35.31,35.22,35.25,35.29,35.24,35.6,35.64,35.81,35.8,35.5,35.46,35.5,35.59,35.76,35.71,35.67,35.54,35.77,35.73,35.69,35.52,35.62,35.2,35.05,34.9,34.97,34.68,35.0,35.32,35.41,35.56,35.81,35.81,35.87,35.93,36.14,35.77,36.2,36.14,36.58,36.7,36.75,36.71,36.58,36.93,36.65,36.35,36.82,36.53,36.51,36.61,36.8,36.47,36.2,36.47,36.46,36.15,36.4,36.41,36.37,36.28,36.33,36.47,36.34,36.62,36.62,36.94,36.92,36.55,36.67,37.05,36.93,37.07,36.71,36.82,36.56,36.61,36.0,36.29,36.2,36.26,36.14],"marker":{"size":12,"color":"rgb(178, 34, 34)"},"line":{}},{"type":"scatter","name":"Women+","x":[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0,20.0,21.0,22.0,23.0,24.0,25.0,26.0,27.0,28.0,29.0,30.0,31.0,32.0,33.0,34.0,35.0,36.0,37.0,38.0,39.0,40.0,41.0,42.0,43.0,44.0,45.0,46.0,47.0,48.0,49.0,50.0,51.0,52.0,53.0,54.0,55.0,56.0,57.0,58.0,59.0,60.0,61.0,62.0,63.0,64.0,65.0,66.0,67.0,68.0,69.0,70.0,71.0,72.0,73.0,74.0,75.0,76.0,77.0,78.0,79.0,80.0,81.0,82.0,83.0,84.0,85.0,86.0,87.0,88.0,89.0,90.0,91.0,92.0,93.0,94.0,95.0,96.0,97.0,98.0,99.0,100.0,101.0,102.0,103.0,104.0,105.0,106.0,107.0,108.0,109.0,110.0,111.0,112.0,113.0,114.0,115.0,116.0,117.0,118.0,119.0,120.0,121.0,122.0,123.0,124.0,125.0,126.0,127.0,128.0,129.0,130.0,131.0,132.0,133.0,134.0,135.0,136.0,137.0,138.0,139.0,140.0,141.0,142.0,143.0,144.0,145.0,146.0,147.0,148.0,149.0,150.0,151.0,152.0,153.0,154.0,155.0,156.0,157.0,158.0,159.0,160.0,161.0,162.0,163.0,164.0,165.0,166.0,167.0,168.0,169.0,170.0,171.0,172.0,173.0,174.0,175.0,176.0,177.0,178.0,179.0,180.0,181.0,182.0,183.0,184.0,185.0,186.0,187.0,188.0,189.0,190.0,191.0,192.0,193.0,194.0,195.0,196.0,197.0,198.0,199.0,200.0,201.0,202.0,203.0,204.0,205.0,206.0,207.0,208.0,209.0,210.0,211.0,212.0,213.0,214.0,215.0,216.0,217.0,218.0,219.0,220.0,221.0,222.0,223.0,224.0,225.0,226.0,227.0,228.0,229.0,230.0,231.0,232.0,233.0,234.0,235.0,236.0,237.0,238.0,239.0,240.0],"y":[18.02,18.27,18.54,18.95,19.2,19.44,19.67,19.82,20.0,20.24,20.48,20.64,20.42,20.45,20.7,20.95,21.07,21.3,21.42,21.6,21.76,21.92,22.15,22.31,22.3,22.34,22.46,22.5,22.63,22.71,22.79,23.0,23.02,23.18,23.22,23.33,23.37,23.35,23.41,23.63,23.75,23.79,23.96,23.99,24.02,24.03,24.06,24.25,24.27,24.37,24.47,24.63,24.74,24.92,24.97,25.18,25.21,25.26,25.37,25.38,25.38,25.38,25.46,25.7,25.62,25.68,25.72,25.71,25.77,25.96,25.99,26.26,26.41,26.4,26.63,26.78,26.81,26.84,27.01,26.94,26.91,26.81,26.99,27.05,26.97,27.07,27.22,27.35,27.46,27.53,27.46,27.61,27.62,27.72,27.92,27.83,27.74,27.64,27.62,27.69,27.86,27.85,28.07,28.08,28.14,28.4,28.35,28.63,28.58,28.68,28.55,28.6,28.58,28.65,28.55,28.48,28.42,28.41,28.31,28.34,28.25,28.14,28.06,28.05,28.17,28.47,28.72,28.88,28.91,29.22,29.36,29.31,29.27,29.17,29.34,29.21,29.38,29.51,29.55,29.76,29.73,29.97,29.97,30.13,29.91,29.77,29.64,29.88,29.53,29.82,29.72,29.76,29.75,29.86,30.12,30.15,30.06,30.04,30.1,30.07,30.1,30.15,30.2,30.23,30.27,30.54,30.84,30.83,30.83,31.04,31.08,31.07,31.07,31.06,31.01,30.87,30.91,30.62,30.66,30.76,30.72,30.84,30.74,30.76,30.46,30.4,30.58,30.71,30.91,30.93,31.32,31.45,31.24,31.26,31.62,31.71,31.76,31.72,32.13,31.93,31.83,32.25,32.12,32.28,32.11,32.08,32.03,32.22,32.01,31.97,31.97,31.95,31.96,32.01,31.89,31.82,31.75,31.99,31.8,31.95,31.84,31.94,31.8,31.78,32.25,32.38,32.19,32.32,32.65,32.77,32.13,32.09,32.27,32.22,32.35,32.06,32.0,31.97,31.78,32.14],"marker":{"size":12,"color":"rgb(65, 105, 225)"},"line":{}}],"layout":{"title":{"text":"Mean hourly wage by job tenure and gender","font":{"family":"","size":0,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9},"legend":{"title":{"text":"Gender","font":{"family":"","size":0,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9}},"xaxis":{"title":{"text":"Mean hourly wage","font":{"family":"","size":0,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9}},"yaxis":{"title":{"text":"Job tenure (months)","font":{"family":"","size":0,"color":"rgb(0, 0, 0)"},"x":0.5,"y":0.9}}},"config":{},"frames":null});
</script>
</div>

Instead of the `to_html()`, you can also write an image using this syntax: `.write_image("./data/output/out.png", 800, 600, 1.0).unwrap()`

## EVCXR / Jupyter!