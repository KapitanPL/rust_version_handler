pub struct SufixHandler
{
    
}

impl SufixHandler{
    pub fn new(patch_string: &str, separator: &str) -> SufixHandler {
        
        let parts: Vec<String> = patch_string
            .split(separator)
            .map(|s| s.to_string().parse<i32>())
            .collect();

        let mut parsed_parts = Vec::new();
        for part in parts {
            match part.parse::<i32>() {
                Ok(num) => parsed_parts.push(StringOrInt::Int(num)), // Store as an integer if parsed successfully
                Err(_) => parsed_parts.push(StringOrInt::Str(part.to_string())), // Otherwise, store as a string
            }
        }

        SufixHandler {
            patch_string: patch_string.to_string(),
            separator: separator.to_string(),
            parts: parsed_parts,
        }
    }

}
