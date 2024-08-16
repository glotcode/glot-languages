use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
let
    hello = "Hello World!";
in
hello
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nix;

impl LanguageConfig for Nix {
    fn id(&self) -> String {
        "nix".to_string()
    }

    fn name(&self) -> String {
        "Nix".to_string()
    }

    fn file_extension(&self) -> String {
        "nix".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/nix".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/nix:latest".to_string(),
            version_command: "nix --version".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" viewBox="0 0 501.704 435.14" {
                defs {
                    linearGradient id="nixGradientD" {
                        stop offset="0" style="stop-color:#699ad7;stop-opacity:1" {
                        }
                        stop offset=".243" style="stop-color:#7eb1dd;stop-opacity:1" {
                        }
                        stop offset="1" style="stop-color:#7ebae4;stop-opacity:1" {
                        }
                    }
                    linearGradient id="nixGradientC" {
                        stop offset="0" style="stop-color:#415e9a;stop-opacity:1" {
                        }
                        stop offset=".232" style="stop-color:#4a6baf;stop-opacity:1" {
                        }
                        stop offset="1" style="stop-color:#5277c3;stop-opacity:1" {
                        }
                    }
                    linearGradient id="nixGradientG" href="#nixGradientD" x1="200.597" x2="290.087" y1="351.411" y2="506.188" gradientTransform="translate(70.65 -1055.151)" gradientUnits="userSpaceOnUse" {
                    }
                    linearGradient id="nixGradientI" href="#nixGradientC" x1="-584.199" x2="-496.297" y1="782.336" y2="937.714" gradientTransform="translate(864.696 -1491.34)" gradientUnits="userSpaceOnUse" {
                    }
                }
                g style="display:inline" {
                    path d="M309.404-710.252 431.6-498.577l-56.157.527-32.623-56.87-32.857 56.566-27.902-.011-14.29-24.69 46.81-80.49-33.23-57.826z" style="color:#000;clip-rule:nonzero;display:inline;overflow:visible;visibility:visible;opacity:1;isolation:auto;mix-blend-mode:normal;color-interpolation:sRGB;color-interpolation-filters:linearRGB;solid-color:#000;solid-opacity:1;fill:#5277c3;fill-opacity:1;fill-rule:evenodd;stroke:none;stroke-width:3;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1;color-rendering:auto;image-rendering:auto;shape-rendering:auto;text-rendering:auto" transform="translate(-156.339 933.19)" {
                    }
                    path d="M353.51-797.443 231.291-585.78l-28.535-48.37 32.938-56.688-65.415-.172-13.941-24.17 14.236-24.72 93.112.293 33.464-57.69zM362.885-628.243l244.415.012-27.623 48.897-65.562-.182 32.56 56.737-13.962 24.159-28.527.032-46.301-80.784-66.693-.136zM505.143-720.989 382.946-932.664l56.157-.527 32.624 56.87 32.856-56.566 27.903.011 14.29 24.69-46.81 80.49 33.23 57.826z" style="color:#000;clip-rule:nonzero;display:inline;overflow:visible;visibility:visible;opacity:1;isolation:auto;mix-blend-mode:normal;color-interpolation:sRGB;color-interpolation-filters:linearRGB;solid-color:#000;solid-opacity:1;fill:#7ebae4;fill-opacity:1;fill-rule:evenodd;stroke:none;stroke-width:3;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1;color-rendering:auto;image-rendering:auto;shape-rendering:auto;text-rendering:auto" transform="translate(-156.339 933.19)" {
                    }
                    path d="M309.404-710.252 431.6-498.577l-56.157.527-32.623-56.87-32.857 56.566-27.902-.011-14.29-24.69 46.81-80.49-33.23-57.826zM451.336-803.533l-244.414-.012 27.622-48.896 65.562.181-32.558-56.737 13.96-24.158 28.528-.032 46.301 80.784 66.693.135zM460.872-633.842l122.217-211.664 28.535 48.37-32.938 56.688 65.415.172 13.941 24.17-14.236 24.72-93.112-.293-33.464 57.69z" style="color:#000;clip-rule:nonzero;display:inline;overflow:visible;visibility:visible;opacity:1;isolation:auto;mix-blend-mode:normal;color-interpolation:sRGB;color-interpolation-filters:linearRGB;solid-color:#000;solid-opacity:1;fill:#5277c3;fill-opacity:1;fill-rule:evenodd;stroke:none;stroke-width:3;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1;color-rendering:auto;image-rendering:auto;shape-rendering:auto;text-rendering:auto" transform="translate(-156.339 933.19)" {
                    }
                }
                g style="display:inline;opacity:1" transform="translate(-156.339 933.19)" {
                    path id="nixPathH" d="m309.549-710.388 122.197 211.675-56.157.527-32.624-56.87-32.856 56.566-27.903-.011-14.29-24.69 46.81-80.49-33.23-57.826z" style="opacity:1;fill:url(#nixGradientG);fill-opacity:1;fill-rule:evenodd;stroke:none;stroke-width:3;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1" {
                    }
                    use href="#nixPathH" width="100%" height="100%" transform="rotate(60 407.112 -715.787)" {
                    }
                    use href="#nixPathH" width="100%" height="100%" transform="rotate(-60 407.312 -715.7)" {
                    }
                    use href="#nixPathH" width="100%" height="100%" transform="rotate(180 407.419 -715.756)" {
                    }
                    path id="nixPathJ" d="m309.549-710.388 122.197 211.675-56.157.527-32.624-56.87-32.856 56.566-27.903-.011-14.29-24.69 46.81-80.49-33.23-57.826z" style="color:#000;clip-rule:nonzero;display:inline;overflow:visible;visibility:visible;opacity:1;isolation:auto;mix-blend-mode:normal;color-interpolation:sRGB;color-interpolation-filters:linearRGB;solid-color:#000;solid-opacity:1;fill:url(#nixGradientI);fill-opacity:1;fill-rule:evenodd;stroke:none;stroke-width:3;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1;color-rendering:auto;image-rendering:auto;shape-rendering:auto;text-rendering:auto" {
                    }
                    use href="#nixPathJ" width="100%" height="100%" style="display:inline" transform="rotate(120 407.34 -716.084)" {
                    }
                    use href="#nixPathJ" width="100%" height="100%" style="display:inline" transform="rotate(-120 407.288 -715.87)" {
                    }
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!("nix-instantiate --eval {}", main_file.display()),
        }
    }
}
