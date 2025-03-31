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
        assert_eq!(counter.count(), 0);
        assert_eq!(counter.count(), 1);
        assert_eq!(counter.count(), 2);

        let mut counter = Counter::new(10, 5);
        assert_eq!(counter.count(), 10);
        assert_eq!(counter.count(), 15);
        assert_eq!(counter.count(), 20);
    }

    #[test]
    fn test_collect_wildcards_no_wildcards() {
        let mut filter = RenameFilter {
            string_representation: "file_name".to_string(),
            ..Default::default()
        };
        let result = filter.collect_wildcards("file_name");
        assert!(result.is_empty());
        assert!(filter.wildcard_type.is_empty());
    }

    #[test]
    fn test_collect_wildcards_some_wildcards() {
        let mut filter = RenameFilter {
            string_representation: "file_*_name_*".to_string(),
            wildcard_type: vec![WildcardType::String, WildcardType::String],
            ..Default::default()
        };
        let result = filter.collect_wildcards("file_123_name_456");
        assert_eq!(result, vec!["123", "456"]);
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
        let result2 = filter.collect_wildcards("file_456_name");
        assert_eq!(result1, vec!["1"]);
        assert_eq!(result2, vec!["2"]);
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
        let result2 = filter.collect_wildcards("file_789_name_012");
        assert_eq!(result1, vec!["1", "10"]);
        assert_eq!(result2, vec!["2", "15"]);
    }

    #[test]
    fn test_does_fulfill_no_fixed_str() {
        let filter = RenameFilter {
            string_representation: "file_name".to_string(),
            fixed_str: vec!["file_name".to_string()],
            ..Default::default()
        };
        assert!(filter.does_fulfill("file_name"));
        assert!(!filter.does_fulfill("file_name_extra"));
    }

    #[test]
    fn test_does_fulfill_with_fixed_str() {
        let filter = RenameFilter {
            string_representation: "file_*_name".to_string(),
            fixed_str: vec!["file_".to_string(), "_name".to_string()],
            wildcard_type: vec![WildcardType::String],
            ..Default::default()
        };
        assert!(filter.does_fulfill("file_123_name"));
        assert!(filter.does_fulfill("file__name"));
        assert!(!filter.does_fulfill("file_name_extra"));
        assert!(!filter.does_fulfill("name_file"));
    }

    #[test]
    fn test_does_fulfill_partial_match() {
        let filter = RenameFilter {
            string_representation: "file_*_name_*".to_string(),
            fixed_str: vec!["file_".to_string(), "_name_".to_string()],
            wildcard_type: vec![WildcardType::String, WildcardType::String],
            ..Default::default()
        };
        assert!(filter.does_fulfill("file_123_name_456"));
        assert!(!filter.does_fulfill("file_123_name"));
        assert!(!filter.does_fulfill("file_name_456_extra"));
    }
}

