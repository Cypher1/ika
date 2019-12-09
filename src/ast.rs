use super::tokens::*;
use super::tree::Tree;

pub trait Visitor<T, U, V> {
    fn visit(&mut self, e: &T) -> U;
    fn visit_root(&mut self, e: &T) -> V;
}
