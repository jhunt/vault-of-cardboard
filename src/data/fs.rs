use serde::Serialize;
use serde_json::json;
use std::io;
use std::fs::{self, File};
use std::io::{Read, Write, Seek};
use std::path::{Path, PathBuf};

mod errors {
    error_chain! {}
}
pub use errors::Error;
use errors::*;

pub struct Store {
    root: PathBuf,
}

impl Store {
    pub fn new(root: &Path) -> Self {
        Self {
            root: PathBuf::from(root),
        }
    }

    fn path_to(&self, filename: &str) -> PathBuf {
        let mut pb = self.root.clone();
        pb.push(filename);
        pb
    }

    pub fn create(&self, filename: &str, contents: &str) -> Result<()> {
        let pb = self.path_to(filename);

        fs::create_dir_all(
            pb.parent()
                .chain_err(|| "failed to construct path for file storage")?,
        )
        .chain_err(|| "failed to create parent directories for file storage")?;

        let mut file = File::create(
            pb.to_str()
                .chain_err(|| "failed to construct a path for file storage")?,
        )
        .chain_err(|| "failed to create file")?;
        write!(file, "{}", contents).chain_err(|| "failed to write to file")?;

        Ok(())
    }

    pub fn get_as_reader(&self, filename: &str) -> Result<File> {
        let pb = self.path_to(filename);
        Ok(File::open(
            pb.to_str()
                .chain_err(|| "failed to construct a path for file retrieval")?,
        )
        .chain_err(|| "failed to open file")?)
    }

    pub fn get_as_string(&self, filename: &str) -> Result<String> {
        let mut file = self.get_as_reader(filename)?;

        let mut s = String::new();
        file.read_to_string(&mut s)
            .chain_err(|| "failed to read from file")?;
        Ok(s)
    }

    pub fn append_to_json_list<T: Serialize>(&self, filename: &str, item: T) -> Result<()> {
        let pb = self.path_to(filename);
        let mut file = fs::OpenOptions::new().read(true).write(true).open(
            pb.to_str()
                .chain_err(|| "failed to construct a path for file modifucation")?,
        )
        .chain_err(|| "failed to open file")?;

        file.seek(io::SeekFrom::End(-2))
            .chain_err(|| "failed to seek to end of json list file")?;

        write!(file, ",{}]]", json!(item)).chain_err(|| "failed to append to json list file")?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn should_be_able_to_create_files() {
        let fs = TempDir::new("data-fs-test").expect("failed to create tempdir");
        let store = Store::new(fs.path());

        let s = store.get_as_string("test");
        assert!(s.is_err());

        assert!(store.create("test", r#"[[""]]"#).is_ok());
        let s = store.get_as_string("test");
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, r#"[[""]]"#);
    }

    #[test]
    fn should_create_intervening_parent_directories() {
        let fs = TempDir::new("data-fs-test").expect("failed to create tempdir");
        let store = Store::new(fs.path());

        let key = "a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z.json";
        assert!(store.create(key, "FOO").is_ok());
        let s = store.get_as_string(key);
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, "FOO");
    }

    #[test]
    fn should_be_able_to_append_new_json() {
        let fs = TempDir::new("data-fs-test").expect("failed to create tempdir");
        let store = Store::new(fs.path());

        assert!(store.create("something.json", r#"[["initial"],[[]]]"#).is_ok());
        assert!(store.append_to_json_list("something.json", vec!["new", "things"]).is_ok());
        let s = store.get_as_string("something.json");
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, r#"[["initial"],[[],["new","things"]]]"#);
    }
}
