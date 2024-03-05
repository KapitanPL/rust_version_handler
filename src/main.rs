use std::error::Error;

mod regex_checker;
use regex_checker::RegexChecker;

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::path::Path;

struct DateHandler{}

struct NumbersHandler{}

enum MainVersion {
    Date(DateHandler),
    Version(NumbersHandler),
}

struct Versionhandler{
    prefix: Option<String>,
    version: MainVersion,
    sufix: Option<String>
}

impl Versionhandler{
    fn new(version_string: &str)->Option<Versionhandler>{
        let date_regexes = vec![
            RegexChecker::new(r"(\D|^)(20\d{2}[\.-]\d{1,2}[\.-]\d{1,2})(\D|$)"),
            RegexChecker::new(r"(\D|^)(20\d{2}-[A-z]{3}-\d{1,2})(\D|$)"),
            RegexChecker::new(r"(\D|^)(\d{1,2}-\d{1,2}-20\d{2})(\D|$)"),
            RegexChecker::new(r"(\D|^)(\d+\.\d+(\.\d+)?)(\D|$)") // this must be last as some date based version strings looks like number based versions. e.g. 2021.10.11
        ];

        let regex: Option<&RegexChecker> = Self::get_date_regex(version_string, &date_regexes);

        if regex.is_some(){
            let splitted_version = Self::split_with_regex(version_string, regex.unwrap());
            let use_number_based_handler = regex.unwrap().regex.as_str() == date_regexes.last().unwrap().regex.as_str();
            if splitted_version.len() == 3{
                return Some(Versionhandler{
                    prefix : splitted_version.get(0).cloned(),
                    version: if use_number_based_handler {MainVersion::Version(NumbersHandler{})} else { MainVersion::Date(DateHandler{})},
                    sufix: splitted_version.get(2).cloned()
                });
            }
            else {
                print!("WARN: unexpected split for version: {}", version_string);
            }
        }
    return None;
    }

    fn get_date_regex<'a>( string: &str, regexes : &'a Vec<RegexChecker>)->Option<&'a RegexChecker>
    {
        for regex in regexes.iter(){
            if regex.is_match(string){
                return Some(regex);
            }
        }
        None
    }

    fn split_with_regex<'a>(string : &'a str, regex_checker: &RegexChecker) -> Vec<String>{
        let captures = regex_checker.get_captures(string);
        let main_version = captures.get(2).unwrap();
        let mut parts: Vec<String> = string.split(main_version).into_iter().map(|p| p.to_string()).collect();
        parts.insert(1, main_version.to_string());
        return parts;
    }

    pub fn print_sufix(self){
        println!("{}", self.sufix.unwrap_or("no suffix".to_string()));
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let path = "../Versions";
    let file = File::open(Path::new(path))?;
    let reader = io::BufReader::new(file);

    let mut versions = HashSet::new();

    for line_result in reader.lines() {
        let line = line_result?;  // Handle potential errors in reading lines
        let parts = line.split(',').map(|s| s.trim().to_string());  // Split line by ',' and trim whitespace
        versions.extend(parts);  // Add the parts to the HashSet
    }

    for version in versions.iter() {
        let handler = Versionhandler::new(version);
        if handler.is_some()
        {
            handler.unwrap().print_sufix();
        }
        else {
            println!("NONE: {}", version);
        }
    }

    /*
    let mut year_base_versions = HashSet::new();
    let mut version_base_versions = HashSet::new();
    let mut unknown_versions = HashSet::new();

    let year_checker = RegexChecker::new(r"(\D|^)20\d{2}(\D|$)");
    let version_string_regex = RegexChecker::new(r"(\D|^)(\d+\.\d+(\.\d+)?)(\D|$)");
    let date_nums_string_regex = RegexChecker::new(r"(\D|^)(20\d{2}[\.-]\d{1,2}[\.-]\d{1,2})(\D|$)");
    let date_abrev_string_regex = RegexChecker::new(r"(\D|^)(20\d{2}-[A-z]{3}-\d{1,2})(\D|$)");
    let reverse_date_string_regex = RegexChecker::new(r"(\D|^)(\d{1,2}-\d{1,2}-20\d{2})(\D|$)");


    for version in versions.iter() {
        if year_checker.is_match(version) // year check must go first as some date formats resemble version format e.g. 2022.10.09
        {
            year_base_versions.insert(version);
        }
        else if version_string_regex.is_match(version) {
            version_base_versions.insert(version);
        }
        else {
            unknown_versions.insert(version);
        }
    }

    for version in version_base_versions {
        let captures = version_string_regex.get_captures(version);
        let main_version = captures.get(2).unwrap();
        let parts: Vec<&str> = version.split(main_version).collect();
        println!("{}: {} - {:?}", version, captures.get(2).unwrap(), parts);
    }

    for version in year_base_versions
    {
        if date_nums_string_regex.is_match(&version){
            let captures = date_nums_string_regex.get_captures(version);
            let main_version = captures.get(2).unwrap();
            let parts: Vec<&str> = version.split(main_version).collect();
            println!("{}: {} - {:?}", version, captures.get(2).unwrap(), parts);
        }
        else if date_abrev_string_regex.is_match(&version) {
            let captures = date_abrev_string_regex.get_captures(version);
            let main_version = captures.get(2).unwrap();
            let parts: Vec<&str> = version.split(main_version).collect();
            println!("{}: {} - {:?}", version, captures.get(2).unwrap(), parts);
        }
        else if reverse_date_string_regex.is_match(&version) {
            let captures = reverse_date_string_regex.get_captures(version);
            let main_version = captures.get(2).unwrap();
            let parts: Vec<&str> = version.split(main_version).collect();
            println!("{}: {} - {:?}", version, captures.get(2).unwrap(), parts);
        }
        else{
            unknown_versions.insert(version);
        }
    }

    print!("{:?}", unknown_versions);
    */
    
    Ok(())
}