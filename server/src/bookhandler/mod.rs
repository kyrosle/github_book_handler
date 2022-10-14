use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use git2::Repository;

pub fn download_github(book_name: &String, url: &str) -> Result<()> {
    if check_book_exists(book_name) {
        Ok(())
    } else {
        let mut dir_path = get_book_resources_path();
        dir_path.push(book_name);
        {
            let dir_path = dir_path.clone();
            let dir_path = dir_path.into_os_string();
            let _repo = match Repository::clone(url, &dir_path) {
                Ok(repo) => repo,
                Err(e) => {
                    return Err(anyhow!("{}", e));
                }
            };
        }
        Ok(())
    }
}

/// 
/// from `https://github.com/kyrosle/inside-std-rust` take out `inside-std-rust`
#[allow(unused)]
pub fn get_name_from_git_url(url: &str) -> Result<String> {
    let book_name = url
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .collect::<Vec<&str>>();
    if !book_name.is_empty() {
        Ok(book_name[0].into())
    } else {
        Err(anyhow!("can't find book_name in url!"))
    }
}

/// return a PathBuf which is the project root dir
#[allow(unused)]
pub fn get_book_resources_path() -> PathBuf {
    let mut dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let put_root = Path::new(r"resources");
    let put_look = Path::new(r"books");
    dir_path.push(put_root);
    dir_path.push(put_look);
    dir_path
}

/// check path whether been git cloned
#[allow(unused)]
pub fn check_book_exists(book_name: &String) -> bool {
    let mut dir_path = get_book_resources_path();
    dir_path.push(book_name);
    dir_path.as_path().exists()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_name_from_git_url_should_ok() {
        assert_eq!(
            get_name_from_git_url("https://github.com/kyrosle/inside-std-rust").unwrap(),
            "inside-std-rust"
        );
    }
}
