use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
println("Hello world!")
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Julia;

impl LanguageConfig for Julia {
    fn id(&self) -> String {
        "julia".to_string()
    }

    fn name(&self) -> String {
        "Julia".to_string()
    }

    fn file_extension(&self) -> String {
        "jl".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/julia".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/julia:latest".to_string(),
            version_command: "julia --version".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" space="preserve" viewBox="0 0 383.37 258.84" {
                g fill="#252525" {
                    path d="M77.642 209.377V99.92l-32.427 8.921v116.397q0 9.204-1.416 12.39t-4.107 3.186q-1.274 0-2.832-.992-1.557-.99-3.823-3.964-1.982-2.69-5.027-5.452-3.044-2.76-8.142-2.761-6.797 0-10.832 3.398T5 239.397q0 5.947 7.222 10.195 7.221 4.248 20.815 4.248 10.055 0 18.267-1.628 8.212-1.63 14.09-6.443 5.876-4.814 9.062-13.523t3.186-22.869zM122.68 103.034H90.393v74.907q0 6.939 2.903 13.027 2.903 6.09 8 10.62 5.099 4.532 11.895 7.151 6.797 2.62 14.727 2.62 6.796 0 14.018-3.257 7.221-3.256 13.594-8.638v9.913h32.285V103.034h-32.285v76.748q-3.682 5.664-8.638 9.416t-8.638 3.753q-3.255 0-6.088-1.204-2.832-1.203-4.956-3.186t-3.328-4.744-1.204-5.876zM232.676 209.377V51.35l-32.143 8.92v149.107zM245.534 108.84v100.537h32.285V99.92zM345.943 154.436v30.727q-4.957 3.682-8.992 6.09-4.036 2.406-8 2.406-1.983 0-3.682-1.203-1.7-1.205-3.116-3.186-1.416-1.983-2.194-4.744a21 21 0 0 1-.78-5.735q0-3.823 2.408-7.363t6.301-6.655q3.894-3.116 8.638-5.735a102 102 0 0 1 9.416-4.602m32.426 54.941v-79.722q0-6.654-2.549-12.036-2.548-5.38-7.93-9.204-5.38-3.822-13.664-5.876-8.284-2.053-19.753-2.053-9.345 0-17.7 1.982-8.355 1.983-14.727 5.38-6.372 3.4-10.124 8.143t-3.753 10.266q0 5.947 4.248 9.841t11.045 3.894q4.39 0 7.293-1.274t4.46-3.398 2.195-4.957a26.4 26.4 0 0 0 .637-5.805q0-5.24 2.974-8.921 2.973-3.681 10.62-3.682 6.513 0 10.408 4.248t3.894 14.444v10.478l-3.54.85a596 596 0 0 0-13.17 4.177 164 164 0 0 0-12.814 4.815q-6.16 2.62-11.541 5.734-5.38 3.115-9.416 7.08-4.036 3.966-6.373 8.921-2.336 4.956-2.336 11.045 0 5.947 2.195 10.974a25 25 0 0 0 6.301 8.709q4.106 3.681 9.983 5.806 5.876 2.124 13.24 2.124 5.38 0 9.345-.78 3.965-.778 7.151-2.194t5.735-3.328q2.55-1.91 5.24-4.177v8.496z" {
                    }
                }
                g stroke-width="3.07" transform="matrix(1.25 0 0 -1.25 0 258.84)" {
                    circle cx="48.842" cy="149.57" r="16" fill="#6682df" stroke="#4063d8" {
                    }
                    circle cx="211.131" cy="149.57" r="16" fill="#d5635c" stroke="#cb3c33" {
                    }
                    circle cx="232.131" cy="185.57" r="16" fill="#60ad51" stroke="#389826" {
                    }
                    circle cx="253.131" cy="149.57" r="16" fill="#aa79c1" stroke="#9558b2" {
                    }
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!("julia {}", main_file.display()),
        }
    }
}
