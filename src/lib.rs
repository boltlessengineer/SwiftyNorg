use std::{fs::File, io::Read, path::Path};

use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher, Utf32Str,
};
use url::Url;
use walkdir::WalkDir;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn get_length(term: String) -> String;

        fn read_file(url: String) -> String;

        fn search(path: &str, query: &str) -> Vec<String>;
    }
}

fn get_length(term: String) -> String {
    parse(term).unwrap_or(String::from("failed to parse"))
}

fn read_file(url: String) -> String {
    let path = Url::parse(&url).unwrap().to_file_path().unwrap();
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => return format!("Error opening file: {}", e),
    };

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => format!("File content length: {} bytes", buffer.len()),
        Err(e) => format!("Error reading file: {}", e),
    }
}

/// Searches for matching files from given file (`query`)
/// path is workspace path (uri)
pub fn safe_search(path: &str, query: &str) -> Result<Vec<String>, String> {
    if query.is_empty() {
        // TODO(boltless): should search return all contents or return Error?
        return Ok(Vec::new());
    }
    let uri = Url::parse(&path).unwrap().to_file_path().unwrap();
    let base_path = Path::new(&uri);
    if !base_path.is_dir() {
        return Err(format!("The path {path} is not a directory"));
    }
    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    let pattern = Pattern::parse(&query, CaseMatching::Ignore, Normalization::Smart);
    let mut matches: Vec<(u32, String)> = WalkDir::new(base_path)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if !entry.file_type().is_file() {
                return None;
            }
            let path = entry.path();
            let relative_path = path.strip_prefix(base_path).ok()?;
            let relative_str = relative_path.to_string_lossy();

            // `buf` is used to convert &str to Utf32Str (copied from pattern.match_list
            // implmentation)
            let mut buf = Vec::new();
            let score = pattern.score(Utf32Str::new(&relative_str, &mut buf), &mut matcher)?;
            let abs_path = path.canonicalize().ok()?;
            let uri = format!("file://{}", abs_path.display());
            Some((score, uri))
        })
        .collect();
    matches.sort_by(|a, b| b.0.cmp(&a.0));
    let results = matches.into_iter().map(|(_, uri)| uri).collect();
    return Ok(results);
}

pub fn search(path: &str, query: &str) -> Vec<String> {
    return safe_search(path, query).unwrap_or_else(|e| vec![e]);
}

fn parse(doc: String) -> Result<String, Box<dyn std::error::Error>> {
    let ast = rust_norg::parse_tree(&doc).or_else(|_| return Err("failed to parse tree"));
    let ast_json = serde_json::to_string(&ast)?;
    Ok(ast_json)
}
