mod image_tester;
mod language;
mod utils;

use std::env;

fn main() {
    let mut args = env::args();
    let app_name = args.next().unwrap_or_default();

    match args.next().unwrap_or_default().as_str() {
        "list" => list_languages(),

        "test-images" => image_tester::test_images(),

        _ => {
            eprintln!("Usage: {} list", app_name);
            std::process::exit(1);
        }
    }
}

fn list_languages() {
    language::list().iter().for_each(|language| {
        println!("{}", language.config().id());
    });
}
