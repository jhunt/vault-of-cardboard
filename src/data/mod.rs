pub mod cdif;
pub mod collection;
pub mod pile;
pub mod pool;
pub mod raw;

use std::io;
use std::fs::File;

pub trait Persistable : Sized {
    fn from_reader<T: io::Read>(src: &mut T) -> Result<Self, io::Error>;
    fn from_file(path: &str) -> Result<Self, io::Error> {
        Self::from_reader(&mut File::open(path)?)
    }

    fn from_stdin() -> Result<Self, io::Error> {
        Self::from_reader(&mut io::stdin().lock())
    }

    fn from_string(s: &str) -> Result<Self, io::Error> {
        Self::from_reader(&mut s.as_bytes())
    }
}
