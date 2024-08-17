use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
say 'Hello World!';
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Raku;

impl LanguageConfig for Raku {
    fn id(&self) -> String {
        "raku".to_string()
    }

    fn name(&self) -> String {
        "Raku".to_string()
    }

    fn file_extension(&self) -> String {
        "raku".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/perl".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/raku:latest".to_string(),
            version_command: "raku --version | head -n 1".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" version="1.0" viewBox="22 40 948 693" {
                defs {
                    clipPath id="a" {
                        path d="M0-.1h792.1V612H0z" {
                        }
                    }
                }
                g clip-path="url(#a)" transform="matrix(1.25 0 0 -1.25 0 765)" {
                    path d="M304.2 75.7c-55.4-40.6-129.5-33.6-165.6 15.7-36 49.2-20.3 122 35.1 162.5 55.3 40.6 129.5 33.6 165.5-15.6 36.1-49.3 20.4-122.1-35-162.6" style="fill:#000;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M304.2 75.7c-55.4-40.6-129.5-33.6-165.6 15.7-36 49.2-20.3 122 35.1 162.5 55.3 40.6 129.5 33.6 165.5-15.6 36.1-49.3 20.4-122.1-35-162.6z" style="fill:none;stroke:#000;stroke-width:4.30158997;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M273.6 523.5c68.9-50.4 88.4-140.9 43.5-202.1-44.8-61.2-137-69.9-205.8-19.5-68.9 50.4-88.4 141-43.6 202.2 44.9 61.2 137 69.9 205.9 19.4" style="fill:#000;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M273.6 523.5c68.9-50.4 88.4-140.9 43.5-202.1-44.8-61.2-137-69.9-205.8-19.5-68.9 50.4-88.4 141-43.6 202.2 44.9 61.2 137 69.9 205.9 19.4z" style="fill:none;stroke:#000;stroke-width:7.1882;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M213.3 370.5c-25.8 21.2-27.3 62.2-3.3 91.4 24.1 29.3 64.5 35.9 90.3 14.6 25.9-21.2 27.4-62.1 3.3-91.4-24-29.3-64.4-35.8-90.3-14.6" style="fill:#0f0;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M213.3 370.5c-25.8 21.2-27.3 62.2-3.3 91.4 24.1 29.3 64.5 35.9 90.3 14.6 25.9-21.2 27.4-62.1 3.3-91.4-24-29.3-64.4-35.8-90.3-14.6" style="fill:none;stroke:none" {
                    }
                    path d="M191.7 375.8c-25.9 21.3-27.4 62.2-3.4 91.5 24.1 29.3 64.6 35.8 90.4 14.6 25.9-21.2 27.4-62.2 3.3-91.5-24-29.3-64.5-35.8-90.3-14.6" style="fill:#000;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M191.7 375.8c-25.9 21.3-27.4 62.2-3.4 91.5 24.1 29.3 64.6 35.8 90.4 14.6 25.9-21.2 27.4-62.2 3.3-91.5-24-29.3-64.5-35.8-90.3-14.6" style="fill:none;stroke:none" {
                    }
                    path d="M245.6 311.6c-41.6 27.7-99.1 49.2-123.8 86.2-18.4 27.5-39.3 38.3-17.3 65.5 21 25.7 32.7 53.3 61.9 57.9 32.8-4.4 58.3-7.3 79.3-43.8-2.4-21.4 4.8-66.6-23-88.1l-48.1-26.7" style="fill:none;stroke:#00f;stroke-width:21.56459045;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M175 400.5c-23.8 0-43.1 18.9-43.1 42.3s19.3 42.3 43.1 42.3 43.1-18.9 43.1-42.3-19.3-42.3-43.1-42.3" style="fill:#f36;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M175 400.5c-23.8 0-43.1 18.9-43.1 42.3s19.3 42.3 43.1 42.3 43.1-18.9 43.1-42.3-19.3-42.3-43.1-42.3" style="fill:none;stroke:none" {
                    }
                    path d="M194.2 418.9c-8.9-7.3-22.2-5.9-29.7 3.2s-6.3 22.4 2.7 29.7c8.9 7.3 22.2 5.9 29.6-3.2 7.5-9.1 6.3-22.4-2.6-29.7" style="fill:#00f;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M194.2 418.9c-8.9-7.3-22.2-5.9-29.7 3.2s-6.3 22.4 2.7 29.7c8.9 7.3 22.2 5.9 29.6-3.2 7.5-9.1 6.3-22.4-2.6-29.7" style="fill:none;stroke:none" {
                    }
                    path d="M249.3 308.8c-41.7 27.8-104.7 27.8-129.4 64.7-18.4 27.6-50.3 63.1-28.3 90.2 20.1 31.9 45.6 53.8 77.5 53.8 32.9-4.3 66.7-17 74.7-42.8-2.3-21.5 2.1-55.4-25.7-76.9l-45.3-36.1" style="fill:none;stroke:#00f;stroke-width:21.56459045;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M268 298c-2.9-5.1-15.3-3.4-27.6 3.7-12.4 7.1-20 17-17.1 22.1 3 5 15.3 3.4 27.7-3.7 12.3-7.2 20-17 17-22.1" style="fill:#00f;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M268 298c-2.9-5.1-15.3-3.4-27.6 3.7-12.4 7.1-20 17-17.1 22.1 3 5 15.3 3.4 27.7-3.7 12.3-7.2 20-17 17-22.1" style="fill:none;stroke:none" {
                    }
                    path d="M629.1 253c55.3-40.5 71-113.4 34.9-162.6-36.1-49.3-110.2-56.3-165.6-15.8-55.3 40.6-71 113.4-34.9 162.6 36.1 49.3 110.2 56.4 165.6 15.8" style="fill:#000;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M629.1 253c55.3-40.5 71-113.4 34.9-162.6-36.1-49.3-110.2-56.3-165.6-15.8-55.3 40.6-71 113.4-34.9 162.6 36.1 49.3 110.2 56.4 165.6 15.8z" style="fill:none;stroke:#000;stroke-width:4.30158997;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M399.5 162c-64 0-115.8 41.4-115.8 92.6 0 51.1 51.8 92.6 115.8 92.6s115.8-41.5 115.8-92.6c0-51.2-51.8-92.6-115.8-92.6" style="fill:#ff0;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M399.5 162c-64 0-115.8 41.4-115.8 92.6 0 51.1 51.8 92.6 115.8 92.6s115.8-41.5 115.8-92.6c0-51.2-51.8-92.6-115.8-92.6z" style="fill:none;stroke:#000;stroke-width:8.63150024;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M320.1 255.7c-25.6 0-46.3 20.8-46.3 46.3 0 25.6 20.7 46.3 46.3 46.3s46.4-20.7 46.4-46.3c0-25.5-20.8-46.3-46.4-46.3" style="fill:#fff;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M320.1 255.7c-25.6 0-46.3 20.8-46.3 46.3 0 25.6 20.7 46.3 46.3 46.3s46.4-20.7 46.4-46.3c0-25.5-20.8-46.3-46.4-46.3z" style="fill:none;stroke:#000;stroke-width:10.07479;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M683.4 296.3c-68.4-50.1-160-41.4-204.5 19.4-44.6 60.8-25.2 150.7 43.2 200.9 68.4 50.1 160 41.4 204.5-19.4 44.6-60.9 25.2-150.8-43.2-200.9" style="fill:#000;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M683.4 296.3c-68.4-50.1-160-41.4-204.5 19.4-44.6 60.8-25.2 150.7 43.2 200.9 68.4 50.1 160 41.4 204.5-19.4 44.6-60.9 25.2-150.8-43.2-200.9z" style="fill:none;stroke:#000;stroke-width:7.1882;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M698.5 335.2c-25.7-21.1-65.9-14.5-89.8 14.6s-22.5 69.8 3.2 90.9 65.9 14.6 89.8-14.6c23.9-29.1 22.5-69.8-3.2-90.9" style="fill:#0f0;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M698.5 335.2c-25.7-21.1-65.9-14.5-89.8 14.6s-22.5 69.8 3.2 90.9 65.9 14.6 89.8-14.6c23.9-29.1 22.5-69.8-3.2-90.9" style="fill:none;stroke:none" {
                    }
                    path d="M677.1 335.2c-25.7-21.1-65.9-14.5-89.8 14.6s-22.5 69.8 3.2 90.8c25.7 21.1 65.9 14.6 89.8-14.5s22.4-69.8-3.2-90.9" style="fill:#000;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M677.1 335.2c-25.7-21.1-65.9-14.5-89.8 14.6s-22.5 69.8 3.2 90.8c25.7 21.1 65.9 14.6 89.8-14.5s22.4-69.8-3.2-90.9" style="fill:none;stroke:none" {
                    }
                    path d="M705.7 464.7c-18 58.1-98.5 38.2-128.5 18.1-41.4-27.6-36.2-22.1-60.7-58.8-18.3-27.5-8.4-63.6 13.5-90.6 20.7-25.6 29-13.8 61.6-9.5s41.5 5.4 49.4 31c9.9 32.1-22.1 65.3-49.7 86.6L560.2 463" style="fill:none;stroke:#00f;stroke-width:21.56459045;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M569.2 345.3c-21 0-37.9 17.2-37.9 38.5s16.9 38.5 37.9 38.5c20.9 0 37.8-17.2 37.8-38.5s-16.9-38.5-37.8-38.5" style="fill:#f36;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M569.2 345.3c-21 0-37.9 17.2-37.9 38.5s16.9 38.5 37.9 38.5c20.9 0 37.8-17.2 37.8-38.5s-16.9-38.5-37.8-38.5" style="fill:none;stroke:none" {
                    }
                    path d="M582.5 361.7c-8.9-7.2-22.1-5.8-29.5 3.3-7.4 9-6.3 22.3 2.5 29.5 8.8 7.3 22.1 5.8 29.5-3.2 7.4-9.1 6.3-22.3-2.5-29.6" style="fill:#00f;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M582.5 361.7c-8.9-7.2-22.1-5.8-29.5 3.3-7.4 9-6.3 22.3 2.5 29.5 8.8 7.3 22.1 5.8 29.5-3.2 7.4-9.1 6.3-22.3-2.5-29.6" style="fill:none;stroke:none" {
                    }
                    path d="M711.2 466.5c-18 58.1-116.7 47.3-146.7 27.2-41.4-27.6-35.3-22.2-59.8-58.9-18.3-27.4-2-76.9 19.9-103.9 20.7-25.6 35.8-19.8 68.3-15.5 32.6 4.3 44 3.5 52 29.2 9.9 32 4 71.5-23.7 92.9l-61 25.5" style="fill:none;stroke:#00f;stroke-width:21.56459045;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M473.4 255.7c-25.6 0-46.4 20.8-46.4 46.3 0 25.6 20.8 46.3 46.4 46.3s46.3-20.7 46.3-46.3c0-25.5-20.7-46.3-46.3-46.3" style="fill:#fff;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M473.4 255.7c-25.6 0-46.4 20.8-46.4 46.3 0 25.6 20.8 46.3 46.4 46.3s46.3-20.7 46.3-46.3c0-25.5-20.7-46.3-46.3-46.3z" style="fill:none;stroke:#000;stroke-width:10.07479;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M321.5 279c-12.8 0-23.2 10.4-23.2 23.2s10.4 23.2 23.2 23.2 23.1-10.4 23.1-23.2-10.3-23.2-23.1-23.2" style="fill:#000;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M321.5 279c-12.8 0-23.2 10.4-23.2 23.2s10.4 23.2 23.2 23.2 23.1-10.4 23.1-23.2-10.3-23.2-23.1-23.2" style="fill:none;stroke:none" {
                    }
                    path d="M475.1 281.6c-12.9 0-23.2 10.4-23.2 23.1 0 12.8 10.3 23.1 23.2 23.1 12.8 0 23.1-10.3 23.1-23.1 0-12.7-10.3-23.1-23.1-23.1" style="fill:#000;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M475.1 281.6c-12.9 0-23.2 10.4-23.2 23.1 0 12.8 10.3 23.1 23.2 23.1 12.8 0 23.1-10.3 23.1-23.1 0-12.7-10.3-23.1-23.1-23.1" style="fill:none;stroke:none" {
                    }
                    path d="M360 230c-4.6-31.7 87.5-54.6 76.4-5.4v2.7" style="fill:none;stroke:#f36;stroke-width:12.93309021;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M385.1 348c-1.9 25.2 7.3 53.8-4.2 77.1l-9.7 23.1-12.6-21.2 9.7-1.9M413.3 348c1.9 25.2-7.4 53.8 4.1 77.1l9.7 23.1 12.6-21.2-9.7-1.9" style="fill:none;stroke:#000;stroke-width:10.07479;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M181.1 99.3c-26.6 21.9-27.5 64.8-2 96 25.6 31.1 67.9 38.6 94.6 16.7 26.6-21.9 27.5-64.8 1.9-95.9-25.5-31.2-67.8-38.7-94.5-16.8" style="fill:#f36;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M181.1 99.3c-26.6 21.9-27.5 64.8-2 96 25.6 31.1 67.9 38.6 94.6 16.7 26.6-21.9 27.5-64.8 1.9-95.9-25.5-31.2-67.8-38.7-94.5-16.8" style="fill:none;stroke:none" {
                    }
                    path d="M375.5 161v-29.9l16.4-27.2M412 161v-30l16.4-27.2" style="fill:none;stroke:#000;stroke-width:8.63150024;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:10;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    path d="M212.3 134.8c-12 9.8-10.1 32 4.3 49.5 14.4 17.6 35.7 23.8 47.8 13.9 11.9-9.8 10-32-4.3-49.6-14.4-17.5-35.8-23.7-47.8-13.8" style="fill:#00f;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M212.3 134.8c-12 9.8-10.1 32 4.3 49.5 14.4 17.6 35.7 23.8 47.8 13.9 11.9-9.8 10-32-4.3-49.6-14.4-17.5-35.8-23.7-47.8-13.8" style="fill:none;stroke:none" {
                    }
                    path d="M616.6 99.2c-26.7-21.9-69-14.4-94.6 16.8-25.5 31.1-24.6 74.1 2 95.9 26.7 21.9 69 14.4 94.6-16.7 25.5-31.2 24.6-74.1-2-96" style="fill:#f36;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M616.6 99.2c-26.7-21.9-69-14.4-94.6 16.8-25.5 31.1-24.6 74.1 2 95.9 26.7 21.9 69 14.4 94.6-16.7 25.5-31.2 24.6-74.1-2-96" style="fill:none;stroke:none" {
                    }
                    path d="M581.8 132c-12-9.8-33.4-3.6-47.7 13.8-14.4 17.5-16.3 39.7-4.2 49.5 11.9 9.9 33.3 3.7 47.7-13.8 14.3-17.4 16.2-39.6 4.2-49.5" style="fill:#00f;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M581.8 132c-12-9.8-33.4-3.6-47.7 13.8-14.4 17.5-16.3 39.7-4.2 49.5 11.9 9.9 33.3 3.7 47.7-13.8 14.3-17.4 16.2-39.6 4.2-49.5" style="fill:none;stroke:none" {
                    }
                    text transform="matrix(1 0 0 -1 658.4 245.8)" {
                        tspan x="0 9.6960001" y="0" style="font-size:16px;font-variant:normal;writing-mode:lr-tb;fill:gray;fill-opacity:1;fill-rule:nonzero;stroke:none;font-family:NimbusSanL;-inkscape-font-specification:NimbusSanL-Regu" {
                            "TM"
                        }
                    }
                    path d="M722.4 447.3c-6.1-4.2-19.3 4.3-29.6 19.1-10.4 14.7-13.8 30.1-7.8 34.3s19.3-4.3 29.6-19.1c10.3-14.7 13.8-30.1 7.8-34.3" style="fill:#00f;fill-opacity:1;fill-rule:evenodd;stroke:none" {
                    }
                    path d="M722.4 447.3c-6.1-4.2-19.3 4.3-29.6 19.1-10.4 14.7-13.8 30.1-7.8 34.3s19.3-4.3 29.6-19.1c10.3-14.7 13.8-30.1 7.8-34.3" style="fill:none;stroke:none" {
                    }
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!("raku {}", main_file.display()),
        }
    }
}
