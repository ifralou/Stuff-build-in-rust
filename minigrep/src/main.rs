use std::env;
use std::fs;

fn get_arguments(arguments: &Vec<String>) -> (&String, &String) {
    if arguments.len() < 3 {
        panic!("Sorry, not enough arguments");
    }
    let search = &arguments[1];
    let filepath = &arguments[2];

    (search, filepath)
}

fn get_lines_with_query<'a>(contents: &'a String, query: &String) -> Vec<&'a str> {
    let mut findings: Vec<&str> = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
       if line.to_lowercase().contains(&query) {
           findings.push(line);
       }
    }
    findings
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (search, filepath) = get_arguments(&args);

    let contents = fs::read_to_string(filepath)
        .expect("Problem reading the file");

    for target_line in get_lines_with_query(&contents, search) {
        println!("{}", target_line);
    }
}
