# Environment

This chapter will set you up with Rust for data analysis. It will focus on VSCode, but you can use your Rust-compatible IDE of choice.

## Rust

Install [Rust](https://www.rust-lang.org/tools/install) for your environment. 

Install [VSCode](https://code.visualstudio.com/download) for your environment and [set it up](https://code.visualstudio.com/docs/languages/rust) to work with Rust. Make sure that in this process you install and configure [clippy](https://github.com/rust-lang/rust-clippy) for lints to catch common mistakes. It is also recommend to enable `Editor: Format on Save` with [rustfmt](https://github.com/rust-lang/rustfmt).

To build the crates required for this book, you will also need to install [cmake](https://cmake.org/) and [perl](https://www.perl.org/).

> [!IMPORTANT]
> This book assumes a minimal amount of familiarity with Rust and it's toolkit. The [Rust Book](https://doc.rust-lang.org/stable/book/) (free) and [Programming Rust, 2nd Edition](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/) (paid) are really great resources to learn Rust.

All examples in this book are accompanied with [example scripts](https://github.com/EricFecteau/rust-data-analysis/tree/main/examples). These scripts can be run with `cargo run -r --example 0_0_0_name_of_example`. The first time an example is run, it will take a long time to build all the dependencies, but subsequent runs, including of other examples, will be very quick.

## Data analysis environment

The [evcxr](https://github.com/evcxr/evcxr/blob/main/evcxr/README.md) evaluation context crate has created a [Rust REPL](https://github.com/evcxr/evcxr/blob/main/evcxr_repl/README.md) (Read-Eval-Print loop) and a [Kernel for Jupyter Notebooks](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md). This removes the need for the `main` function. Like R and Python, this allows you to run code in chunks, or re-run a peice of analysis quickly with different options.

To import a dependency in either the REPL or the Jupyter Notebook, you submit a `Cargo.toml` dependency line starting with `:dep` (e.g. `:dep polars = { version = "0.51", features = ["lazy"] }` for `Polars 0.51` with the `lazy` feature). 

> [!WARNING]
> The `evcxr` REPL and the `Jupyter Kernel` have multiple quirks that make multi-step analysis impossible to run in chunks (e.g. failure to infer types without knowing the future code). The original goal for this book was to include `evcxr` and `jupyter kernel` code for each examples (e.g. add a `:dep` line for each script example in the book), but so many of the examples failed to run that it was removed. Caution should be used when using this analysis method. The following was kept in the event that someone was interested. 

### REPL

Like with R, Python and Julia, the Rust REPL allows you to run code in chunks, without having the re-run the whole script. You can install the Rust REPL from [the pre-built binary](https://github.com/evcxr/evcxr/blob/main/evcxr_repl/README.md).

To make it work in VSCode, you have to install the [multi-command](https://marketplace.visualstudio.com/items?itemName=ryuta46.multi-command) VSCode extension and edit the [keybindings.json](https://code.visualstudio.com/docs/configure/keybindings) with the following code. This will make it possible to highlight code and submit it to the Rust REPL with `ctrl + enter`.

```json
[
    {
        "key": "ctrl+enter", // or ctrl+enter
        "command": "extension.multiCommand.execute",
        "args": {
            "sequence": [
                "workbench.action.terminal.runSelectedText", // run line
                {
                    "command": "workbench.action.terminal.sendSequence",
                    "args": {
                        "text": "\n"
                    }
                },
                "workbench.action.focusActiveEditorGroup", // shift focus back to editor
                "cursorDown" // jump to next line so you can spam cmd+enter
            ]
        },
        "when": "editorTextFocus && editorLangId == 'rust'",
    }
]
```
To submit code, start `evcxr` in the terminal window. 

### Jupyter Notebooks

Like with R, Python and Julia, the Rust Jupyter Kernel allows you to run Rust in a Jupyter Notebook. This allows you to share interactive notebook that contains code, visualizations, and outputs.

To start with Rust in a Jupyter notebook, install [Jupyter notebook](https://jupyter.org/install) and the [Evcxr Jupyter Kernel](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md). Start Jupyter Notebook with `jupyter notebook`. It will open up in your browser. Once started, look for the "New" menu on the right and select "Rust". You will then be able to submit blocks Rust code. 
