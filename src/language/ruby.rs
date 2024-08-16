use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
puts "Hello World!"
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ruby;

impl LanguageConfig for Ruby {
    fn id(&self) -> String {
        "ruby".to_string()
    }

    fn name(&self) -> String {
        "Ruby".to_string()
    }

    fn file_extension(&self) -> String {
        "rb".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/ruby".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/ruby:latest".to_string(),
            version_command: "ruby --version".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 255" preserveAspectRatio="xMidYMid" {
                defs {
                    linearGradient id="rubyGradient1" x1="84.75%" x2="58.254%" y1="111.399%" y2="64.584%" {
                        stop offset="0%" stop-color="#FB7655" {
                        }
                        stop offset="0%" stop-color="#FB7655" {
                        }
                        stop offset="41%" stop-color="#E42B1E" {
                        }
                        stop offset="99%" stop-color="#900" {
                        }
                        stop offset="100%" stop-color="#900" {
                        }
                    }
                    linearGradient id="rubyGradient2" x1="116.651%" x2="1.746%" y1="60.89%" y2="19.288%" {
                        stop offset="0%" stop-color="#871101" {
                        }
                        stop offset="0%" stop-color="#871101" {
                        }
                        stop offset="99%" stop-color="#911209" {
                        }
                        stop offset="100%" stop-color="#911209" {
                        }
                    }
                    linearGradient id="rubyGradient3" x1="75.774%" x2="38.978%" y1="219.327%" y2="7.829%" {
                        stop offset="0%" stop-color="#871101" {
                        }
                        stop offset="0%" stop-color="#871101" {
                        }
                        stop offset="99%" stop-color="#911209" {
                        }
                        stop offset="100%" stop-color="#911209" {
                        }
                    }
                    linearGradient id="rubyGradient4" x1="50.012%" x2="66.483%" y1="7.234%" y2="79.135%" {
                        stop offset="0%" stop-color="#FFF" {
                        }
                        stop offset="0%" stop-color="#FFF" {
                        }
                        stop offset="23%" stop-color="#E57252" {
                        }
                        stop offset="46%" stop-color="#DE3B20" {
                        }
                        stop offset="99%" stop-color="#A60003" {
                        }
                        stop offset="100%" stop-color="#A60003" {
                        }
                    }
                    linearGradient id="rubyGradient5" x1="46.174%" x2="49.932%" y1="16.348%" y2="83.047%" {
                        stop offset="0%" stop-color="#FFF" {
                        }
                        stop offset="0%" stop-color="#FFF" {
                        }
                        stop offset="23%" stop-color="#E4714E" {
                        }
                        stop offset="56%" stop-color="#BE1A0D" {
                        }
                        stop offset="99%" stop-color="#A80D00" {
                        }
                        stop offset="100%" stop-color="#A80D00" {
                        }
                    }
                    linearGradient id="rubyGradient6" x1="36.965%" x2="49.528%" y1="15.594%" y2="92.478%" {
                        stop offset="0%" stop-color="#FFF" {
                        }
                        stop offset="0%" stop-color="#FFF" {
                        }
                        stop offset="18%" stop-color="#E46342" {
                        }
                        stop offset="40%" stop-color="#C82410" {
                        }
                        stop offset="99%" stop-color="#A80D00" {
                        }
                        stop offset="100%" stop-color="#A80D00" {
                        }
                    }
                    linearGradient id="rubyGradient7" x1="13.609%" x2="85.764%" y1="58.346%" y2="-46.717%" {
                        stop offset="0%" stop-color="#FFF" {
                        }
                        stop offset="0%" stop-color="#FFF" {
                        }
                        stop offset="54%" stop-color="#C81F11" {
                        }
                        stop offset="99%" stop-color="#BF0905" {
                        }
                        stop offset="100%" stop-color="#BF0905" {
                        }
                    }
                    linearGradient id="rubyGradient8" x1="27.624%" x2="50.745%" y1="21.135%" y2="79.056%" {
                        stop offset="0%" stop-color="#FFF" {
                        }
                        stop offset="0%" stop-color="#FFF" {
                        }
                        stop offset="31%" stop-color="#DE4024" {
                        }
                        stop offset="99%" stop-color="#BF190B" {
                        }
                        stop offset="100%" stop-color="#BF190B" {
                        }
                    }
                    linearGradient id="rubyGradient9" x1="-20.667%" x2="104.242%" y1="122.282%" y2="-6.342%" {
                        stop offset="0%" stop-color="#BD0012" {
                        }
                        stop offset="0%" stop-color="#BD0012" {
                        }
                        stop offset="7%" stop-color="#FFF" {
                        }
                        stop offset="17%" stop-color="#FFF" {
                        }
                        stop offset="27%" stop-color="#C82F1C" {
                        }
                        stop offset="33%" stop-color="#820C01" {
                        }
                        stop offset="46%" stop-color="#A31601" {
                        }
                        stop offset="72%" stop-color="#B31301" {
                        }
                        stop offset="99%" stop-color="#E82609" {
                        }
                        stop offset="100%" stop-color="#E82609" {
                        }
                    }
                    linearGradient id="rubyGradient10" x1="58.792%" x2="11.964%" y1="65.205%" y2="50.128%" {
                        stop offset="0%" stop-color="#8C0C01" {
                        }
                        stop offset="0%" stop-color="#8C0C01" {
                        }
                        stop offset="54%" stop-color="#990C00" {
                        }
                        stop offset="99%" stop-color="#A80D0E" {
                        }
                        stop offset="100%" stop-color="#A80D0E" {
                        }
                    }
                    linearGradient id="rubyGradient11" x1="79.319%" x2="23.088%" y1="62.754%" y2="17.888%" {
                        stop offset="0%" stop-color="#7E110B" {
                        }
                        stop offset="0%" stop-color="#7E110B" {
                        }
                        stop offset="99%" stop-color="#9E0C00" {
                        }
                        stop offset="100%" stop-color="#9E0C00" {
                        }
                    }
                    linearGradient id="rubyGradient12" x1="92.88%" x2="59.841%" y1="74.122%" y2="39.704%" {
                        stop offset="0%" stop-color="#79130D" {
                        }
                        stop offset="0%" stop-color="#79130D" {
                        }
                        stop offset="99%" stop-color="#9E120B" {
                        }
                        stop offset="100%" stop-color="#9E120B" {
                        }
                    }
                    linearGradient id="rubyGradient15" x1="56.57%" x2="3.105%" y1="101.717%" y2="11.993%" {
                        stop offset="0%" stop-color="#8B2114" {
                        }
                        stop offset="0%" stop-color="#8B2114" {
                        }
                        stop offset="43%" stop-color="#9E100A" {
                        }
                        stop offset="99%" stop-color="#B3100C" {
                        }
                        stop offset="100%" stop-color="#B3100C" {
                        }
                    }
                    linearGradient id="rubyGradient16" x1="30.87%" x2="92.471%" y1="35.599%" y2="100.694%" {
                        stop offset="0%" stop-color="#B31000" {
                        }
                        stop offset="0%" stop-color="#B31000" {
                        }
                        stop offset="44%" stop-color="#910F08" {
                        }
                        stop offset="99%" stop-color="#791C12" {
                        }
                        stop offset="100%" stop-color="#791C12" {
                        }
                    }
                    radialGradient id="rubyGradient13" cx="32.001%" cy="40.21%" r="69.573%" fx="32.001%" fy="40.21%" {
                        stop offset="0%" stop-color="#A80D00" {
                        }
                        stop offset="0%" stop-color="#A80D00" {
                        }
                        stop offset="99%" stop-color="#7E0E08" {
                        }
                        stop offset="100%" stop-color="#7E0E08" {
                        }
                    }
                    radialGradient id="rubyGradient14" cx="13.549%" cy="40.86%" r="88.386%" fx="13.549%" fy="40.86%" {
                        stop offset="0%" stop-color="#A30C00" {
                        }
                        stop offset="0%" stop-color="#A30C00" {
                        }
                        stop offset="99%" stop-color="#800E08" {
                        }
                        stop offset="100%" stop-color="#800E08" {
                        }
                    }
                }
                path fill="url(#rubyGradient1)" d="m197.467 167.764-145.52 86.41 188.422-12.787L254.88 51.393z" {
                }
                path fill="url(#rubyGradient2)" d="M240.677 241.257 224.482 129.48l-44.113 58.25z" {
                }
                path fill="url(#rubyGradient3)" d="m240.896 241.257-118.646-9.313-69.674 21.986z" {
                }
                path fill="url(#rubyGradient4)" d="m52.744 253.955 29.64-97.1L17.16 170.8z" {
                }
                path fill="url(#rubyGradient5)" d="M180.358 188.05 153.085 81.226l-78.047 73.16z" {
                }
                path fill="url(#rubyGradient6)" d="m248.693 82.73-73.777-60.256-20.544 66.418z" {
                }
                path fill="url(#rubyGradient7)" d="M214.191.99 170.8 24.97 143.424.669z" {
                }
                path fill="url(#rubyGradient8)" d="m0 203.372 18.177-33.151-14.704-39.494z" {
                }
                path fill="#FFF" d="m2.496 129.48 14.794 41.963 64.283-14.422 73.39-68.207 20.712-65.787L143.063 0 87.618 20.75c-17.469 16.248-51.366 48.396-52.588 49-1.21.618-22.384 40.639-32.534 59.73" {
                }
                path fill="url(#rubyGradient9)" d="M54.442 54.094c37.86-37.538 86.667-59.716 105.397-40.818 18.72 18.898-1.132 64.823-38.992 102.349-37.86 37.525-86.062 60.925-104.78 42.027-18.73-18.885.515-66.032 38.375-103.558" {
                }
                path fill="url(#rubyGradient10)" d="m52.744 253.916 29.408-97.409 97.665 31.376c-35.312 33.113-74.587 61.106-127.073 66.033" {
                }
                path fill="url(#rubyGradient11)" d="m155.092 88.622 25.073 99.313c29.498-31.016 55.972-64.36 68.938-105.603z" {
                }
                path fill="url(#rubyGradient12)" d="M248.847 82.833c10.035-30.282 12.35-73.725-34.966-81.791l-38.825 21.445z" {
                }
                path fill="#9E1209" d="M0 202.935c1.39 49.979 37.448 50.724 52.808 51.162l-35.48-82.86z" {
                }
                path fill="url(#rubyGradient13)" d="M155.232 88.777c22.667 13.932 68.35 41.912 69.276 42.426 1.44.81 19.695-30.784 23.838-48.64z" {
                }
                path fill="url(#rubyGradient14)" d="m82.113 156.507 39.313 75.848c23.246-12.607 41.45-27.967 58.121-44.42z" {
                }
                path fill="url(#rubyGradient15)" d="m17.174 171.34-5.57 66.328c10.51 14.357 24.97 15.605 40.136 14.486-10.973-27.311-32.894-81.92-34.566-80.814" {
                }
                path fill="url(#rubyGradient16)" d="m174.826 22.654 78.1 10.96c-4.169-17.662-16.969-29.06-38.787-32.623z" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!("ruby {}", main_file.display()),
        }
    }
}
