use std::default;

type IntPR = i32; // Precision

pub struct Counter{
    counter: IntPR,
    increment: IntPR,
}

impl Counter{
    pub fn new(counter: IntPR, increment: IntPR) -> Counter{
        Counter{
            counter: counter,
            increment: increment,
        }
    }

    pub fn count(&mut self) -> IntPR{
        let value = self.counter;
        self.counter += self.increment;
        return value; 
    }
}

fn get_char(string:&String, index:&usize)->char{string.chars().nth(*index).unwrap()}

enum WildcardType{
    Counter,
    String,
}

pub struct RenameFilter{
    string_representation: String,
    fixed_str: Vec<String>,
    counters: Vec<Counter>,
    wildcard_type: Vec<WildcardType>,
}

enum AutomataResult{
    Success,
    Failure,
}

fn rename_filter_find_counter_automata(end_index: &mut usize, string_representation: &String, counter_start: &mut IntPR, counter_increment: &mut IntPR)->AutomataResult{
    let mut state = 1;
    loop {
        *end_index += 1;
        match state {
            1 => {
                if get_char(string_representation, end_index) == ' '{
                    continue;
                }
                if get_char(string_representation, end_index).is_digit(10){
                    *counter_start = get_char(string_representation, &end_index).to_digit(10).unwrap() as IntPR;
                    state = 2;
                }
                else{
                    break;
                }
            }
            2 => {
                if get_char(string_representation, end_index).is_digit(10){
                    *counter_start = *counter_start * 10 + get_char(string_representation, end_index).to_digit(10).unwrap() as IntPR;
                    continue;
                }
                if get_char(string_representation, end_index) == ' '{
                    state = 3;
                    continue;
                }
                if get_char(string_representation, end_index) == ':'{
                    state = 4;
                }
                else{
                    break;
                }
            }
            3 => {
                if get_char(string_representation, end_index) == ' '{
                    continue;
                }
                if get_char(string_representation, end_index) == ':'{
                    state = 4;
                }
                else{
                    break;
                }
            }
            4 => {
                if get_char(string_representation, end_index) == ' '{
                    continue;
                }
                if get_char(string_representation, end_index).is_digit(10){
                    *counter_increment = get_char(string_representation, end_index).to_digit(10).unwrap() as IntPR;
                    state = 5;
                }
                else{
                    break;
                }
            }
            5 => {
                if get_char(string_representation, end_index).is_digit(10){
                    *counter_increment = *counter_increment * 10 + get_char(string_representation, end_index).to_digit(10).unwrap() as IntPR;
                    continue;
                }
                if get_char(string_representation, end_index) == ' '{
                    state = 6;
                    continue;
                }
                if get_char(string_representation, end_index) == '}'{
                    state = 7;
                    break;
                }
                else{
                    break;
                }
            }
            6 => {
                if get_char(string_representation, end_index) == ' '{
                    continue;
                }
                if get_char(string_representation, end_index) == '}'{
                    state = 7;
                    break;
                }
                else{
                    break;
                }
            }
            _ => break
            }
        }
        print!("state: {}", state);
        match state{
            7 => return AutomataResult::Success,
            _ => return AutomataResult::Failure,
        }
}

