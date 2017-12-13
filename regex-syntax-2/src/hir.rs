use std::error;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
}

impl error::Error for Error {
    fn description(&self) -> &str {
        ""
    }
}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        // use self::ErrorKind::*;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Hir {
    Empty,
    Literal(Vec<u8>),
    Class(CharClass),
    Anchor(Anchor),
    WordBoundary(WordBoundary),
    Group(Group),
    Repetition(Repetition),
    Concat(Vec<Hir>),
    Alternate(Vec<Hir>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Class {
    Unicode(ClassUnicode),
    Bytes(ClassBytes),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    pub ranges: Vec<ClassRangeUnicode>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassRangeUnicode {
    pub start: char,
    pub end: char,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassBytes {
    pub ranges: Vec<ClassRangeBytes>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassRangeBytes {
    pub start: u8,
    pub end: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Anchor {
    StartLine,
    EndLine,
    StartText,
    EndText,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WordBoundary {
    Unicode,
    NotUnicode,
    Ascii,
    NotAscii,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group {
    pub kind: GroupKind,
    pub hir: Box<Hir>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroupKind {
    CaptureIndex(u32),
    CaptureName(String),
    NonCapturing,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Repetition {
    pub kind: RepetitionKind,
    pub greedy: bool,
    pub hir: Box<Hir>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepetitionKind {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    Range(RepetitionRange),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepetitionRange {
    Exactly(u32),
    AtLeast(u32),
    Bounded(u32, u32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharClass {
    ranges: Vec<ClassRange>,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct ClassRange {
    pub start: char,
    pub end: char,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ByteClass {
    ranges: Vec<ByteRange>,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct ByteRange {
    pub start: u8,
    pub end: u8,
}
