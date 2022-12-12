use std::collections::HashMap;
use std::fs::{self};

fn add_file_size<'a>(files: &'a mut HashMap<&str, u64>, dirpath: &mut Vec<&str>, size: u64) -> &'a HashMap<&'static str, u64> {
    let path_str = dirpath.join("/");

    if !files.contains_key(&path_str as &str) {
        files[&path_str as &str] = 0;
    }

    files[&path_str as &str] += size;
    if dirpath.len() == 0 {
        files
    }
    else {
        dirpath.pop();
        add_file_size(files, dirpath, size)
    }
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap().to_string()
}

pub fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_file_size() {
        let mut files: &HashMap<&str, u64> = &HashMap::new();
        let answers = HashMap::from([
            ("", 123 + 321 + 217),
            ("a", 123 + 321),
            ("b", 217),
            ("a/d", 123),
            ("a/e", 321),
            ("b/c", 217)
        ]);

        files = add_file_size(&mut files, &mut Vec::from(["a","d"]), 123);
        files = add_file_size(&mut files, &mut Vec::from(["a","e"]), 321);
        files = add_file_size(&mut files, &mut Vec::from(["b","c"]), 217);

        assert_eq!(files, answers);
    }
}