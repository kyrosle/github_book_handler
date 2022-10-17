use super::*;
use anyhow::Result;
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
