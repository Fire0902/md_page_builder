mod utils;

use std::{env, process};


fn main() {
    let path : Vec<String> = env::args().collect();
    if path.len() < 2 {process::exit(1);}
    if !utils::is_file(&path[1]) {process::exit(1)}
    if utils::is_extension(&path[1], ".md") {process::exit(1);}
    let file : String = utils::read_file(path[1].clone());

    let path_template = "./template/template.html";
    if !utils::is_file(path_template){process::exit(1)}
    let template_content = utils::read_file(String::from(path_template));
    
    let template_title =  template_content.replace("$title", &path[1][..path[1].len()-3]); // on remplace le titre par le nom du fichier sans l'extension

    

    for s in file.chars() {
        if s != '\n'{
            print!("{}", s);
        }
    }
    println!("{}", file);
}
