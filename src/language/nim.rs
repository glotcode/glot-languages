use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
echo("Hello World!")
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nim;

impl LanguageConfig for Nim {
    fn id(&self) -> String {
        "nim".to_string()
    }

    fn name(&self) -> String {
        "Nim".to_string()
    }

    fn file_extension(&self) -> String {
        "nim".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/nim".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/nim:latest".to_string(),
            version_command: "nim --version | head -n 1".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" data-name="nim-crown" viewBox="0 0 290 201" {
                path d="M217 126.5c-10.4 5.3-25 7.5-25 7.5l-47-27.5L98 134s-14.6-2.2-25-7.5c-15.8-8.5-33-29.7-38-38 0 0 13.1 36.6 22 62 19.2 29.9 53.4 45.4 88 45.5 34.6-.1 68.8-15.6 88-45.5 8.9-25.4 22-62 22-62-5 8.3-22.2 29.5-38 38" style="fill:#ffe953" {
                }
                path d="M250 50c-4.3-4.6-10.5-8.7-18-12.5-4.7-9-12.5-25-12.5-25s-7.8 7.3-17 14.2c-11.8-2.9-25.1-4.7-39.2-5.7C154.1 12.6 145 3.8 145 3.8s-9.1 8.8-18.3 17.2c-14.1 1-27.4 2.8-39.2 5.7-9.2-6.9-17-14.2-17-14.2s-7.8 16-12.5 25c-7.5 3.8-13.7 7.9-18 12.5-6.8-3.2-15-7-15-7 9 21.5 15.5 43.1 32 56 13.2-24.7 50.5-36 88-35.3 37.5-.7 74.8 10.6 88 35.3 16.5-12.9 23-34.5 32-56 0 0-8.2 3.8-15 7" style="fill:#f3d400" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!(
                "nim --hints:off --verbosity:0 compile --run {}",
                main_file.display()
            ),
        }
    }
}
