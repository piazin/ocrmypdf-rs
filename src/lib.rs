use std::process::Command;

pub struct OcrMyPdf {
    args: Vec<String>,
    input_path: String,
    output_path: String,
}

impl OcrMyPdf {
    pub fn new(
        args: Option<Vec<String>>,
        input_path: Option<String>,
        output_path: Option<String>,
    ) -> Self {
        let args = args.unwrap_or(vec![]);
        let input_path = input_path.unwrap_or(String::new());
        let output_path = output_path.unwrap_or(String::new());
        OcrMyPdf {
            args,
            input_path,
            output_path,
        }
    }
}

pub trait Ocr {
    fn execute(&mut self);
    fn set_args(&mut self, args: Vec<String>) -> &mut Self;
    fn set_input_path(&mut self, path: String) -> &mut Self;
    fn set_output_path(&mut self, path: String) -> &mut Self;
}

impl Ocr for OcrMyPdf {
    fn execute(&mut self) {
        let _ = execute_ocr(&self.args, &self.input_path, &self.output_path);
        println!("{:?}, {}, {}", self.args, self.input_path, self.output_path);
    }

    fn set_args(&mut self, args: Vec<String>) -> &mut Self {
        self.args = args;
        self
    }

    fn set_input_path(&mut self, input_path: String) -> &mut Self {
        self.input_path = input_path;
        self
    }

    fn set_output_path(&mut self, output_path: String) -> &mut Self {
        self.output_path = output_path;
        self
    }
}

fn execute_ocr(
    args: &Vec<String>,
    input_path: &String,
    output_path: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("ocrmypdf");
    cmd.arg(input_path).arg(output_path).args(args);

    let _ = cmd.output().expect("error to parse");

    Ok(())
}