pub struct CliParameters {
    pub Help: bool,
    pub Version: bool,
    pub Verbose: bool,
    pub DryRun: bool,
    pub Recursive: bool,
    pub Directory: std::path::PathBuf,
    pub Filter: String,
    pub Output: String,
    pub WildcardChar: char,
    pub PositionSelectWrapper: (char, char),
}

enum ParametersType {
    Help,
    Version,
    Vebose,
    DryRun,
    Recursive,
    Directory,
    Filter,
    Output,
    WildcardChar,
    PositionSelectWrapper,
    Error,
}

fn map_parameter_to_type(parameter: &String) -> ParametersType {
    match parameter.as_str() {
        "--help" => ParametersType::Help,
        "-h" => ParametersType::Help,
        "--version" => ParametersType::Version,
        "-V" => ParametersType::Version,
        "--verbose" => ParametersType::Vebose,
        "-v" => ParametersType::Vebose,
        "--dry-run" => ParametersType::DryRun,
        "--recursive" => ParametersType::Recursive,
        "-r" => ParametersType::Recursive,
        "--directory" => ParametersType::Directory,
        "-d" => ParametersType::Directory,
        "--filter" => ParametersType::Filter,
        "-f" => ParametersType::Filter,
        "--new-name" => ParametersType::Output,
        "-n" => ParametersType::Output,
        "--wildcard-char" => ParametersType::WildcardChar,
        "--position-select-wrapper" => ParametersType::PositionSelectWrapper,
        _ => ParametersType::Error,
    }
}

impl PartialEq for ParametersType{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParametersType::Help, ParametersType::Help) => true,
            (ParametersType::Version, ParametersType::Version) => true,
            (ParametersType::Vebose, ParametersType::Vebose) => true,
            (ParametersType::DryRun, ParametersType::DryRun) => true,
            (ParametersType::Recursive, ParametersType::Recursive) => true,
            (ParametersType::Directory, ParametersType::Directory) => true,
            (ParametersType::Filter, ParametersType::Filter) => true,
            (ParametersType::Output, ParametersType::Output) => true,
            (ParametersType::WildcardChar, ParametersType::WildcardChar) => true,
            (ParametersType::PositionSelectWrapper, ParametersType::PositionSelectWrapper) => true,
            _ => false,
        }
    }
}

impl std::hash::Hash for ParametersType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Eq for ParametersType {
    
}

