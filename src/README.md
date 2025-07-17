# Introduction

This book is a complete, learn by example, guide to data analysis in Rust. It assumes minimal knowledge of data analysis and minimal familiarity with Rust and it's tooling. One of the goals of this book is to demonstrates that Rust is data analysis ready.

# Overview

The [first section](./1_start/index.md) explores concepts related to data analysis in Rust, the crates (libraries) used in the book and how to collect the data necessary for the examples in the book. It also shows the ability to run the examples in this book in a Jupyter Notebook with a Rust kernel.

The [second section](./2_data/index.md) explains how to read and write various types of data, including larger-than-memory data. This section also focuses on the various locations data can be read from and written to, including local data, cloud-based data and databases. 

The [third section](./3_transformation/index.md) demonstrates how to transform data by adding and removing columns, filtering rows, pivoting the data and joining various data together.

The [fourth section](./4_stats/index.md) shows how do summary statistics, such as counts, totals, means and percentiles, with and without survey weights. It also gives some examples of hypothesis testing. 

The [fifth and last section](./5_pub/index.md) has examples of publication avenues, such as exporting summary statistics to excel, plotting the results, writing reports and creating interactive BI reports.

# Specification

All code in this book was run on a 16 GB RAM computer, but the examples focus on a much smaller 