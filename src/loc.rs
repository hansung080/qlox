use std::ops::Range;

pub type SrcIdx = usize;
pub type SrcRng = Range<SrcIdx>;

#[derive(Debug)]
pub struct SrcPos {
    pub line: SrcIdx,
    pub column: SrcIdx,
}

#[derive(Debug)]
pub enum SrcLoc {
    Created {
        offset: SrcIdx,
    },
    Resolved {
        offset: SrcPos,
        line: SrcRng,
    },
}

impl SrcLoc {
    pub fn resolve(&mut self, source: &[u8]) {
        todo!()
    }
}

pub trait ResolveLoc {
    fn resolve_loc(&mut self, source: &[u8]);
}

impl<T: ResolveLoc> ResolveLoc for Vec<T> {
    fn resolve_loc(&mut self, source: &[u8]) {
        for e in self {
            e.resolve_loc(source);
        }
    }
}