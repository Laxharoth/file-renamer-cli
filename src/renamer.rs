pub struct Renamer{
    pub StringRepresentation: String,
    pub WildcardChar: char,
    pub FixedStrings: Vec<String>,
    pub PositionSelectWrapper: (char,char),
    pub PositionsOrder: Vec<usize>,
}

fn get_char(string:&String, index:&usize)->char{string.chars().nth(*index).unwrap()}

enum AutomataResult{
    Success(usize),
    Failure,
}

fn selector_wrapper_automata(
    end_index: &mut usize,
    string_representation: &String,
    position_select_wrapper: &(char,char)
) -> AutomataResult{
    // ( -> space -> number -> space -> )
    let mut state = 1;
    let mut position = 0;
    loop {
        *end_index += 1;
        match &state {
            1 =>{
                let character = get_char(string_representation, end_index);
                if character == ' '{
                    continue;
                }
                if character.is_numeric(){
                    state = 2;
                    position = character.to_digit(10).unwrap() as usize;
                    continue;
                }
                return AutomataResult::Failure;
            },
            2 =>{
                let character = get_char(string_representation, &end_index);
                if character.is_numeric(){
                    position = position * 10 + character.to_digit(10).unwrap() as usize;
                    continue;
                }
                if character == ' '{
                    state = 3;
                    continue;
                }
                if character == position_select_wrapper.1{
                    return AutomataResult::Success(position);
                }
                return AutomataResult::Failure;
            },
            3 =>{
                let character = get_char(string_representation, &end_index);
                if character == ' '{
                    continue;
                }
                if character == position_select_wrapper.1{
                    return AutomataResult::Success(position);
                }
                return AutomataResult::Failure;
            },
            _ => {
                return AutomataResult::Failure;
            }
        }
    }

    return AutomataResult::Success(0);
}

impl Renamer {
    pub fn new(StringRepresentation:String, WildcardChar:char, PositionSelectWrapper:(char,char)) -> Self{
        let mut FixedStrings: Vec<String> = Vec::new();
        let mut PositionsOrder: Vec<usize> = Vec::new();

        let mut start_index = 0;
        let mut end_index = 0;
        let mut wildcard_counter = 0;

        while end_index < StringRepresentation.len() {
            // Case where wildcard is found
            if get_char(&StringRepresentation, &end_index) == WildcardChar{
                FixedStrings.push(StringRepresentation[start_index..end_index].to_string());
                start_index = end_index + 1;
                end_index = end_index + 1;
                PositionsOrder.push(wildcard_counter);
                wildcard_counter += 1;
                continue;
            }
            // Case where position select wrapper could found
            if get_char(&StringRepresentation, &end_index) == PositionSelectWrapper.0{
                // Call automata to find the end of the position select wrapper
                let starting_position = end_index;
                match selector_wrapper_automata(
                    &mut end_index,
                    &StringRepresentation, 
                    &PositionSelectWrapper
                ){
                    AutomataResult::Success(position)=>{
                        FixedStrings.push(StringRepresentation[start_index..starting_position].to_string());
                        PositionsOrder.push(position);
                        start_index = end_index + 1;
                        end_index = end_index + 1;
                        continue;
                    },
                    AutomataResult::Failure =>{}
                }
            }
            end_index += 1;
        }
        FixedStrings.push(StringRepresentation[start_index..end_index].to_string());

        Renamer { 
            StringRepresentation, 
            WildcardChar, 
            FixedStrings, 
            PositionSelectWrapper,
            PositionsOrder
        }
    }

    pub fn get_fixed_strings(&self) -> Vec<String> {
        self.FixedStrings.clone()
    }

    pub fn generate_rename_filename(&self, filename:&String, wildcard_catched: &Vec<String>) -> String{
        let mut result = String::new();

        for i in 0..self.PositionsOrder.len(){
            result.push_str(self.FixedStrings[i].as_str());
            result.push_str(wildcard_catched[self.PositionsOrder[i]].as_str());
        }
        result.push_str(self.FixedStrings[self.FixedStrings.len()-1].as_str());

        result
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

    #[test]
    fn test_generate_rename_filename_with_wildcard_at_beginning() {
        let renamer = Renamer::new("*_file_name".to_string(), '*', ('(', ')'));
        let wildcard_catched = vec!["123".to_string()];
        let result = renamer.generate_rename_filename(&"123_file_name".to_string(), &wildcard_catched);
        assert_eq!(
            result,
            "123_file_name",
            "Filename should be generated correctly with a wildcard replacement at the beginning."
        );
    }
}