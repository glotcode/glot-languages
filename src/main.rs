pub mod language;
pub mod utils;

fn main() {
    language::list().iter().for_each(|language| {
        println!("{}", language.config().id());
    });
}
