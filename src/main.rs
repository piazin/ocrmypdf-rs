use ocrmypdf_rs::{Ocr, OcrMyPdf};

fn main() {
    let args: Vec<String> = vec!["--force-ocr".into(), "--return-text".into()];
    let input_path = "pdf.test.pdf".into();
    let output_path = "output.pdf".into();
    let mut ocr_my_pdf = OcrMyPdf::new(Some(args), None, Some(output_path));

    ocr_my_pdf.set_input_path(input_path).execute();
}

/*
fn main() {}
*/
