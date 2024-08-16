use std::path::PathBuf;

pub fn filter_by_extension(files: Vec<PathBuf>, extension: &str) -> Vec<PathBuf> {
    files
        .into_iter()
        .filter(|file| file.extension().and_then(|s| s.to_str()) == Some(extension))
        .collect()
}

pub fn join_files(files: Vec<PathBuf>) -> String {
    files
        .iter()
        .map(|file| file.display().to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn titlecase_ascii(s: &str) -> String {
    if !s.is_ascii() || s.len() < 2 {
        s.to_string()
    } else {
        let (head, tail) = s.split_at(1);
        format!("{}{}", head.to_ascii_uppercase(), tail)
    }
}
