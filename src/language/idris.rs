use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
module Main

main : IO ()
main = putStrLn "Hello World!"
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Idris;

impl LanguageConfig for Idris {
    fn id(&self) -> String {
        "idris".to_string()
    }

    fn name(&self) -> String {
        "Idris".to_string()
    }

    fn file_extension(&self) -> String {
        "idr".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/plain_text".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/idris:latest".to_string(),
            version_command: "idris2 --version".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="-100 -100 595 841" {
                path d="M79.077 85.235c73.08 22.117 91.775 40.337 117.469 106.37-4.984-80.517-37.208-114.082-117.469-106.37 18.827-67.89 0 0 0 0M-22.208 211.874C25.235 226.484 80.324 238.16 100.68 328.8c8.404-113.077-45.134-118.838-122.888-116.925 90.649-302.18 0 0 0 0" style="fill:#8a0819;fill-opacity:1;stroke:none" transform="translate(22)" {
                }
                path d="M9.848 139.77c71.289 14.732 116.732 34.088 143.292 126.084 6.093-118.68-59.046-130.596-143.292-126.084 78.39-131.188 0 0 0 0" style="fill:#8a0819;fill-opacity:1;stroke:none" transform="translate(22)" {
                }
                path d="M103.33.379c389.022 253.662-86.412 258.144 17 552.638l61.216 17.827C12.903 342.22 556.898 224.698 103.33.379" style="fill:#8a0819;fill-opacity:1;stroke:none" transform="translate(22)" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![format!(
                "idris2 -o a.out --output-dir . {}",
                main_file.display()
            )],
            run_command: "./a.out".to_string(),
        }
    }
}
