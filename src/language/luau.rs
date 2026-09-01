use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
print("Hello World!")
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Luau;

impl LanguageConfig for Luau {
    fn id(&self) -> String {
        "luau".to_string()
    }

    fn name(&self) -> String {
        "Luau".to_string()
    }

    fn file_extension(&self) -> String {
        "luau".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/lua".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/luau:latest".to_string(),
            // Official luau CLI has no --version yet; _VERSION prints "Luau".
            version_command: "echo 'print(_VERSION)' | luau /dev/stdin".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" preserveAspectRatio="xMidYMid" {
                path fill="#00A2FF" d="M128 16c61.856 0 112 50.144 112 112s-50.144 112-112 112S16 189.856 16 128 66.144 16 128 16" {
                }
                path fill="#003A70" d="M128 56c39.764 0 72 32.236 72 72s-32.236 72-72 72-72-32.236-72-72 32.236-72 72-72" {
                }
                path fill="#FFF" d="M176 80c0-17.673-14.327-32-32-32s-32 14.327-32 32 14.327 32 32 32 32-14.327 32-32" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!("luau {}", main_file.display()),
        }
    }
}
