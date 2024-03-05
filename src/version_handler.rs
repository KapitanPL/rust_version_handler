mod regex_checker;
use regex_checker::RegexChecker;


struct DateHandler{}

struct Versionhandler{}

enum MainVersion {
    Date(DateHandler),
    Version(Versionhandler),
}

struct Versionhandler{
    prefix: Option<String>,
    version: MainVersion,
    sufix: Option<String>
}

impl Versionhandler{
    fn new(version_string: &str)->Option<Versionhandler>{
        let year_checker = RegexChecker::new(r"(\D|^)20\d{2}(\D|$)");
        let version_string_regex = RegexChecker::new(r"(\D|^)(\d+\.\d+(\.\d+)?)(\D|$)");

        if year_checker.is_match(version) // year check must go first as some date formats resemble version format e.g. 2022.10.09
        {
            let date_regex = get_date_regex(version);
            if date_regex.is_some()
            {

            
            }
            else
            {
                return None;
            }
        }
        else if version_string_regex.is_match(version) {
            

        }
        else {
            return None;
        }
    }

    fn get_date_regex(string: &str)->Option<RegexChecker>
    {
        let date_regexes = vec![
            RegexChecker::new(r"(\D|^)(20\d{2}[\.-]\d{1,2}[\.-]\d{1,2})(\D|$)"),
            RegexChecker::new(r"(\D|^)(20\d{2}-[A-z]{3}-\d{1,2})(\D|$)"),
            RegexChecker::new(r"(\D|^)(\d{1,2}-\d{1,2}-20\d{2})(\D|$)")
        ];
        for regex in date_regexes.iter(){
            if regex.is_match(string){
                return Some(regex.clone());
            }
        }
        None
    }

    fn split_with_regex(string : &str, regex_checker: &RegexChecker) -> Vec<&str>{
        let captures = regex_checker.get_captures(version);
        let main_version = captures.get(2).unwrap();
        let mut parts: Vec<&str> = version.split(main_version).collect();
    }
}