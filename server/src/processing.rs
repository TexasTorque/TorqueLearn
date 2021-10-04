use std::{ffi::OsString, fs::{self, File, copy, create_dir_all}, io::Write, path::{Path, PathBuf}};

use copy_dir::copy_dir;
use glob::glob;
use markdown::file_to_html;
use walkdir::WalkDir;
use std::process::Command;

const FORMAT_HTML: &'static str = include_str!("../../layout/format.html");
const SECTION_FORMAT_HTML : &'static str = include_str!("../../layout/section_format.html");

const TOP_DIR_ENTRY: &'static str = "<a href=\"{URL}\"class=\"box\"><h3>";
const MIDDLE_SECTION_ENTRY: &'static str = "</h3>";
const FOLDER_SVG_SECTION: &'static str = "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" version=\"1.1\" class=\"svgicon\" x=\"0px\" y=\"0px\" viewBox=\"0 0 298.757 298.757\" style=\"enable-background:new 0 0 298.757 298.757;\" xml:space=\"preserve\"><path d=\"M291.701,119.091h-39.95v-29.61c0-3.893-3.156-7.05-7.05-7.05h-95.549l-16.896-35.955    c-1.162-2.472-3.648-4.051-6.381-4.051H7.05c-3.893,0-7.05,3.157-7.05,7.05v199.806c0,3.846,3.135,7.051,7.054,7.051    c0.004,0,0.008-0.001,0.012-0.001h237.635c2.923,0,5.543-1.805,6.587-4.536l47-123.14    C300.048,124.044,296.635,119.091,291.701,119.091z M14.1,56.526h107.299l16.896,35.955c1.162,2.472,3.648,4.051,6.381,4.051    h92.975v22.56H54.05c-2.923,0-5.544,1.805-6.587,4.536L14.1,211.04V56.526z M239.846,242.231H17.287l41.618-109.04    c10.158,0,212.404,0,222.559,0L239.846,242.231z\"/>";
const DOCUMENT_SVG_SECTION: &'static str = "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" version=\"1.1\" class=\"svgicon\" x=\"0px\" y=\"0px\" viewBox=\"0 0 293.151 293.151\" style=\"enable-background:new 0 0 293.151 293.151;\" xml:space=\"preserve\"><path d=\"M255.316,55.996l-51.928-52.94C201.471,1.102,198.842,0,196.104,0h-82.302h-8.232H45.113    c-5.631,0-10.197,4.566-10.197,10.192c0,5.626,4.566,10.192,10.197,10.192h60.457h8.232h72.11l0.005,44.231    c0,5.631,4.561,10.197,10.192,10.197h41.731v197.955H56.592V47.828c0-5.631-4.566-10.197-10.197-10.197    c-5.631,0-10.192,4.566-10.192,10.197v235.131c0,5.631,4.566,10.192,10.192,10.192h201.642c5.631,0,10.197-4.566,10.197-10.192    V63.137C258.229,60.467,257.185,57.903,255.316,55.996z M206.307,54.423V35.147l18.906,19.276H206.307z\"/>";
const BOTTOM_DIR_ENTRY: &'static str = "</a>";

const TOC_LIST_START: &'static str = "<li><a class=\"\" href=\"/Tutorials/";
const TOC_LIST_MID: &'static str = "\">";
const TOC_LIST_END: &'static str = "</a></li>";

pub fn process() {
    println!("Processing files...");
    
    // Remove deploy directory
    fs::remove_dir_all("deploy/").ok();
    
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

    // Copy WASM to directory
    Command::new("cp")
        .args(["-r", "./pkg", "./deploy/static/"])
        .output()
        .expect("Failed to move WASM");
    

    let featured_file : String = fs::read_to_string("pages/Featured.csv")
        .expect("Something went wrong reading the file");
    let featured : Vec<&str> = featured_file.split(",").collect();

    // Read all MDs
    for entry in glob("pages/**/*.md").expect("Failed to read glob pattern!") {
        match entry {
            Ok(path) => handle_md(path, featured.clone()),
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

                    let mut paths: Vec<_> = fs::read_dir(dir.path()).unwrap().map(|r| r.unwrap()).collect(); 
                    paths.sort_by_key(|dir| dir.path()); 
                    for path in paths {
                        let n = path.file_name();
                        let name = n.to_str().expect("Failed OSString->str");
                        ret.push_str(TOP_DIR_ENTRY);
                        ret.push_str(name);
                        ret.push_str(MIDDLE_SECTION_ENTRY);
                        
                        if Path::new(&([path.path().to_str().expect("Failed path->str"), "/index.html"].join(""))).exists() {
                            ret.push_str(DOCUMENT_SVG_SECTION);
                        } else {
                            ret.push_str(FOLDER_SVG_SECTION);
                        }

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

fn handle_md(path: PathBuf, featured: Vec<&str>) {
    let html : String = file_to_html(&path).expect("failed to read MD!");
    let file_name : OsString = path.file_name().expect("Failed to read file name!").to_owned();

    let mut output = FORMAT_HTML.clone().to_string();
    output = output.replace("{CONTENT}", html.as_str());
    output = output.replace("{PAGE_NAME}", file_name.to_str().expect("Failed getting file name"));
    
    let mut toc : String = String::new(); 
    for feature in featured {
        toc.push_str(TOC_LIST_START);
        toc.push_str(feature);
        toc.push_str(TOC_LIST_MID);
        toc.push_str(feature);
        toc.push_str(TOC_LIST_END);
    }
    output = output.replace("{TOC}", &toc);

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
