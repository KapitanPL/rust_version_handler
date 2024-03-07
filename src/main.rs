use std::error::Error;
use std::ops::Index;

mod regex_checker;
use regex_checker::RegexChecker;

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::path::Path;

const MONTH_ABBREVIATIONS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

const UPPER_YEAR_BOUND: i32 = 2040;
const LOWER_YEAR_BOUND: i32 = 2000;

const LOWER_MONTH_BOUND: i32 = 1;

struct DateHandler{
    year: i32,
    month: i32,
    day: i32,
    separator: String,
    year_first: bool,
    month_as_string: bool
}

impl DateHandler{
    fn new(date_string: &str)->Option<DateHandler>{

        let numeric_year_first_regex = RegexChecker::new(r"^(20\d{2})([\.-])(\d{1,2})[\.-](\d{1,2})$");
        let mixex_year_first_regex = RegexChecker::new(r"^(20\d{2})(-)([A-z]{3})-(\d{1,2})$");
        let numeric_year_last_regex = RegexChecker::new(r"^(\d{1,2})(-)(\d{1,2})-(20\d{2}))$");

        let mut year;
        let mut month;
        let mut day;
        let mut separator;
        let mut year_first;
        let mut month_as_string;
        if numeric_year_last_regex.is_match(date_string)
        {
            let captures = numeric_year_first_regex.get_captures(date_string);
            year = captures.get(4).unwrap().parse::<i32>().unwrap();
            month = captures.get(3).unwrap().parse::<i32>().unwrap();
            separator = captures.get(2).unwrap();
            day = captures.get(1).unwrap().parse::<i32>().unwrap();
            year_first = false;
            month_as_string = false;
        }
        else if  numeric_year_first_regex.is_match(date_string) {
            let captures = numeric_year_first_regex.get_captures(date_string);
            year = captures.get(1).unwrap().parse::<i32>().unwrap();
            separator = captures.get(2).unwrap();
            month = captures.get(3).unwrap().parse::<i32>().unwrap();
            day = captures.get(4).unwrap().parse::<i32>().unwrap();
            year_first = true;
            month_as_string = false;
        }
        else if mixex_year_first_regex.is_match(date_string) {
            let captures = mixex_year_first_regex.get_captures(date_string);
            year = captures.get(1).unwrap().parse::<i32>().unwrap();
            separator = captures.get(2).unwrap();
            month = Self::parse_month_string(captures.get(3).unwrap());
            day = captures.get(4).unwrap().parse::<i32>().unwrap();
            year_first = true;
            month_as_string = true;
        }
        Some(DateHandler{
            year: year,
            month: month,
            day: day,
            separator: separator.clone(),
            year_first: year_first,
            month_as_string: month_as_string
        })
    }

    fn parse_month_string(month: &str)->i32
    {
        let index = MONTH_ABBREVIATIONS.iter().position(|&m| m.eq_ignore_ascii_case(month));
        if index.is_some()
        {
            return index.unwrap() as i32;
        }
        return -1;
    }

    fn check_year_valid(year: i32)->bool{
        return year > LOWER_YEAR_BOUND && year < UPPER_YEAR_BOUND;
    }
}

struct NumbersHandler{
    major: i32,
    minor: i32,
    patch: i32
}

impl NumbersHandler{
    fn new(version: &str)->NumbersHandler{

    }
}

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
                    version: if use_number_based_handler {
                            MainVersion::Version(NumbersHandler::new(splitted_version.get(1).cloned().unwrap().as_str()))
                        } else { 
                            MainVersion::Date(DateHandler{})
                        },
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