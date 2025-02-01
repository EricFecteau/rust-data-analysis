# Environment

This chapter will set you up with Rust for data analysis. It will focus on VSCode, but you can use your Rust-compatible IDE of choice.

## Rust

Install [Rust](https://www.rust-lang.org/tools/install) for your environment (rustc, cargo, rustup). 

Install [VSCode](https://code.visualstudio.com/download) for your environment and [set it up](https://code.visualstudio.com/docs/languages/rust) to work with Rust. Make sure that in this process you install and configure [clippy](https://github.com/rust-lang/rust-clippy) for lints to catch common mistakes. I also recommend `Editor: Format on Save` with [rustfmt](https://github.com/rust-lang/rustfmt). It is also useful to install and configure the rust debugging. 

> [!IMPORTANT]
> This book assumes some familiarity with Rust and it's toolkit. The [Rust Book](https://doc.rust-lang.org/stable/book/) (free) and [Programming Rust, 2nd Edition](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/) (paid) are really great resources to learn Rust.

All examples in this book are accompanied with [example scripts](https://github.com/EricFecteau/rust-data-analysis/tree/main/examples). These scripts can be run with `cargo run -r --example name_of_script`.

## Data Analysis Environment

Rust is a compiled programming language, which has significant advantages, at the disadvantage of quick data exploration (e.g. quickly querying the data multiple times to build the final query for your analysis). Thankfully, the [evcxr](https://github.com/evcxr/evcxr/blob/main/evcxr/README.md) evaluation context crate has created a [Rust REPL](https://github.com/evcxr/evcxr/blob/main/evcxr_repl/README.md) (Read-Eval-Print loop) and a [Kernel for Jupyter Notebooks]((https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md)). This removes the need for the `main` function.


> [!NOTE]
> To import a dependency in either the REPL or the Jupyter Notebook, you submit a `Cargo.toml` dependency line starting with `:dep` (e.g. `:dep polars = { version = "0.46", features = ["lazy"] }` for Polars 0.45 with the `lazy` feature). 
>
> All [example scripts](https://github.com/EricFecteau/rust-data-analysis/tree/main/examples) in this book start with a commented out `:dep` block that can be uncommented to run the script in the Rust REPL. 

### REPL

Like with Python, R and Julia, the Rust REPL allows you to query your data over and over, changing your parameters, analysis type and variables, without having the re-run the whole script. You can install the Rust REPL from [the pre-built binary](https://github.com/evcxr/evcxr/blob/main/evcxr_repl/README.md).

To make it work in VSCode, in a similar way as R, Python and Julia, you have to install the [multi-command](https://marketplace.visualstudio.com/items?itemName=ryuta46.multi-command) VSCode extension and edit the [keybindings.json](https://code.visualstudio.com/docs/getstarted/keybindings#_advanced-customization) with the following code. This will make it possible to highlight code and submit it to the Rust REPL with `ctrl+enter`.

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

Like with Python, R and Julia, the Rust Jupyter Kernel allows you to run Rust in a Jupyter Notebook. This allows you to do quick development by running similar code multiple time and also to create and share interactive notebook that contains code, visualizations, and outputs.

To start with Rust in a Jupyter notebook, install [Jupyter notebook](https://jupyter.org/install) and the [Evcxr Jupyter Kernel](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md). Start Jupyter Notebook with `jupyter notebook`. It will open up in your browser. Once started, look for the "New" menu on the right and from it, select "Rust". You will then be able to submit blocks Rust code. 
