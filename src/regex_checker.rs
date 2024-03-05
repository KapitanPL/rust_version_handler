extern crate regex;
use regex::Regex;

pub struct RegexChecker {
    pub regex: Regex,
}

impl RegexChecker {
    pub fn new(new_regex: &str) -> RegexChecker {
        RegexChecker {
            regex: Regex::new(new_regex).unwrap(),
        }
    }

    pub fn is_match(&self, input: &str) -> bool {
        self.regex.is_match(input)
    }

    pub fn get_captures(&self, text: &str) -> Vec<String> {
        let mut captures = Vec::new();
        for caps in self.regex.captures_iter(text) {
            for cap in caps.iter() {
                if cap.is_some(){
                    captures.push(cap.unwrap().as_str().to_string());
                }
            }
        }
        captures
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module

    #[test]
    fn test_contains_year() {
        let _year_checker = RegexChecker::new(r"(\D|^)20\d{2}(\D|$)");
        assert!(_year_checker.is_match("release-2023."));
        assert!(_year_checker.is_match("09-28-2023.1"));
        assert!(_year_checker.is_match("2023-10-9.1"));
        assert!(!_year_checker.is_match("20211.1"));
        assert!(!_year_checker.is_match("12021.1"));
    }
}