mod image_tester;
mod language;
mod utils;

use language::Language;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let args: Vec<&str> = arguments.iter().map(String::as_str).collect();

    match &args[1..] {
        ["list"] => print_languages(),

        ["test", "hello", ids @ ..] => {
            let languages = ids_to_languages(ids);
            image_tester::test_images(image_tester::Test::HelloWorld, languages)
        }

        ["test", "version", ids @ ..] => {
            let languages = ids_to_languages(ids);
            image_tester::test_images(image_tester::Test::Version, languages)
        }

        _ => {
            eprintln!("Usage: {} list", args[0]);
            std::process::exit(1);
        }
    }
}

fn print_languages() {
    language::list().iter().for_each(|language| {
        println!("{}", language.config().id());
    });
}

fn ids_to_languages(language_ids: &[&str]) -> Vec<Language> {
    if language_ids.is_empty() {
        language::list()
    } else {
        language::list()
            .into_iter()
            .filter(|lang| language_ids.contains(&lang.config().id().as_str()))
            .collect()
    }
}
