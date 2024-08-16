use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use crate::utils;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
import std.stdio;

void main()
{
    writeln("Hello World!");
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct D;

impl LanguageConfig for D {
    fn id(&self) -> String {
        "d".to_string()
    }

    fn name(&self) -> String {
        "D".to_string()
    }

    fn file_extension(&self) -> String {
        "d".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/d".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/dlang:latest".to_string(),
            version_command: "dmd --version | head -n 1".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" viewBox="0 0 123.865 93.753" version="1.0" {
                defs {
                    linearGradient id="dGradientB" {
                        stop offset="0" style="stop-color:#fff;stop-opacity:1" {
                        }
                        stop offset="1" style="stop-color:#fff;stop-opacity:.33333334" {
                        }
                    }
                    linearGradient id="dGradientA" {
                        stop offset="0" style="stop-color:#f2f2f0;stop-opacity:.13541667" {
                        }
                        stop offset="1" style="stop-color:#eeeeec;stop-opacity:.39583334" {
                        }
                    }
                    linearGradient id="dGradientD" href="#dGradientA" x1="27.248" x2="44.496" y1="33.563" y2="47.031" gradientTransform="matrix(1 0 0 .99176 -.678 .501)" gradientUnits="userSpaceOnUse" spreadMethod="reflect" {
                    }
                    linearGradient id="dGradientE" href="#dGradientB" x1="24.482" x2="104.024" y1="30.994" y2="90.719" gradientTransform="matrix(.99719 0 0 .98872 -.497 .687)" gradientUnits="userSpaceOnUse" {
                    }
                    linearGradient id="dGradientF" href="#dGradientA" x1="27.248" x2="44.496" y1="33.563" y2="47.031" gradientTransform="matrix(1 0 0 -.99176 -.678 121.014)" gradientUnits="userSpaceOnUse" spreadMethod="reflect" {
                    }
                }
                g style="display:inline" transform="translate(-33.347 -44.392)scale(1.47509)" {
                    rect width="80.582" height="60.168" x="25.996" y="33.484" rx="7.694" ry="8.543" style="fill:#2e3436;fill-opacity:.2745098;fill-rule:nonzero;stroke:none" {
                    }
                    rect width="80.582" height="60.168" x="23.285" y="30.772" rx="7.694" ry="8.543" style="fill:#a40000;fill-opacity:1;fill-rule:nonzero;stroke:none" {
                    }
                    rect width="74.011" height="54.138" x="26.57" y="33.787" rx="5.221" ry="5.62" style="fill:url(#dGradientD);fill-opacity:1;fill-rule:nonzero;stroke:none" {
                    }
                    path d="M32.333 39.188c-.81.1-1.445.747-1.448 1.53l.051 39.977a2 2 0 0 0 0 .174 1.3 1.3 0 0 0 0 .27v.04c.01.037.03.077.042.115v.02c.01.038.01.078.021.115v.04q.03.058.063.115v.02c.028.039.071.079.103.115v.02q.03.058.063.116v.038c.04.03.082.05.124.077v.04c.03.029.051.049.083.076l.042.039c.03.03.05.05.083.077h.02q.116.097.25.174h.04q.061.03.125.058h.02q.062.03.125.057c.06.013.125.013.187.02.1.018.208.037.31.038h.166l15.31-.062c4.376-.007 7.307-.082 9.053-.303h.041c1.67-.232 3.44-.66 5.364-1.284 3.345-1.046 6.311-2.591 8.861-4.655 2.497-2 4.432-4.366 5.792-7.029s2.046-5.478 2.04-8.397c-.007-4.062-1.236-7.867-3.702-11.289-2.466-3.423-5.832-6.044-9.974-7.78-4.212-1.785-9.703-2.599-16.515-2.586l-16.533.024c-.07 0-.138-.008-.207 0m8.898 8.226 7.127-.01c3.33-.006 5.7.095 7.044.28 1.363.187 2.855.582 4.435 1.192 1.567.597 2.932 1.328 4.105 2.238v.038c3.228 2.471 4.75 5.441 4.756 9.373.007 4.027-1.463 7.163-4.607 9.793a15.3 15.3 0 0 1-3.23 2.036c-1.12.522-2.584.972-4.431 1.36-1.742.349-4.387.547-7.83.553l-7.334.01z" style="font-size:64px;font-style:normal;font-variant:normal;font-weight:400;font-stretch:normal;text-align:start;line-height:125%;writing-mode:lr-tb;text-anchor:start;fill:#eeeeec;fill-opacity:1;stroke:none;font-family:Gill Sans MT" {
                    }
                    path d="M89.368 35.648a5.969 5.472 0 1 1-11.938 0 5.969 5.472 0 1 1 11.938 0" style="fill:#eeeeec;fill-opacity:1;fill-rule:nonzero;stroke:none" transform="matrix(1.95002 0 0 1.95002 -82.918 -16.343)" {
                    }
                    rect width="78.006" height="57.75" x="24.572" y="31.981" rx="6.57" ry="7.306" style="fill:none;stroke:url(#dGradientE);stroke-width:1.34628034;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4;stroke-opacity:1;stroke-dasharray:none" {
                    }
                    rect width="80.582" height="60.168" x="23.285" y="30.772" rx="7.694" ry="8.543" style="fill:none;stroke:#323232;stroke-width:1.3558476;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4;stroke-opacity:1;stroke-dasharray:none" {
                    }
                    path d="M31.791 87.728H95.36c2.892 0 5.22-2.506 5.22-5.62v-9.001c-22.704-8.734-55.576-13.412-74.01-13.559v22.56c0 3.114 2.329 5.62 5.221 5.62" style="fill:url(#dGradientF);fill-opacity:1;fill-rule:nonzero;stroke:none" {
                    }
                    path d="M89.368 35.648a5.969 5.472 0 1 1-11.938 0 5.969 5.472 0 1 1 11.938 0" style="fill:#eeeeec;fill-opacity:1;fill-rule:nonzero;stroke:none;display:inline" transform="matrix(.62657 0 0 .62657 40.72 19.11)" {
                    }
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, other_files: Vec<PathBuf>) -> RunInstructions {
        let other_source_files = utils::filter_by_extension(other_files, "d");

        RunInstructions {
            build_commands: vec![format!(
                "dmd -ofa.out {} {}",
                main_file.display(),
                utils::join_files(other_source_files)
            )],
            run_command: "./a.out".to_string(),
        }
    }
}