impl RenameFilter{
    pub fn new(string_representation: String, wildcard_char: char)->Self{
        let mut rf = RenameFilter{
            string_representation,
            fixed_str: vec![],
            counters: vec![],
            wildcard_type: vec![],
        };


        let mut start_index = 0;
        let mut end_index = 0;
        while end_index  < rf.string_representation.len(){
            // case where wildcard is found
            if get_char(&rf.string_representation, &end_index) == wildcard_char{
                rf.fixed_str.push(rf.string_representation[start_index..end_index].to_string());
                start_index = end_index + 1;
                end_index = end_index + 1;
                rf.wildcard_type.push(WildcardType::String);
                continue;
            }
            // case where counter could be found
            if get_char(&rf.string_representation, &end_index) == '{'{
                let starting_counter_wildcard = end_index;
                // automata  '{' -> ' '* -> digit+ -> ' '* -> ':' -> ' '* -> digit+ -> ' '* -> '}'
                let mut counter_start:IntPR = 0;
                let mut counter_increment:IntPR = 0;
                match rename_filter_find_counter_automata(&mut end_index, &rf.string_representation, &mut counter_start, &mut counter_increment){
                    AutomataResult::Success => {
                        rf.fixed_str.push(rf.string_representation[start_index..starting_counter_wildcard].to_string());
                        rf.counters.push(Counter::new(counter_start, counter_increment));
                        rf.wildcard_type.push(WildcardType::Counter);
                        start_index = end_index + 1;
                        end_index = end_index + 1;
                        continue;
                    },
                    AutomataResult::Failure => {}
                }
            }
            end_index += 1;
        }
        rf.fixed_str.push(rf.string_representation[start_index..].to_string());

        rf
    }

    pub fn get_fixed_str(&self) -> Vec<String>{
        self.fixed_str.clone()
    }

    pub fn does_fulfill(&self, filename: &str) -> bool{
        let mut filename_index = 0;
        for fixed_str in &self.fixed_str[..self.fixed_str.len()-1]{
            let index = filename[filename_index..].find(fixed_str);
            match index{
                Some(i) => filename_index += i + fixed_str.len(),
                None => return false,
            }
        }
        if &self.fixed_str[self.fixed_str.len()-1] == ""{
            return true;
        }
        let index = filename[filename_index..].find(&self.fixed_str[self.fixed_str.len()-1]);
        match index{
            Some(i) => return filename[filename_index+i..] == self.fixed_str[self.fixed_str.len()-1],
            None => return false,
        }
    }

    pub fn collect_wildcards(&mut self, filename: &str) -> Vec<String>{
        if self.wildcard_type.is_empty(){
            return vec![];
        }

        let mut filename_index = 0;
        let mut wildcard_index = 0;
        let mut counter_index = 0;
        let mut catched_wildcards = vec![];
        
        let index = filename[filename_index..].find(&self.fixed_str[0]);
        match index{
            Some(i) => filename_index += i + self.fixed_str[0].len(),
            None => return vec![],
        }

        for fixed_str in &self.fixed_str[1..self.fixed_str.len()-1]{
            let index = filename[filename_index..].find(fixed_str);
            match index{
                Some(i) => {
                    match &self.wildcard_type[wildcard_index]{
                        WildcardType::Counter =>{
                            catched_wildcards.push(self.counters[counter_index].count().to_string());
                            counter_index += 1;
                        }
                        WildcardType::String =>{
                            catched_wildcards.push(filename[filename_index..filename_index+i].to_string());
                        }
                    }
                    wildcard_index += 1;
                    filename_index += i + fixed_str.len();
                },
                None => return vec![],
            }
        }
        if &self.fixed_str[self.fixed_str.len()-1] == ""{
            match &self.wildcard_type[wildcard_index]{
                WildcardType::Counter =>{
                    catched_wildcards.push(self.counters[counter_index].count().to_string());
                }
                WildcardType::String =>{
                    catched_wildcards.push(filename[filename_index..].to_string());
                }
            }
            return  catched_wildcards;
        }
        let index = filename[filename_index..].find(&self.fixed_str[self.fixed_str.len()-1]);
        match index{
            Some(i) => {
                match &self.wildcard_type[wildcard_index]{
                    WildcardType::Counter =>{
                        catched_wildcards.push(self.counters[counter_index].count().to_string());
                    }
                    WildcardType::String =>{
                        catched_wildcards.push(filename[filename_index..filename_index+i].to_string());
                    }
                }
            },
            None => return vec![],
        }
        return catched_wildcards;
    }
}

