use std::cell::RefCell;
use std::result;

use ast::{self, Ast};
use hir::{self, Hir};

type Result<T> = result::Result<T, hir::Error>;

#[derive(Clone, Debug)]
pub struct TranslatorBuilder {
    _priv: (),
}

impl Default for TranslatorBuilder {
    fn default() -> TranslatorBuilder {
        TranslatorBuilder::new()
    }
}

impl TranslatorBuilder {
    pub fn new() -> TranslatorBuilder {
        TranslatorBuilder { _priv: () }
    }

    pub fn build<'a>(&self, ast: &'a Ast) -> Translator<'a> {
        Translator { stack: RefCell::new(vec![]) }
    }
}

#[derive(Clone, Debug)]
pub struct Translator<'a> {
    stack: RefCell<Vec<Case<'a>>>,
}

impl<'a> Translator<'a> {
    pub fn new(ast: &'a Ast) -> Translator<'a> {
        TranslatorBuilder::new().build(ast)
    }

    pub fn translate(&self, ast: &'a Ast) -> Result<Hir> {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
struct Frame<'a> {
    case: Case<'a>,
}

#[derive(Clone, Debug)]
enum Case<'a> {
    Group(&'a ast::Group),
    Concat(&'a [Ast]),
}
