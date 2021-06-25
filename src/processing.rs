use std::{ffi::OsString, fs::{self, File, copy, create_dir_all}, io::Write, path::{Path, PathBuf}};

use copy_dir::copy_dir;
use glob::glob;
use markdown::file_to_html;
use walkdir::WalkDir;

const FORMAT_HTML: &'static str = include_str!("../layout/format.html");
const SECTION_FORMAT_HTML : &'static str = include_str!("../layout/section_format.html");
const TOP_DIR_ENTRY: &'static str = "<a href=\"{URL}\"class=\"section\">";
const BOTTOM_DIR_ENTRY: &'static str = "</a>";

pub fn process() {
    println!("Processing files...");
    // Remove deploy directory
    fs::remove_dir_all("deploy/").expect("Failed to remove deploy directory!");
    
    // Create deploy directory
    fs::create_dir("deploy/").expect("Failed to create a new deploy directory!");

    // Copy non-page files
    for entry in glob("layout/*.html").expect("Failed to read glob pattern!") {
        match entry {
            Ok(path) => {
                let file_name : &str = path.file_name().expect("Failed to get file name!").to_str().expect("Failed converting OS String to &str!"); 
                if file_name != "format.html" && file_name != "section_format.html" {
                    if file_name == "index.html" {
                        copy(path.clone(), ["deploy/", file_name].join("")).expect("Failed copying HTML file over");
                    } else {
                        let dir = ["deploy/", file_name.trim_end_matches(".html")].join("");
                        fs::create_dir(&dir).expect("Failed to create layout file dir!");
                        copy(path.clone(), [dir, "/index.html".to_owned()].join("")).expect("Failed copying HTML file over");
                    }
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
    WalkDir::new("deploy/Tutorials")
        .into_iter()
        .filter_entry(|dir| dir.metadata().expect("Failed to read file metadata").is_dir() && !Path::new(&([dir.path().to_str().expect("Failed path->str"), "/index.html"].join(""))).exists())
        .for_each(|x|{
            match x {
                Ok(dir) => {
                    let mut ret : String = String::new();
                    let p : &str = dir.path().to_str().expect("Failed path->str").trim_start_matches("deploy");
                    
                    for path in fs::read_dir(dir.path()).unwrap()  {
                        let n = path.unwrap().file_name();
                        let name = n.to_str().expect("Failed OSString->str");
                        ret.push_str(TOP_DIR_ENTRY);
                        ret.push_str(name);
                        ret = ret.replace("{URL}", &[p, name].join("/")); 
                        ret.push_str(BOTTOM_DIR_ENTRY);
                    }

                    let mut output = SECTION_FORMAT_HTML.clone().to_string();
                    output = output.replace("{CONTENT}", &ret);
                    output = output.replace("{PAGE_NAME}", dir.file_name().to_str().expect("Failed getting file name"));
                    output = output.replace("{TITLE}", dir.file_name().to_str().expect("Failed getting file name"));
                    
                    let mut file = File::create([dir.path().to_str().expect("Failed path->str"), "/index.html"].join("")).expect("Failed opening file to save");
                    file.write_all(output.as_bytes()).expect("Failed to write to file :(!");
                    println!("{:?}", dir.path().to_str().expect("Failed path->str").trim_start_matches("deploy"));
                }
                Err(e) => println!("Failed walking directory! {:?}", e),
            }
        });
    println!("Deploy directory successfully created!");
}

fn handle_md(path: PathBuf) {
    let html : String = file_to_html(&path).expect("failed to read MD!");
    let file_name : OsString = path.file_name().expect("Failed to read file name!").to_owned();

    let mut output = FORMAT_HTML.clone().to_string();
    output = output.replace("{CONTENT}", html.as_str());
    output = output.replace("{PAGE_NAME}", file_name.to_str().expect("Failed getting file name"));
   

    let mut file_name: String = path.to_str().expect("Failed converting Path").split("pages/").last().expect("Invalid path directory, missing pages!").to_string();

    file_name = file_name.trim_end_matches(".md").to_string();
    file_name.push_str(".html");
    file_name = ["deploy/Tutorials/", &file_name].join("");
   
    // get directory name and do
    let directories = file_name.trim_end_matches(".html");
    create_dir_all(directories).expect("Failed to create directories!");
    
    let mut file = File::create([directories, "/index.html"].join("")).expect("Failed to open file to save!");
    file.write_all(output.as_bytes()).expect("Failed to write to file :(!");
}
