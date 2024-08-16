use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use crate::utils;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
IO.puts "Hello World!"
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Elixir;

impl LanguageConfig for Elixir {
    fn id(&self) -> String {
        "elixir".to_string()
    }

    fn name(&self) -> String {
        "Elixir".to_string()
    }

    fn file_extension(&self) -> String {
        "ex".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/elixir".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/elixir:latest".to_string(),
            version_command: "elixirc --version | tail -n 1".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" space="preserve" viewBox="0 0 100 100" {
                path d="M57.221 24.648c7.321 15.719 26.377 22.286 24.654 42.742-2.029 24.092-19.164 30.145-28.638 30.576s-27.561-2.907-32.514-25.623C15.161 46.828 39.456 7.638 53.452 2.039c-.538 6.352.819 16.277 3.769 22.609M44.761 89.69c6.407 1.331 11.317 2.256 11.899-.324.877-3.884-14.063-6.075-24.049-7.156 2.997 3.162 9.048 6.835 12.15 7.48" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, other_files: Vec<PathBuf>) -> RunInstructions {
        let other_source_files = utils::filter_by_extension(other_files, "c");

        RunInstructions {
            build_commands: vec![],
            run_command: format!(
                "elixirc {} {}",
                main_file.display(),
                utils::join_files(other_source_files),
            ),
        }
    }
}
