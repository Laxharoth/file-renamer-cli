#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs;

    fn setup_test_directory() -> PathBuf {
        let test_dir = PathBuf::from("test_dir");
        if !test_dir.exists() {
            fs::create_dir(&test_dir).unwrap();
        }
        test_dir
    }

    fn cleanup_test_directory(test_dir: &PathBuf) {
        if test_dir.exists() {
            fs::remove_dir(test_dir).unwrap();
        }
    }

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
        let test_dir = setup_test_directory();
        let args = vec![
            "program".to_string(),
            "--directory".to_string(),
            test_dir.to_str().unwrap().to_string(),
        ];
        let params = CliParameters::new(args);
        assert_eq!(
            params.Directory,
            test_dir,
            "Directory should be set to the test directory."
        );
        cleanup_test_directory(&test_dir);
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
        let test_dir = setup_test_directory();
        let args = vec![
            "program".to_string(),
            "--help".to_string(),
            "--directory".to_string(),
            test_dir.to_str().unwrap().to_string(),
            "--filter".to_string(),
            "file_*".to_string(),
        ];
        let params = CliParameters::new(args);
        assert!(params.Help, "Help flag should be set to true.");
        assert_eq!(
            params.Directory,
            test_dir,
            "Directory should be set to the test directory."
        );
        assert_eq!(
            params.Filter, "file_*",
            "Filter should be set to 'file_*'."
        );
        cleanup_test_directory(&test_dir);
    }

    #[test]
    fn test_parameters_in_different_order() {
        let test_dir = setup_test_directory();
        let args = vec![
            "program".to_string(),
            "--filter".to_string(),
            "file_*".to_string(),
            "--directory".to_string(),
            test_dir.to_str().unwrap().to_string(),
            "--help".to_string(),
        ];
        let params = CliParameters::new(args);
        assert!(params.Help, "Help flag should be set to true.");
        assert_eq!(
            params.Directory,
            test_dir,
            "Directory should be set to the test directory."
        );
        assert_eq!(
            params.Filter, "file_*",
            "Filter should be set to 'file_*'."
        );
        cleanup_test_directory(&test_dir);
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
        let test_dir = setup_test_directory();
        let args = vec![
            "program".to_string(),
            "--directory".to_string(),
            test_dir.to_str().unwrap().to_string(),
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
        cleanup_test_directory(&test_dir);
    }
}