use spinners::{Spinner, Spinners};
use std::process::Command;
use std::{env, fs};

static CUSTOM_ARGS: [&'static str; 1] = ["--return-text"];
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

#[allow(dead_code)]
pub trait Ocr {
    fn execute(&mut self) -> Option<String>;
    fn set_args(&mut self, args: Vec<String>) -> &mut Self;
    fn set_input_path(&mut self, path: String) -> &mut Self;
    fn set_output_path(&mut self, path: String) -> &mut Self;
}

impl Ocr for OcrMyPdf {
    fn execute(&mut self) -> Option<String> {
        let result = execute_ocr(&self.args, &self.input_path, &self.output_path);
        result.ok().flatten()
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

/*todo
return text
disabled log options
parse args
x input required output optional replace file
 */

fn execute_ocr(
    args: &Vec<String>,
    input_path: &String,
    output_path: &String,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("ocrmypdf");
    let (parsed_args, parsed_custom_args) = parse_args(args);
    let (input, output, return_txt, tmp) =
        parse_params(input_path, output_path, parsed_custom_args).expect("error parse params");

    let return_text_command = return_txt.unwrap_or("".into());
    cmd.arg(input).arg(output).args(parsed_args);
    if return_text_command.chars().count() > 0 {
        cmd.args(return_text_command.split(' '));
    }

    let mut spinner = Spinner::new(Spinners::Runner, "Running OCR...".into());
    let output = cmd.output().expect("Failed to execute ocrmypdf");

    let mut return_text: String = "".into();
    if output.status.success() && return_text_command.chars().count() > 0 {
        return_text = read_text_file(&tmp);
    }

    if output.status.success() {
        spinner.stop_with_symbol("✔");
    } else {
        spinner.stop_with_message(format!(
            "✘ Failed to run ocrmypdf with error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    if return_text.chars().count() > 0 {
        Ok(Some(return_text))
    } else {
        Ok(None)
    }
}

fn parse_args(args: &Vec<String>) -> (Vec<&String>, Vec<&String>) {
    let mut parsed_custom_args: Vec<&String> = vec![];

    let parsed_args: Vec<&String> = args
        .into_iter()
        .filter(|&a| {
            if CUSTOM_ARGS.iter().any(|&c_a| c_a == a) {
                parsed_custom_args.push(a);
                false
            } else {
                true
            }
        })
        .collect();

    (parsed_args, parsed_custom_args)
}

#[derive(Debug)]
pub enum ErrorValidationParams {
    InvalidInput,
}

pub fn parse_params(
    input_path: &String,
    output_path: &String,
    custom_args: Vec<&String>,
) -> Result<(String, String, Option<String>, String), ErrorValidationParams> {
    let (input, mut output, mut return_text): (String, String, Option<String>) =
        (input_path.clone(), output_path.clone(), None);

    if input_path.trim().chars().count() <= 0 {
        return Err(ErrorValidationParams::InvalidInput);
    }

    if output_path.trim().chars().count() <= 0 {
        output = input.clone();
    }

    let tmp_dir = env::temp_dir();
    let tmp_file_path = format!(
        "{}/ocrmypdf_output.txt",
        String::from(tmp_dir.to_string_lossy())
    );

    if custom_args.iter().any(|a| **a == CUSTOM_ARGS[0]) {
        return_text = Some(format!("--sidecar {}", tmp_file_path));
    }

    Ok((input, output, return_text, tmp_file_path))
}

pub fn read_text_file(tmp_file_path: &String) -> String {
    let content = fs::read_to_string(tmp_file_path).expect("error to read file");
    fs::remove_file(tmp_file_path).expect("error to remove tmp file");
    content
}
