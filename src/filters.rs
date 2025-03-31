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

enum WildcardType{
    Counter,
    String,
}

pub struct RenameFilter{
    string_representation: String,
    wildcard_char: char,
    fixed_str: Vec<String>,
    counters: Vec<Counter>,
    wildcard_type: Vec<WildcardType>,
}
}

impl RenameFilter{
    pub fn new(string_representation: String, wildcard_char: char){
        
    }

    pub fn get_fixed_str(&self) -> Vec<String>{
        vec![]
    }

    pub fn does_fulfill(&self, filename: &str) -> bool{
        false
    }

    pub fn collect_wildcards(&mut self, filename: &str) -> Vec<String>{
        vec![]
    }
}

impl default::Default for RenameFilter{
    fn default() -> Self{
        RenameFilter{
            string_representation: "".to_string(),
            wildcard_char: '*',
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
            fixed_str: vec!["file_".to_string(), "_name_".to_string()],
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
        assert_eq!(filter.fixed_str, vec!["file_", "_name_"], "Fixed strings should be extracted correctly.");
        assert_eq!(filter.wildcard_type.len(), 2, "Two wildcards should be detected.");
        assert!(matches!(filter.wildcard_type[0], WildcardType::String), "First wildcard is a string.");
        assert!(matches!(filter.wildcard_type[1], WildcardType::String), "Second wildcard is a string.");
        assert!(filter.counters.is_empty(), "No counters should be present.");
    }

    #[test]
    fn test_rename_filter_new_with_counters_no_spaces() {
        let filter = RenameFilter::new("file_{1:1}_name_{10:5}".to_string(), '*');
        assert_eq!(filter.fixed_str, vec!["file_", "_name_"], "Fixed strings should be extracted correctly.");
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
        assert_eq!(filter.fixed_str, vec!["file_", "_name_"], "Fixed strings should be extracted correctly.");
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

