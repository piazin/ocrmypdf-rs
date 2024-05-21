# ocrmypdf-rs

A Rust library that adds “layers” of text to images in PDFs, making scanned image PDFs searchable using [ocrmypdf](https://ocrmypdf.readthedocs.io/en/latest/introduction.html), which is a Python application and library.

## Prerequisites

For everything to work correctly, you need to have it installed on your OS [ocrmypdf](https://ocrmypdf.readthedocs.io/en/latest/installation.html).

#### Example

Debian or Ubuntu users can simply use the following:

```
sudo apt install ocrmypdf
```

For more information on how to install on different OS, see the installation [documents](https://ocrmypdf.readthedocs.io/en/latest/installation.html).

## Installation

Install ocrmypdf-rs with cargo;

```Cargo.toml
[dependencies]
ocrmypdf-rs = "0.0.7"
```

## Usage/Examples

#### Basic example [ref](https://ocrmypdf.readthedocs.io/en/latest/cookbook.html#add-an-ocr-layer-and-convert-to-pdf-a).

```rust
use ocrmypdf_rs::{Ocr, OcrMyPdf};

fn main() {
    let mut ocr = OcrMyPdf::new(None, None, None);

    ocr.set_input_path("input.pdf".into())
        .set_output_path("output.pdf".into())
        .set_args(vec!["--force-ocr".into()])
        .execute();
}
```

### new method

When instantiating the `OcrMyPdf` structure it is possible to pass the following parameters:

- `args: Option<Vec<String>>` see about arguments in [documentation](https://ocrmypdf.readthedocs.io/en/latest/cookbook.html#basic-examples)
- `input_path: Option<String>` input pdf path
- `output_path: Option<String>` output pdf path

> [!TIP]
> 💡 If the input_path or output_path fields are provided, there is no need to provide them at runtime.

```rust
use ocrmypdf_rs::{Ocr, OcrMyPdf};

fn main() {
    let args: Vec<String> = vec!["-l por".into()];
    let input_path = "input.pdf";
    let output_path = "output.pdf";

    let mut ocr = OcrMyPdf::new(
        Some(args),
        Some(input_path.into()),
        Some(output_path.into()),
    );

    ocr.execute();
}
```

> [!NOTE]
> The `-l por` args to work requires the additional selected language to be installed, see how [install](https://ocrmypdf.readthedocs.io/en/latest/languages.html);
