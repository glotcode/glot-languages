use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
section .data
    msg db "Hello World!", 0ah

section .text
    global _start
_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, msg
    mov rdx, 13
    syscall
    mov rax, 60
    mov rdi, 0
    syscall
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Assembly;

impl LanguageConfig for Assembly {
    fn id(&self) -> String {
        "assembly".to_string()
    }

    fn name(&self) -> String {
        "Assembly".to_string()
    }

    fn file_extension(&self) -> String {
        "asm".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/assembly_x86".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/assembly:latest".to_string(),
            version_command: "nasm --version".to_string(),
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![
                format!("nasm -f elf64 -o a.o {}", main_file.display()),
                "ld -o a.out a.o".to_string(),
            ],
            run_command: "./a.out".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1792 1792" {
                path d="m553 1399-50 50q-10 10-23 10t-23-10L-9 983q-10-10-10-23t10-23l466-466q10-10 23-10t23 10l50 50q10 10 10 23t-10 23L160 960l393 393q10 10 10 23t-10 23m591-1067L771 1623q-4 13-15.5 19.5T732 1645l-62-17q-13-4-19.5-15.5T648 1588l373-1291q4-13 15.5-19.5t23.5-2.5l62 17q13 4 19.5 15.5t2.5 24.5m657 651-466 466q-10 10-23 10t-23-10l-50-50q-10-10-10-23t10-23l393-393-393-393q-10-10-10-23t10-23l50-50q10-10 23-10t23 10l466 466q10 10 10 23t-10 23" {
                }
            }
        }
    }
}
