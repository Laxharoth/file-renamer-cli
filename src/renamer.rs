struct Renamer{
    pub StringRepresentation: String,
    pub WildcardChar: char,
    pub FixedStrings: Vec<String>,
    pub PositionSelectWrapper: (char,char),
    pub PositionsOrder: Vec<usize>,
}

impl Renamer {
    pub fn new(StringRepresentation:String, WildcardChar:char, PositionSelectWrapper:(char,char)) -> Self{
        let mut FixedStrings: Vec<String> = Vec::new();
        let mut PositionsOrder: Vec<usize> = Vec::new();

        Renamer { 
            StringRepresentation, 
            WildcardChar, 
            FixedStrings, 
            PositionSelectWrapper,
            PositionsOrder
        }
    }


    pub fn generate_rename_filename(&self, filename:&String, wildcard_catched: &Vec<String>) -> String{

        return "".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renamer_new() {
        let renamer = Renamer::new("file_*_name_*".to_string(), '*', ('(', ')'));
        assert_eq!(renamer.StringRepresentation, "file_*_name_*", "StringRepresentation should be set correctly.");
        assert_eq!(renamer.WildcardChar, '*', "WildcardChar should be set to '*'.");
        assert_eq!(renamer.PositionSelectWrapper, ('(', ')'), "PositionSelectWrapper should be set to ('(', ')').");
        assert_eq!(renamer.FixedStrings, vec!["file_", "_name_", ""], "FixedStrings should be the strings between wildcards.");
    }

    #[test]
    fn test_fixed_strings_with_wildcard_char() {
        let renamer = Renamer::new("file_*_name_*".to_string(), '*', ('(', ')'));
        assert_eq!(
            renamer.get_fixed_strings(),
            vec!["file_", "_name_", ""],
            "FixedStrings should split correctly using the wildcard char."
        );
    }

    #[test]
    fn test_fixed_strings_with_position_select_wrapper() {
        let renamer = Renamer::new("file_( 123 )_name_(456)".to_string(), '*', ('(', ')'));
        assert_eq!(
            renamer.get_fixed_strings(),
            vec!["file_", "_name_", ""],
            "FixedStrings should split correctly using the PositionSelectWrapper."
        );
    }

    #[test]
    fn test_fixed_strings_with_different_wildcard_char() {
        let renamer = Renamer::new("file_?_name_?".to_string(), '?', ('(', ')'));
        assert_eq!(
            renamer.get_fixed_strings(),
            vec!["file_", "_name_", ""],
            "FixedStrings should split correctly using a different WildcardChar."
        );
    }

    #[test]
    fn test_fixed_strings_with_different_position_select_wrapper() {
        let renamer = Renamer::new("file_[123]_name_[456]".to_string(), '*', ('[', ']'));
        assert_eq!(
            renamer.get_fixed_strings(),
            vec!["file_", "_name_", ""],
            "FixedStrings should split correctly using a different PositionSelectWrapper."
        );
    }

    #[test]
    fn test_fixed_strings_with_incomplete_position_select_wrapper() {
        let renamer = Renamer::new("file_(123_name_456)".to_string(), '*', ('(', ')'));
        assert_eq!(
            renamer.get_fixed_strings(),
            vec!["file_(123_name_456)"],
            "FixedStrings should not split if the PositionSelectWrapper is incomplete."
        );
    }

    #[test]
    fn test_fixed_strings_with_spaces_in_position_select_wrapper() {
        let renamer = Renamer::new("file_(  123  )_name_(  456  )".to_string(), '*', ('(', ')'));
        assert_eq!(
            renamer.get_fixed_strings(),
            vec!["file_", "_name_", ""],
            "FixedStrings should split correctly even with spaces in the PositionSelectWrapper."
        );
    }

    #[test]
    fn test_generate_rename_filename_with_wildcard_char() {
        let renamer = Renamer::new("file_*_name_*".to_string(), '*', ('(', ')'));
        let wildcard_catched = vec!["123".to_string(), "456".to_string()];
        let result = renamer.generate_rename_filename(&"file_123_name_456".to_string(), &wildcard_catched);
        assert_eq!(
            result,
            "file_123_name_456",
            "Filename should be generated correctly using wildcard characters."
        );
    }

    #[test]
    fn test_generate_rename_filename_with_position_select_wrapper() {
        let renamer = Renamer::new("file_(0)_name_(1)".to_string(), '*', ('(', ')'));
        let wildcard_catched = vec!["123".to_string(), "456".to_string()];
        let result = renamer.generate_rename_filename(&"file_123_name_456".to_string(), &wildcard_catched);
        assert_eq!(
            result,
            "file_123_name_456",
            "Filename should be generated correctly using position selectors."
        );
    }

    #[test]
    fn test_generate_rename_filename_with_both_wildcard_and_position_selectors() {
        let renamer = Renamer::new("file_*_name_(1)_extra_(0)".to_string(), '*', ('(', ')'));
        let wildcard_catched = vec!["123".to_string(), "456".to_string()];
        let result = renamer.generate_rename_filename(&"file_123_name_456".to_string(), &wildcard_catched);
        assert_eq!(
            result,
            "file_123_name_456_extra_123",
            "Filename should be generated correctly using both wildcard characters and position selectors."
        );
    }

    #[test]
    fn test_generate_rename_filename_with_position_out_of_bounds() {
        let renamer = Renamer::new("file_(2)_name".to_string(), '*', ('(', ')'));
        let wildcard_catched = vec!["123".to_string(), "456".to_string()];
        let result = std::panic::catch_unwind(|| {
            renamer.generate_rename_filename(&"file_123_name_456".to_string(), &wildcard_catched)
        });
        assert!(
            result.is_err(),
            "Using a position selector out of bounds should throw an error."
        );
    }

    #[test]
    fn test_generate_rename_filename_with_empty_captured_strings() {
        let renamer = Renamer::new("file_(0)_name".to_string(), '*', ('(', ')'));
        let wildcard_catched: Vec<String> = vec![];
        let result = std::panic::catch_unwind(|| {
            renamer.generate_rename_filename(&"file_123_name_456".to_string(), &wildcard_catched)
        });
        assert!(
            result.is_err(),
            "Using a position selector when no strings are captured should throw an error."
        );
    }
}