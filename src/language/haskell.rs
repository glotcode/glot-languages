use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
main = putStrLn "Hello World!"
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Haskell;

impl LanguageConfig for Haskell {
    fn id(&self) -> String {
        "haskell".to_string()
    }

    fn name(&self) -> String {
        "Haskell".to_string()
    }

    fn file_extension(&self) -> String {
        "hs".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/haskell".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/haskell:latest".to_string(),
            version_command: "ghc --version".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 181" preserveAspectRatio="xMidYMid" {
                path fill="#F97E2F" d="m0 180.664 60.222-90.332L0 0h45.166l60.222 90.332-60.222 90.332z" {
                }
                path fill="#95653A" d="m60.222 180.664 60.222-90.332L60.222 0h45.166L225.83 180.664h-45.166l-37.637-56.457-37.639 56.457z" {
                }
                path fill="#F97E2F" d="m205.757 127.971-20.072-30.11 70.257-.002v30.112zM175.647 82.805l-20.074-30.11 100.369-.002v30.112z" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!("runghc {}", main_file.display()),
        }
    }
}