impl CliParameters {
    pub fn new(args: Vec<String>) -> Self {
        use ParametersType::*;
        let mut default =CliParameters {
            Help: false,
            Version: false,
            Verbose: false,
            DryRun: false,
            Recursive: false,
            Directory: std::path::PathBuf::new(),
            Filter: "".to_string(),
            Output: "".to_string(),
            WildcardChar: '*',
            PositionSelectWrapper: ('(', ')'),
        };
        let mut defaults_overriden :std::collections::HashSet<ParametersType> = std::collections::HashSet::new();
        let mut index = 1;
        while index < args.len(){
            let parameter_type = map_parameter_to_type(&args[index]);
            if defaults_overriden.contains(&parameter_type) {
                panic!("Duplicate parameter: {}", args[index]);
            }
            match &parameter_type{
                Help => default.Help = true,
                Version => default.Version = true,
                ParametersType::Vebose => default.Verbose = true,
                DryRun => {
                    default.DryRun = true;
                    default.Verbose = true;
                },
                Recursive => default.Recursive = true,
                Directory => {
                    index += 1;
                    if index < args.len() {
                        default.Directory = std::path::PathBuf::from(&args[index]);
                        if !default.Directory.exists(){
                            panic!("Directory does not exist.");
                        }
                    }
                    else {
                        panic!("Directory parameter requires a value.");
                    }
                },
                Filter => {
                    index +=1;
                    if index < args.len() {
                        default.Filter = args[index].clone();
                    }
                    else {
                        panic!("Filter parameter requires a value.");
                    }
                },
                Output => {
                    index += 1;
                    if index < args.len() {
                        default.Output = args[index].clone();
                    }
                    else {
                        panic!("Output parameter requires a value.");
                    }
                },
                WildcardChar => {
                    index += 1;
                    if index < args.len() {
                        let wildcard_char = &args[index];
                        if wildcard_char.len() == 1 {
                            default.WildcardChar = wildcard_char.chars().next().unwrap();
                        }
                        else {
                            panic!("WildcardChar parameter must be a single character.");
                        }
                    }
                    else {
                        panic!("WildcardChar parameter requires a value.");
                    }
                },
                PositionSelectWrapper => {
                    index += 1;
                    if index < args.len() {
                        let position_select_wrapper = &args[index];
                        if position_select_wrapper.len() == 2 {
                            default.PositionSelectWrapper = (
                                position_select_wrapper.chars().nth(0).unwrap(),
                                position_select_wrapper.chars().nth(1).unwrap(),
                            );
                        }
                        else {
                            panic!("PositionSelectWrapper parameter must be two characters.");
                        }
                    }
                    else {
                        panic!("PositionSelectWrapper parameter requires a value.");
                    }
                },
                Error => {
                    panic!("Invalid parameter: {}", args[index]);
                },
            }
            defaults_overriden.insert(parameter_type);
            
            index += 1;
        }
        default   
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_help_parameter() {
        let args = vec!["program".to_string(), "--help".to_string()];
        let params = CliParameters::new(args);
        assert!(params.Help, "Help flag should be set to true.");
    }

    #[test]
    fn test_version_parameter() {
        let args = vec!["program".to_string(), "--version".to_string()];
        let params = CliParameters::new(args);
        assert!(params.Version, "Version flag should be set to true.");
    }

    #[test]
    fn test_dry_run_parameter() {
        let args = vec!["program".to_string(), "--dry-run".to_string()];
        let params = CliParameters::new(args);
        assert!(params.DryRun, "DryRun flag should be set to true.");
    }

    #[test]
    fn test_recursive_parameter() {
        let args = vec!["program".to_string(), "--recursive".to_string()];
        let params = CliParameters::new(args);
        assert!(params.Recursive, "Recursive flag should be set to true.");
    }

    #[test]
    fn test_directory_parameter() {
        let current_dir = env::current_dir().unwrap();
        let args = vec![
            "program".to_string(),
            "--directory".to_string(),
            current_dir.to_str().unwrap().to_string(),
        ];
        let params = CliParameters::new(args);
        assert_eq!(
            params.Directory,
            current_dir,
            "Directory should be set to the current execution root."
        );
    }

    #[test]
    fn test_filter_parameter() {
        let args = vec![
            "program".to_string(),
            "--filter".to_string(),
            "file_*".to_string(),
        ];
        let params = CliParameters::new(args);
        assert_eq!(
            params.Filter, "file_*",
            "Filter should be set to 'file_*'."
        );
    }

    #[test]
    fn test_output_parameter() {
        let args = vec![
            "program".to_string(),
            "--output".to_string(),
            "output_file".to_string(),
        ];
        let params = CliParameters::new(args);
        assert_eq!(
            params.Output, "output_file",
            "Output should be set to 'output_file'."
        );
    }

    #[test]
    fn test_wildcard_char_parameter() {
        let args = vec![
            "program".to_string(),
            "--wildcard-char".to_string(),
            "?".to_string(),
        ];
        let params = CliParameters::new(args);
        assert_eq!(
            params.WildcardChar, '?',
            "WildcardChar should be set to '?'."
        );
    }

    #[test]
    fn test_position_select_wrapper_parameter() {
        let args = vec![
            "program".to_string(),
            "--position-select-wrapper".to_string(),
            "[]".to_string(),
        ];
        let params = CliParameters::new(args);
        assert_eq!(
            params.PositionSelectWrapper,
            ('[', ']'),
            "PositionSelectWrapper should be set to ('[', ']')."
        );
    }

    #[test]
    fn test_multiple_parameters() {
        let current_dir = env::current_dir().unwrap();
        let args = vec![
            "program".to_string(),
            "--help".to_string(),
            "--directory".to_string(),
            current_dir.to_str().unwrap().to_string(),
            "--filter".to_string(),
            "file_*".to_string(),
        ];
        let params = CliParameters::new(args);
        assert!(params.Help, "Help flag should be set to true.");
        assert_eq!(
            params.Directory,
            current_dir,
            "Directory should be set to the current execution root."
        );
        assert_eq!(
            params.Filter, "file_*",
            "Filter should be set to 'file_*'."
        );
    }

    #[test]
    fn test_parameters_in_different_order() {
        let current_dir = env::current_dir().unwrap();
        let args = vec![
            "program".to_string(),
            "--filter".to_string(),
            "file_*".to_string(),
            "--directory".to_string(),
            current_dir.to_str().unwrap().to_string(),
            "--help".to_string(),
        ];
        let params = CliParameters::new(args);
        assert!(params.Help, "Help flag should be set to true.");
        assert_eq!(
            params.Directory,
            current_dir,
            "Directory should be set to the current execution root."
        );
        assert_eq!(
            params.Filter, "file_*",
            "Filter should be set to 'file_*'."
        );
    }

    #[test]
    fn test_duplicate_parameter_error() {
        let args = vec![
            "program".to_string(),
            "--help".to_string(),
            "--help".to_string(),
        ];
        let result = std::panic::catch_unwind(|| CliParameters::new(args));
        assert!(result.is_err(), "Duplicate parameters should cause an error.");
    }

    #[test]
    fn test_invalid_position_select_wrapper() {
        let args = vec![
            "program".to_string(),
            "--position-select-wrapper".to_string(),
            "abc".to_string(),
        ];
        let result = std::panic::catch_unwind(|| CliParameters::new(args));
        assert!(result.is_err(), "Invalid PositionSelectWrapper should cause an error.");
    }

    #[test]
    fn test_invalid_wildcard_char() {
        let args = vec![
            "program".to_string(),
            "--wildcard-char".to_string(),
            "**".to_string(),
        ];
        let result = std::panic::catch_unwind(|| CliParameters::new(args));
        assert!(result.is_err(), "Invalid WildcardChar should cause an error.");
    }

    #[test]
    fn test_directory_exists() {
        let current_dir = env::current_dir().unwrap();
        let args = vec![
            "program".to_string(),
            "--directory".to_string(),
            current_dir.to_str().unwrap().to_string(),
        ];
        let params = CliParameters::new(args);
        assert!(
            params.Directory.exists(),
            "Directory should exist."
        );
        assert!(
            params.Directory.is_dir(),
            "Directory should be a valid directory."
        );
    }
}