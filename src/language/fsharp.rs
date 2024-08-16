use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use crate::utils;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
printfn "Hello World!"
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fsharp;

impl LanguageConfig for Fsharp {
    fn id(&self) -> String {
        "fsharp".to_string()
    }

    fn name(&self) -> String {
        "F#".to_string()
    }

    fn file_extension(&self) -> String {
        "fs".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/fsharp".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/fsharp:latest".to_string(),
            version_command: "fsharpc --version 2>/dev/null | head -n 1".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128" {
                path d="M5 63 61 7v28L33 63l28 28v28z" style="fill:#378bba" {
                }
                path d="m41 63 20-20v40z" style="fill:#378bba" {
                }
                path d="M123 63 65 7v28l28 28-28 28v28z" style="fill:#30b9db" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, other_files: Vec<PathBuf>) -> RunInstructions {
        let other_source_files = utils::filter_by_extension(other_files, "fs")
            .into_iter()
            .rev()
            .collect::<Vec<PathBuf>>();

        RunInstructions {
            build_commands: vec![format!(
                "fsharpc --out:a.exe {} {}",
                utils::join_files(other_source_files),
                main_file.display()
            )],
            run_command: "mono a.exe".to_string(),
        }
    }
}
