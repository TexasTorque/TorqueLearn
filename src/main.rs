use std::{ffi::OsString, fs::{self, File, copy, create_dir_all}, io::Write, path::PathBuf};

use copy_dir::copy_dir;
use glob::glob;
use markdown::file_to_html;
use regex::Regex;

const FORMAT_HTML: &'static str = include_str!("../layout/format.html");

fn main() {

    // Remove deploy directory
    fs::remove_dir_all("deploy/").expect("Failed to remove deploy directory!");
    
    // Create deploy directory
    fs::create_dir("deploy/").expect("Failed to create a new deploy directory!");

    // Copy non-page files
    for entry in glob("layout/*.html").expect("Failed to read glob pattern!") {
        match entry {
            Ok(path) => {
                let file_name : &str = path.file_name().expect("Failed to get file name!").to_str().expect("Failed converting OS String to &str!"); 
                if file_name != "format.html" {
                    copy(path.clone(), ["deploy/", file_name].join("")).expect("Failed copying HTML file over");
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }

    // Bring over static folder
    copy_dir("layout/static/", "deploy/static").expect("Failed copying static!");

    // Read all MDs
    for entry in glob("pages/**/*.md").expect("Failed to read glob pattern!") {
        match entry {
            Ok(path) => handle_md(path),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn handle_md(path: PathBuf) {
    println!("{:?}", path.display());
    let html : String = file_to_html(&path).expect("failed to read MD!");
    let file_name : OsString = path.file_name().expect("Failed to read file name!").to_owned();

    let mut output = FORMAT_HTML.clone().to_string();
    output = output.replace("{CONTENT}", html.as_str());
    output = output.replace("{PAGE_NAME}", file_name.to_str().expect("Failed getting file name"));
   

    let mut file_name: String = path.to_str().expect("Failed converting Path").split("pages/").last().expect("Invalid path directory, missing pages!").to_string();
    let re : Regex = Regex::new(r"\..+").unwrap();
    
    file_name = re.replace(&file_name, ".html").to_string();
    file_name = ["deploy/", &file_name].join("");
   
    // get directory name and do
    //create_dir_all().expect("Failed to create directories!");

    let mut file = File::create(file_name).expect("Failed to open file to save!");
    file.write_all(output.as_bytes()).expect("Failed to write to file :(!");
}
