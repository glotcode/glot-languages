pub mod assembly;
pub mod ats;
pub mod bash;
pub mod c;
pub mod clisp;
pub mod clojure;
pub mod cobol;
pub mod coffeescript;
pub mod cpp;
pub mod crystal;
pub mod csharp;
pub mod d;
pub mod dart;
pub mod elixir;
pub mod elm;
pub mod erlang;
pub mod fsharp;
pub mod go;
pub mod groovy;
pub mod guile;
pub mod hare;
pub mod haskell;
pub mod idris;
pub mod java;
pub mod javascript;
pub mod julia;
pub mod kotlin;
pub mod lua;
pub mod mercury;
pub mod nim;
pub mod nix;
pub mod ocaml;
pub mod pascal;
pub mod perl;
pub mod php;
pub mod python;
pub mod raku;
pub mod ruby;
pub mod rust;
pub mod sac;
pub mod scala;
pub mod swift;
pub mod typescript;
pub mod zig;

use maud::Markup;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Assembly,
    Ats,
    Bash,
    C,
    Clisp,
    Clojure,
    Cobol,
    CoffeeScript,
    Cpp,
    Crystal,
    Csharp,
    D,
    Dart,
    Elixir,
    Elm,
    Erlang,
    Fsharp,
    Go,
    Groovy,
    Guile,
    Hare,
    Haskell,
    Idris,
    Java,
    JavaScript,
    Julia,
    Kotlin,
    Lua,
    Mercury,
    Nim,
    Nix,
    Ocaml,
    Pascal,
    Perl,
    Php,
    Python,
    Raku,
    Ruby,
    Rust,
    Sac,
    Scala,
    Swift,
    TypeScript,
    Zig,
}

impl Language {
    pub fn config(&self) -> Box<dyn LanguageConfig> {
        match self {
            Self::Assembly => Box::new(assembly::Assembly),
            Self::Ats => Box::new(ats::Ats),
            Self::Bash => Box::new(bash::Bash),
            Self::C => Box::new(c::C),
            Self::Clisp => Box::new(clisp::Clisp),
            Self::Clojure => Box::new(clojure::Clojure),
            Self::Cobol => Box::new(cobol::Cobol),
            Self::CoffeeScript => Box::new(coffeescript::CoffeeScript),
            Self::Cpp => Box::new(cpp::Cpp),
            Self::Crystal => Box::new(crystal::Crystal),
            Self::Csharp => Box::new(csharp::Csharp),
            Self::D => Box::new(d::D),
            Self::Dart => Box::new(dart::Dart),
            Self::Elixir => Box::new(elixir::Elixir),
            Self::Elm => Box::new(elm::Elm),
            Self::Erlang => Box::new(erlang::Erlang),
            Self::Fsharp => Box::new(fsharp::Fsharp),
            Self::Go => Box::new(go::Go),
            Self::Groovy => Box::new(groovy::Groovy),
            Self::Guile => Box::new(guile::Guile),
            Self::Hare => Box::new(hare::Hare),
            Self::Haskell => Box::new(haskell::Haskell),
            Self::Idris => Box::new(idris::Idris),
            Self::Java => Box::new(java::Java),
            Self::JavaScript => Box::new(javascript::JavaScript),
            Self::Julia => Box::new(julia::Julia),
            Self::Kotlin => Box::new(kotlin::Kotlin),
            Self::Lua => Box::new(lua::Lua),
            Self::Mercury => Box::new(mercury::Mercury),
            Self::Nim => Box::new(nim::Nim),
            Self::Nix => Box::new(nix::Nix),
            Self::Ocaml => Box::new(ocaml::Ocaml),
            Self::Pascal => Box::new(pascal::Pascal),
            Self::Perl => Box::new(perl::Perl),
            Self::Php => Box::new(php::Php),
            Self::Python => Box::new(python::Python),
            Self::Raku => Box::new(raku::Raku),
            Self::Ruby => Box::new(ruby::Ruby),
            Self::Rust => Box::new(rust::Rust),
            Self::Sac => Box::new(sac::Sac),
            Self::Scala => Box::new(scala::Scala),
            Self::Swift => Box::new(swift::Swift),
            Self::TypeScript => Box::new(typescript::TypeScript),
            Self::Zig => Box::new(zig::Zig),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ParseIdError;

impl FromStr for Language {
    type Err = ParseIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        list()
            .into_iter()
            .find(|language| s == language.config().id())
            .ok_or(ParseIdError)
    }
}

pub fn list() -> Vec<Language> {
    vec![
        Language::Assembly,
        Language::Ats,
        Language::Bash,
        Language::C,
        Language::Clisp,
        Language::Clojure,
        Language::Cobol,
        Language::CoffeeScript,
        Language::Cpp,
        Language::Crystal,
        Language::Csharp,
        Language::D,
        Language::Dart,
        Language::Elixir,
        Language::Elm,
        Language::Erlang,
        Language::Fsharp,
        Language::Go,
        Language::Groovy,
        Language::Guile,
        Language::Hare,
        Language::Haskell,
        Language::Idris,
        Language::Java,
        Language::JavaScript,
        Language::Julia,
        Language::Kotlin,
        Language::Lua,
        Language::Mercury,
        Language::Nim,
        Language::Nix,
        Language::Ocaml,
        Language::Pascal,
        Language::Perl,
        Language::Php,
        Language::Python,
        Language::Raku,
        Language::Ruby,
        Language::Rust,
        Language::Sac,
        Language::Scala,
        Language::Swift,
        Language::TypeScript,
        Language::Zig,
    ]
}

pub trait LanguageConfig {
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn file_extension(&self) -> String;
    fn editor_config(&self) -> EditorConfig;
    fn run_config(&self) -> RunConfig;
    fn logo(&self) -> Markup;
    fn run_instructions(&self, main_file: PathBuf, other_files: Vec<PathBuf>) -> RunInstructions;
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorConfig {
    pub default_filename: String,
    pub mode: String,
    pub use_soft_tabs: bool,
    pub soft_tab_size: u8,
    pub example_code: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunConfig {
    pub container_image: String,
    pub version_command: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunInstructions {
    pub build_commands: Vec<String>,
    pub run_command: String,
}
