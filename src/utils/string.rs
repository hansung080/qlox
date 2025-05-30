use std::ops::Range;
use std::string::FromUtf8Error;
use crate::utils::range::IntoRange;

#[derive(Debug)]
pub struct SubstringError {
    pub source: FromUtf8Error,
    pub range: Range<usize>,
}

pub trait Substring<Rng> {
    fn substring(&self, range: Rng) -> Result<String, SubstringError>;
    fn substring_lossy(&self, range: Rng) -> String;
}

impl<Rng: IntoRange<usize>> Substring<Rng> for str {
    fn substring(&self, range: Rng) -> Result<String, SubstringError> {
        Ok(self[range.into_range(0..self.len())].to_string())
    }

    fn substring_lossy(&self, range: Rng) -> String {
        self[range.into_range(0..self.len())].to_string()
    }
}

impl<Rng: IntoRange<usize>> Substring<Rng> for [u8] {
    fn substring(&self, range: Rng) -> Result<String, SubstringError> {
        let range = range.into_range(0..self.len());
        String::from_utf8(self[range.clone()].to_vec()).map_err(|source| {
            SubstringError {
                source,
                range,
            }
        })
    }

    fn substring_lossy(&self, range: Rng) -> String {
        String::from_utf8_lossy(&self[range.into_range(0..self.len())].to_vec()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substring() {
        let s: String = String::from("[Alice]");
        assert_eq!(s.substring(1..3).unwrap(), "Al");
        assert_eq!(s.substring(1..=3).unwrap(), "Ali");
        assert_eq!(s.substring(1..).unwrap(), "Alice]");
        assert_eq!(s.substring(..3).unwrap(), "[Al");
        assert_eq!(s.substring(..).unwrap(), "[Alice]");

        assert_eq!(s.substring_lossy(1..3), "Al");
        assert_eq!(s.substring_lossy(1..=3), "Ali");
        assert_eq!(s.substring_lossy(1..), "Alice]");
        assert_eq!(s.substring_lossy(..3), "[Al");
        assert_eq!(s.substring_lossy(..), "[Alice]");

        let s: &str = &s[1..s.len() - 1];
        assert_eq!(s.substring(..).unwrap(), "Alice");
        assert_eq!(s.substring_lossy(1..3), "li");

        let v: Vec<u8> = String::from("[Alice]").into_bytes();
        assert_eq!(v.substring(1..3).unwrap(), "Al");
        assert_eq!(v.substring(1..=3).unwrap(), "Ali");
        assert_eq!(v.substring(1..).unwrap(), "Alice]");
        assert_eq!(v.substring(..3).unwrap(), "[Al");
        assert_eq!(v.substring(..).unwrap(), "[Alice]");

        assert_eq!(v.substring_lossy(1..3), "Al");
        assert_eq!(v.substring_lossy(1..=3), "Ali");
        assert_eq!(v.substring_lossy(1..), "Alice]");
        assert_eq!(v.substring_lossy(..3), "[Al");
        assert_eq!(v.substring_lossy(..), "[Alice]");

        let v: &[u8] = &v[1..v.len() - 1];
        assert_eq!(v.substring(..).unwrap(), "Alice");
        assert_eq!(v.substring_lossy(1..3), "li");
    }
}