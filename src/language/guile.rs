use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
(display "Hello World!")
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Guile;

impl LanguageConfig for Guile {
    fn id(&self) -> String {
        "guile".to_string()
    }

    fn name(&self) -> String {
        "Guile".to_string()
    }

    fn file_extension(&self) -> String {
        "scm".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/scheme".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/guile:latest".to_string(),
            version_command: "guile --version | head -n 1".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 47.6 47.6" {
                path fill="#d0343f" d="M20.8 1a23 23 0 0 0 0 45.6V40a16.4 16.4 0 0 1 0-32.2z" {
                }
                path fill="#1a1a1a" d="M26.9 1v6.7a16.4 16.4 0 0 1 0 32.2v6.7a23 23 0 0 0 0-45.6" {
                }
                g style="line-height:0%;-inkscape-font-specification:\"URW Gothic L Semi-Bold\";text-align:center" text-anchor="middle" word-spacing="0" {
                    path fill="#1a1a1a" d="M90 1846v-14.5h54.4v2q0 16.3-11.5 28.7l-.5.4q-12.5 12.8-31.5 12.8-18.7 0-31.7-11.9-12.6-11.6-13.5-29v-2.3q0-17.1 11.7-29.5 11.9-12.2 29-13.3l3-.1q16 0 28.5 9.7 8.8 6.8 12.2 16.2H122q-8-11-23-11-12.5 0-20 8.2-6.5 6.7-7.6 17l-.2 3q0 12.9 9.4 21 8 7.2 19.4 7.2 13.7 0 21.5-10 1.7-2 3-4.7z" aria-label="G" font-family="URW Gothic L" font-size="112.6" font-weight="600" letter-spacing="0" style="line-height:1.25" transform="translate(-2.6 -461)scale(.2646)" {
                    }
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!(
                "guile --no-debug --fresh-auto-compile --no-auto-compile -s {}",
                main_file.display()
            ),
        }
    }
}
