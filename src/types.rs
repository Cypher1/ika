
type Layout = Vec<(Box<Type>, i32)>;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Type {
    Variable(String),
    TypeOr(Layout),
    TypeAnd(Layout),
    TypeImplication(Layout, Layout),
}

