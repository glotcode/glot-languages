use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use crate::utils;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
fun main(args : Array<String>){
    println("Hello World!")
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Kotlin;

impl LanguageConfig for Kotlin {
    fn id(&self) -> String {
        "kotlin".to_string()
    }

    fn name(&self) -> String {
        "Kotlin".to_string()
    }

    fn file_extension(&self) -> String {
        "kt".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/kotlin".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/kotlin:latest".to_string(),
            version_command: "kotlinc -version 2>&1 | cut -c 7-".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" space="preserve" viewBox="0 0 500 500" {
                linearGradient id="kotlinGradient" x1="500.003" x2="-.097" y1="579.106" y2="1079.206" gradientTransform="translate(.097 -578.99)scale(.9998)" gradientUnits="userSpaceOnUse" {
                    stop offset=".003" style="stop-color:#e44857" {
                    }
                    stop offset=".469" style="stop-color:#c711e1" {
                    }
                    stop offset="1" style="stop-color:#7f52ff" {
                    }
                }
                path d="M500 500H0V0h500L250 250z" style="fill:url(#kotlinGradient)" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        let file_stem = main_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Main");

        RunInstructions {
            build_commands: vec![format!("kotlinc {}", main_file.display())],
            run_command: format!("kotlin {}Kt", utils::titlecase_ascii(file_stem)),
        }
    }
}
