use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("{s}\n", .{"Hello World!"});
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Zig;

impl LanguageConfig for Zig {
    fn id(&self) -> String {
        "zig".to_string()
    }

    fn name(&self) -> String {
        "Zig".to_string()
    }

    fn file_extension(&self) -> String {
        "zig".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/plain_text".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/zig:latest".to_string(),
            version_command: "zig version".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 154 140" {
                g fill="#F7A41D" {
                    path d="M46 22 28 44l-9-14z" {
                    }
                    path d="M46 22 33 33l-5 11h-6v51h9l-11 5-8 17H0V22z" shape-rendering="crispEdges" {
                    }
                    path d="m31 95-19 22-8-11zM56 22l6 14-25 8z" {
                    }
                    path d="M56 22h55v22H37l19-12z" shape-rendering="crispEdges" {
                    }
                    path d="m116 95-19 22-7-13z" {
                    }
                    path d="m116 95-16 9-3 13H42V95z" shape-rendering="crispEdges" {
                    }
                    path d="M150 0 52 117 3 140l98-118zM141 22l-1 18-18 5z" {
                    }
                    path d="M153 22v95h-47l14-12 5-10h6V45h-9l10-9 9-14z" shape-rendering="crispEdges" {
                    }
                    path d="m125 95 5 15-24 7z" {
                    }
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!("zig run {}", main_file.display()),
        }
    }
}