impl default::Default for RenameFilter{
    fn default() -> Self{
        RenameFilter{
            string_representation: "".to_string(),
            fixed_str: vec![],
            counters: vec![],
            wildcard_type: vec![],
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut counter = Counter::new(0, 1);
        assert_eq!(counter.count(), 0, "Initial value of the counter should be 0.");
        assert_eq!(counter.count(), 1, "Counter increments by 1.");
        assert_eq!(counter.count(), 2, "Counter increments by 1 again.");

        let mut counter = Counter::new(10, 5);
        assert_eq!(counter.count(), 10, "Initial value of the counter should be 10.");
        assert_eq!(counter.count(), 15, "Counter increments by 5.");
        assert_eq!(counter.count(), 20, "Counter increments by 5 again.");
    }

    #[test]
    fn test_collect_wildcards_no_wildcards() {
        let mut filter = RenameFilter {
            string_representation: "file_name".to_string(),
            fixed_str: vec!["file_name".to_string()],
            ..Default::default()
        };
        let result = filter.collect_wildcards("file_name");
        assert!(result.is_empty(), "No wildcards, so the result should be empty.");
        assert!(filter.wildcard_type.is_empty(), "No wildcards, so the wildcard_type should be empty.");
    }

    #[test]
    fn test_collect_wildcards_some_wildcards() {
        let mut filter = RenameFilter {
            string_representation: "file_*_name_*".to_string(),
            wildcard_type: vec![WildcardType::String, WildcardType::String],
            fixed_str: vec!["file_".to_string(), "_name_".to_string(), "".to_string()],
            ..Default::default()
        };
        let result = filter.collect_wildcards("file_123_name_456");
        assert_eq!(result, vec!["123", "456"], "Wildcards should match '123' and '456'.");
    }

    #[test]
    fn test_collect_wildcards_one_counter() {
        let mut filter = RenameFilter {
            string_representation: "file_{1:1}_name".to_string(),
            counters: vec![Counter::new(1, 1)],
            wildcard_type: vec![WildcardType::Counter],
            fixed_str: vec!["file_".to_string(), "_name".to_string()],
            ..Default::default()
        };
        let result1 = filter.collect_wildcards("file_123_name");
        assert_eq!(result1, vec!["1"], "First counter starts at 1.");
        let result2 = filter.collect_wildcards("file_456_name");
        assert_eq!(result2, vec!["2"], "Counter increments to 2.");
    }

    #[test]
    fn test_collect_wildcards_two_counters() {
        let mut filter = RenameFilter {
            string_representation: "file_{ 1   : 1      }_name_{10:5}".to_string(),
            counters: vec![Counter::new(1, 1), Counter::new(10, 5)],
            wildcard_type: vec![WildcardType::Counter, WildcardType::Counter],
            fixed_str: vec!["file_".to_string(), "_name_".to_string(), "".to_string()],
            ..Default::default()
        };
        let result1 = filter.collect_wildcards("file_123_name_456");
        assert_eq!(result1, vec!["1", "10"], "First counter starts at 1, second at 10.");
        let result2 = filter.collect_wildcards("file_789_name_012");
        assert_eq!(result2, vec!["2", "15"], "First counter increments to 2, second to 15.");
    }

    #[test]
    fn test_does_fulfill_no_fixed_str() {
        let filter = RenameFilter {
            string_representation: "file_name".to_string(),
            fixed_str: vec!["file_name".to_string()],
            ..Default::default()
        };
        assert!(filter.does_fulfill("file_name"), "Filename matches the fixed string.");
        assert!(!filter.does_fulfill("file_name_extra"), "Filename does not match exactly.");
    }

    #[test]
    fn test_does_fulfill_with_fixed_str() {
        let filter = RenameFilter {
            string_representation: "file_*_name".to_string(),
            fixed_str: vec!["file_".to_string(), "_name".to_string()],
            wildcard_type: vec![WildcardType::String],
            ..Default::default()
        };
        assert!(filter.does_fulfill("file_123_name"), "Matches the fixed strings with wildcard in between.");
        assert!(filter.does_fulfill("file__name"), "Matches even with an empty wildcard.");
        assert!(!filter.does_fulfill("file_name_extra"), "Does not match due to extra characters.");
        assert!(!filter.does_fulfill("name_file"), "Does not match due to incorrect order.");
    }

    #[test]
    fn test_does_fulfill_partial_match() {
        let filter = RenameFilter {
            string_representation: "file_*_name_*".to_string(),
            fixed_str: vec!["file_".to_string(), "_name_".to_string(), "".to_string()],
            wildcard_type: vec![WildcardType::String, WildcardType::String],
            ..Default::default()
        };
        assert!(filter.does_fulfill("file_123_name_456"), "Matches the fixed strings with wildcards in between.");
        assert!(!filter.does_fulfill("file_123_name"), "Does not match due to missing second wildcard.");
        assert!(!filter.does_fulfill("file_name_456_extra"), "Does not match due to extra characters.");
    }

    #[test]
    fn test_rename_filter_new_with_string_wildcards() {
        let filter = RenameFilter::new("file_*_name_*".to_string(), '*');
        assert_eq!(filter.fixed_str, vec!["file_", "_name_", ""], "Fixed_str should include an empty string at the end for the last wildcard.");
        assert_eq!(filter.wildcard_type.len(), 2, "Two wildcards should be detected.");
        assert!(matches!(filter.wildcard_type[0], WildcardType::String), "First wildcard is a string.");
        assert!(matches!(filter.wildcard_type[1], WildcardType::String), "Second wildcard is a string.");
        assert!(filter.counters.is_empty(), "No counters should be present.");
    }

    #[test]
    fn test_rename_filter_new_with_counters_no_spaces() {
        let filter = RenameFilter::new("file_{1:1}_name_{10:5}".to_string(), '*');
        assert_eq!(filter.fixed_str, vec!["file_", "_name_", ""], "Fixed_str should include an empty string at the end for the last wildcard.");
        assert_eq!(filter.wildcard_type.len(), 2, "Two wildcards should be detected.");
        assert!(matches!(filter.wildcard_type[0], WildcardType::Counter), "First wildcard is a counter.");
        assert!(matches!(filter.wildcard_type[1], WildcardType::Counter), "Second wildcard is a counter.");
        assert_eq!(filter.counters.len(), 2, "Two counters should be present.");
        assert_eq!(filter.counters[0].counter, 1, "First counter starts at 1.");
        assert_eq!(filter.counters[0].increment, 1, "First counter increments by 1.");
        assert_eq!(filter.counters[1].counter, 10, "Second counter starts at 10.");
        assert_eq!(filter.counters[1].increment, 5, "Second counter increments by 5.");
    }

    #[test]
    fn test_rename_filter_new_with_counters_with_spaces() {
        let filter = RenameFilter::new("file_{  1  :  1  }_name_{  10  :  5  }".to_string(), '*');
        assert_eq!(filter.fixed_str, vec!["file_", "_name_", ""], "Fixed_str should include an empty string at the end for the last wildcard.");
        assert_eq!(filter.wildcard_type.len(), 2, "Two wildcards should be detected.");
        assert!(matches!(filter.wildcard_type[0], WildcardType::Counter), "First wildcard is a counter.");
        assert!(matches!(filter.wildcard_type[1], WildcardType::Counter), "Second wildcard is a counter.");
        assert_eq!(filter.counters.len(), 2, "Two counters should be present.");
        assert_eq!(filter.counters[0].counter, 1, "First counter starts at 1.");
        assert_eq!(filter.counters[0].increment, 1, "First counter increments by 1.");
        assert_eq!(filter.counters[1].counter, 10, "Second counter starts at 10.");
        assert_eq!(filter.counters[1].increment, 5, "Second counter increments by 5.");
    }
}

