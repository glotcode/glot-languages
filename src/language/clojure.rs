use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
(println "Hello World!")
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clojure;

impl LanguageConfig for Clojure {
    fn id(&self) -> String {
        "clojure".to_string()
    }

    fn name(&self) -> String {
        "Clojure".to_string()
    }

    fn file_extension(&self) -> String {
        "clj".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/clojure".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/clojure:latest".to_string(),
            version_command: "clj --version".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 99" {
                circle cx="49.75" cy="49.5" r="48.5" fill="#fff" {
                }
                path fill="#5881d8" d="M39.3 6.22c12.41-3.11 26.15-.58 36.53 6.94 12.85 8.94 20.29 25.06 18.6 40.64-.77 6.31-5.03 12.21-11.06 14.44-4.16 1.73-8.73 1.54-13.14 1.56 10.54-10.13 11.18-28.47 1.22-39.2-7.85-9.28-21.7-12.08-32.8-7.44-7.38-4.36-16.82-4.48-24.38-.47 6.38-7.9 15.05-14.13 25.03-16.47" {
                }
                path fill="#90b4fe" d="M42.93 26.99c5.56-1.49 11.62-1.37 16.86 1.15 8.92 4.05 14.82 14 13.62 23.8-.56 6.7-4.49 12.59-9.6 16.75-4.24-1.98-6.28-6.39-8.15-10.39-4.9-10.18-5.43-22.28-12.73-31.31" {
                }
                path fill="#63b132" d="M12.3 33.3c4.81-4.81 12.03-6.4 18.61-5.24-5.69 5.43-9.47 12.97-9.45 20.93-.35 9.98 5.12 19.77 13.62 24.93 8.2 5.14 18.87 5.36 27.58 1.37 7.71 2.28 15.86 2.07 23.65.28C80.05 84 70.94 90.35 60.69 92.84c-12.67 3.19-26.69.4-37.13-7.47C12.16 77.09 5.12 63.11 5.44 49c-.29-5.94 2.78-11.58 6.86-15.7" {
                }
                path fill="#91dc47" d="M26.94 54c-1.97-8.94 2.26-18.41 9.51-23.76 5.54 3.47 7.78 9.9 10.1 15.67C43 53.4 38.44 60.46 35.94 68.42c-4.44-3.68-7.98-8.65-9-14.42M41.97 71.8c-.51-7.53 3.34-14.28 6.14-21 2.29 7.33 3.73 15.39 9.07 21.26-5.01 1.31-10.25 1.2-15.21-.26" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!("clj -M {}", main_file.display()),
        }
    }
}
