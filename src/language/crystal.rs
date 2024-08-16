use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
puts "Hello World!"
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Crystal;

impl LanguageConfig for Crystal {
    fn id(&self) -> String {
        "crystal".to_string()
    }

    fn name(&self) -> String {
        "Crystal".to_string()
    }

    fn file_extension(&self) -> String {
        "cr".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/crystal".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/crystal:latest".to_string(),
            version_command: "crystal --version | head -n 1".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" space="preserve" viewBox="-134 328.3 99.409 99.1" {
                path d="m-15.6 410.7-36 35.9c-.1.1-.3.1-.6.1l-49.1-13.1c-.1 0-.3-.1-.4-.4l-13.1-49c0-.1 0-.4.1-.6l36-35.9c.1-.1.3-.1.6-.1l49.1 13.2c.1 0 .3.1.4.4l13.1 49c.2.2.1.4-.1.5m-48.1-39-48.2 13q-.15 0 0 .3l35.3 35.3c.1.1.1 0 .3 0l13-48.1c-.2-.5-.4-.5-.4-.5" style="fill:#010101" transform="translate(-19.2 -19.3)" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!("crystal run {}", main_file.display()),
        }
    }
}
