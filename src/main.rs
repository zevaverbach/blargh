// TODO: blog drafts (list)
use std::env::{args, var};
use std::io::stdin;
use std::path::Path;
use std::process::{exit, Command};

use regex::Regex;

const SUBCOMMANDS: [&str; 1] = ["new"];

fn main() {
    let args: Vec<String> = args().collect();
    validate_args(&args);
    let handle = get_handler(&args);
    handle(args);
}

fn get_handler(args: &Vec<String>) -> fn(Vec<String>) {
    match args[1].as_str() {
        "new" => handle_new,
        _ => no_match_in_handler,
    }
}

fn no_match_in_handler(args: Vec<String>) {
    eprintln!("no match in handler for subcommand {}", args[1]);
    exit(1);
}

fn create_markdown_file_and_open_in_editor(url: String) {
    // TODO: make this a const
    let editor: String = var("EDITOR").expect("Please set the EDITOR environment variable");
    println!("create_markdown_file_and_open_in_editor");
}

fn url_exists(url: &str) -> &str {
    "hi"
}

fn convert_to_html_and_preview_and_push_to_bucket(markdown_filepath: String) {
    println!("convert_to_html_and_preview_and_push_to_bucket");
}

fn validate_url(url: &String) -> Result<&str, &str>{
    let valid_url = Regex::new("^\\b([-a-zA-Z0-9@:%._\\+~#?&//=]*)$").unwrap();
    if valid_url.is_match(url) {
        return Ok("valid URL")
    }
    Err("invalid URL")
}

fn handle_new(args: Vec<String>) {
    let mut is_markdown = false;
    let mut url = String::new();
    let mut url_or_fn = String::new();

    if args.len() == 2 {
        println!("what do you want the blog post's URL to be?");
        match stdin().read_line(&mut url) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("problem parsing input");
                exit(1);
            }
        };
    } else {
        url_or_fn.push_str(&args[2]);
        if url_or_fn.ends_with(".md") {
            is_markdown = true;
            url.push_str(Path::new(&url_or_fn).file_stem().unwrap().to_str().expect("couldn't get file stem"));
        } else if url_or_fn.contains('.') {
            eprintln!("please provide either a url or a markdown filepath");
            exit(1);
        } else {
            url.push_str(&url_or_fn);
        }
    };
    let url = url.trim().to_string();
    match validate_url(&url) {
        Err(_) => {
            eprintln!("please provide either a url or a markdown filepath");
            exit(1);
        },
        Ok(_) => (),
    };
    match url_exists(&url) {
        "draft" => {
            eprintln!("this URL already exists! do `blog edit {url}` to edit it or `blog publish {url}` to publish it");
            exit(1);
        },
        "published" => {
            eprintln!("this URL already exists! do `blog edit {url}` to edit it or `blog unpublish {url}` to un-publish it");
            exit(1);
        },
        _ => (),
    }
    if is_markdown {
        convert_to_html_and_preview_and_push_to_bucket(url_or_fn);
    } else {
        create_markdown_file_and_open_in_editor(url);
        // TODO: convert to html and preview after close
    }
}

fn validate_args(args: &Vec<String>) {
    println!("{:?}", args);
    if args.len() < 2 {
        eprintln!("not enough arguments, please provide at least two");
        exit(1);
    }
    if args.len() > 3 {
        eprintln!("too many arguments, please provide at most three");
        exit(1);
    }
    let subcommand = &args[1];
    if !SUBCOMMANDS.contains(&subcommand.as_str()) {
        let subcommand_string = SUBCOMMANDS.join(", ");
        eprintln!("invalid subcommand {}, please choose from ({:?})", subcommand, subcommand_string);
        exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_url_ok() {
        let url = "mygreatblogpost".to_string();
        assert!(validate_url(&url).is_ok());
        let url = "my-great-blogpost".to_string();
        assert!(validate_url(&url).is_ok());
        let url = "my%20great-blogpost".to_string();
        assert!(validate_url(&url).is_ok());
        let url = "my%0Agreat-blogpost".to_string();
        assert!(validate_url(&url).is_ok());
        let url = "/posts/my%0Agreat-blogpost".to_string();
        assert!(validate_url(&url).is_ok());
    }
    
    #[test]
    fn validate_url_is_err() {
        let url = "my>-great-blogpost".to_string();
        assert!(validate_url(&url).is_err());
        let url = "my[-]great-blogpost".to_string();
        assert!(validate_url(&url).is_err());
        let url = "my-great-blogpost*".to_string();
        assert!(validate_url(&url).is_err());
    }

    // more tests...
}
