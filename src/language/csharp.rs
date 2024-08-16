use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use crate::utils;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
using System;
using System.Collections.Generic;
using System.Linq;

class MainClass {
    static void Main() {
        Console.WriteLine("Hello World!");
    }
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Csharp;

impl LanguageConfig for Csharp {
    fn id(&self) -> String {
        "csharp".to_string()
    }

    fn name(&self) -> String {
        "C#".to_string()
    }

    fn file_extension(&self) -> String {
        "cs".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/csharp".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/csharp:latest".to_string(),
            version_command: "mcs --version".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" space="preserve" viewBox="0 0 512 512" {
                path d="m233.274 286.089 89.802 27.145q-9.053 37.781-28.5 63.107c-12.975 16.894-29.071 29.638-48.297 38.233q-28.841 12.894-73.412 12.894-54.058 0-88.33-15.711c-22.852-10.477-42.563-28.896-59.155-55.271Q.5 316.927.5 255.208c0-54.855 14.594-97.016 43.769-126.479 29.18-29.461 70.464-44.197 123.855-44.197 41.776 0 74.614 8.451 98.51 25.336 23.91 16.892 41.659 42.834 53.273 77.816l-90.481 20.131c-3.164-10.098-6.485-17.492-9.95-22.168q-8.598-11.759-21.043-18.095c-8.294-4.22-17.564-6.337-27.82-6.337-23.23 0-41.024 9.343-53.391 28.021-9.342 13.861-14.018 35.626-14.018 65.294q-.001 55.132 16.732 75.584 16.741 20.44 47.059 20.439 29.397.002 44.449-16.516c10.03-10.997 17.307-26.991 21.83-47.948m252.071-46.83-6.854 34.292H511.5v37.262h-40.452l-9.5 47.522h-38.41l9.527-47.522h-29.769l-9.595 47.522h-38.14l9.527-47.522h-18.572v-37.262h26.047l6.876-34.292h-32.923v-37.262h40.398l9.688-48.332h38.409l-9.752 48.332h29.625l9.694-48.332h38.273l-9.657 48.332H511.5v37.262zm-38.328 0h-29.68l-6.921 34.292h29.724z" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, other_files: Vec<PathBuf>) -> RunInstructions {
        let other_source_files = utils::filter_by_extension(other_files, "cs");

        RunInstructions {
            build_commands: vec![format!(
                "mcs -out:a.exe {} {}",
                main_file.display(),
                utils::join_files(other_source_files),
            )],
            run_command: "mono a.exe".to_string(),
        }
    }
}
