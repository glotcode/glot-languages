use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use crate::utils;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
implement main0 () = print"Hello World!\n"
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ats;

impl LanguageConfig for Ats {
    fn id(&self) -> String {
        "ats".to_string()
    }

    fn name(&self) -> String {
        "ATS".to_string()
    }

    fn file_extension(&self) -> String {
        "dats".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/ats".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/ats:latest".to_string(),
            version_command: "patscc -vats".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" version="1.0" viewBox="0 0 400 400" {
                path fill="red" d="M81.5 100.4c-14.8 3.8-30.7 13.8-48.3 30.2l-8.4 7.7 11.3 11.3 11.3 11.3 5.5-5.2c9.6-9.1 20.8-17.4 28.5-21.3 6.6-3.2 8.2-3.6 13.3-3.2 6.9.5 10.1 2.5 13.5 8.3 3.3 5.7 6.8 14.4 7.4 18.7.6 3.5-1.3 6.7-43 71.8-24 37.4-43.6 68.4-43.6 68.9 0 .9 24.5 17.1 25.8 17.1.4 0 14-20.8 30.2-46.2s30.2-47.1 31.1-48.2c1.5-1.8 1.6-1 2.2 11 2.2 45.8 15.6 70 44.4 80.2l7.8 2.7 95.3.3 95.2.3V284l-93.7-.2-93.8-.3-4.1-2.2c-8.8-4.7-14-13.6-17-29.3-2-10-3.4-45.9-2.4-59 2.8-34.8-8.7-70.6-27-84.4-8.7-6.5-15.6-8.8-27-9.2-5.6-.2-11.5.2-14.5 1" {
                }
                path fill="#00f" d="M180 125v20h67v16h-80v109h186V161h-80v-16h67v-40H180zm67 90.5V230h-54v-29h54zm80 0V230h-54v-29h54z" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, other_files: Vec<PathBuf>) -> RunInstructions {
        let other_source_files = utils::filter_by_extension(other_files, "dats");

        RunInstructions {
            build_commands: vec![format!(
                "patscc -o a.out {} {}",
                main_file.display(),
                utils::join_files(other_source_files),
            )],
            run_command: "./a.out".to_string(),
        }
    }
}
