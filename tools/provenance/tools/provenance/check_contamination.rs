use std::path::Path;

pub fn check_contamination(path: &str, altitude: &str) -> bool {
    match altitude {
        "A1" => Path::new(path).starts_with("docs"),
        "A5" => Path::new(path).starts_with("provenance"),
        _ => false
    }
}
