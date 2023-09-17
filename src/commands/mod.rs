use chrono::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub fn make_today_path(p: &String) -> PathBuf {
    let now: DateTime<Local> = Local::now();

    let path = Path::new(p)
        .join(now.year().to_string())
        .join(now.month().to_string())
        .join(now.day().to_string());

    return path;
}

pub fn make_today_dir(p: &String) -> std::io::Result<()> {
    let today_path = make_today_path(p);
    fs::create_dir_all(&today_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_dir() {
        let str_path = String::from(".");
        let path: PathBuf = make_today_path(&str_path);
        println!("{:?}", path)
    }

    #[test]
    fn make_dir() {
        let str_path = String::from(".");
        let _ = make_today_dir(&str_path);
        assert_eq!(Path::new(s))
    }
}
