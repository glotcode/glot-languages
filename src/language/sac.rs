use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
int main () {
    StdIO::printf ("Hello World!");
    return 0;
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sac;

impl LanguageConfig for Sac {
    fn id(&self) -> String {
        "sac".to_string()
    }

    fn name(&self) -> String {
        "SaC".to_string()
    }

    fn file_extension(&self) -> String {
        "sac".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/sac".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/sac:latest".to_string(),
            version_command: "sac2c -V | head -n 1".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120" {
                path d="M40.215 36.816q-11.643 0-19.758 5.528-7.997 5.527-12.23 15.289-4.117 9.76-4.116 22.46 0 13.173 3.881 22.934 3.88 9.645 11.643 14.936 7.761 5.176 19.404 5.176 11.407 0 20.7-4.235v-14.7q-5.059 1.999-9.645 3.292a36 36 0 0 1-9.174 1.176q-9.76 0-14.465-7.408-4.586-7.41-4.586-21.051 0-13.054 4.703-20.934 4.823-7.997 13.996-7.998 4.234 0 8.467 1.53a48 48 0 0 1 8.233 3.646l5.41-13.996q-10.938-5.645-22.463-5.645" aria-label="C" style="font-weight:700;font-stretch:semi-condensed;font-size:117.604px;line-height:100%;font-family:\"NotoSans Nerd Font Mono\";-inkscape-font-specification:\"NotoSans Nerd Font Mono, Bold Semi-Condensed\";letter-spacing:0;word-spacing:0;stroke-width:2.94009px" transform="matrix(1.06223 0 0 .94142 3.522 -5.544)" {
                }
                path d="M74.996 15.164q-1.329 0-2.953.22-1.55.149-2.584.37v7.605q.739-.147 1.7-.295a15 15 0 0 1 2.14-.148q2.953 0 4.578 1.328 1.697 1.33 3.1 4.947l1.55 4.135L65.473 71.72h9.597l8.047-18.754a46 46 0 0 0 1.774-4.65q.885-2.585 1.476-4.727h.295q.368 1.772 1.254 4.504.887 2.657 1.773 5.168l4.207 11.812q1.257 3.545 3.25 5.465 2.068 1.92 5.907 1.92 1.403 0 3.027-.297 1.697-.296 2.51-.738v-7.088q-1.256.37-2.363.37-1.477 0-2.659-1.18-1.18-1.256-2.51-4.874L89.91 27.863q-1.476-4.134-3.322-6.941-1.772-2.88-4.504-4.281-2.732-1.477-7.088-1.477" aria-label="λ" style="font-weight:600;font-size:73.8305px;line-height:100%;font-family:\"NotoSans Nerd Font Mono\";-inkscape-font-specification:\"NotoSans Nerd Font Mono, Semi-Bold\";letter-spacing:0;word-spacing:0;fill:#ffc02e;stroke-width:1.84577px" transform="translate(3.522 -5.544)" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![format!("sac2c -t seq -o a.out {}", main_file.display())],
            run_command: "./a.out".to_string(),
        }
    }
}
