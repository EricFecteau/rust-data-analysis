# Introduction

This book is a "learn by example" guide to data analysis in Rust. It assumes minimal knowledge of data analysis and minimal familiarity with Rust and its tooling. One of the goals of this book is to demonstrates that Rust is data analysis ready.

# Overview

The [first section](./1_start/index.md) explores concepts related to data analysis in Rust, the crates (libraries) used in the book and how to collect the data necessary for the examples. It also shows how to run these examples in a Jupyter Notebook with a Rust kernel.

The [second section](./2_data/index.md) explains how to read and write various types of data, including larger-than-memory data. This section also focuses on the various locations data can be read from and written to, including local data, cloud-based data and databases. 

The [third section](./3_transformation/index.md) demonstrates how to transform data by adding and removing columns, filtering rows, pivoting the data and joining various data together.

The [fourth section](./4_stats/index.md) shows how do summary statistics, such as counts, totals, means and percentiles, with and without survey weights. It also gives some examples of hypothesis testing. 

The [fifth and last section](./5_pub/index.md) has examples of publication avenues, such as exporting summary statistics to excel, plotting results and writing markdown reports.

# Running the examples

This [repository](https://github.com/EricFecteau/rust-data-analysis) works both as the repository for this `mdBook` and a runnable copy of all the examples in the book. All examples in this book can be [found in the examples folder](https://github.com/EricFecteau/rust-data-analysis/tree/main/examples). You can get access to all the examples by running `git clone "https://github.com/EricFecteau/rust-data-analysis.git"` and then running `cargo run -r --example name_of_example`.

# Specification

While this book uses larger-than-memory data (at least larger than 16 GB of RAM), at no point does the example bring all the data into memory at the same time. The memory spikes at a maximum of around 5 GB or so on some processes.

# Contributing

All contributions are welcome! For any bug, typo or issue, open up an issue on the [GitHub repo](https://github.com/EricFecteau/rust-data-analysis/issues). For any small changes, feel free to do a merge request. For large changes (e.g. a new chapter), please open up an issue first to brainstorm the contribution.