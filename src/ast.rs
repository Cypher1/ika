#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Apply {
    pub inner: Box<Node>,
    pub args: Vec<Let>,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Sym {
    pub name: String,
    pub info: Info,
}

impl ToNode Sym {
    to_node(self) {
        SymNode(self)
    }
    get_info(self) {
        self.info
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Prim {
    Unit(Info),
    Bool(bool, Info),
    I32(i32, Info),
    Str(String, Info),
}

impl ToNode Prim {
    to_node(self) {
        PrimNode(self)
    }
    get_info(self) {
        match self {
            Unit(info) => info,
            Bool(_, info) => info,
            I32(_, info) => info,
            Str(_, info) => info,
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Let {
    pub name: String,
    pub value: Option<Box<Node>>,
    pub info: Info,
}

impl ToNode Let {
    to_node(self) {
        LetNode(self)
    }
    get_info(self) {
        self.info
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct UnOp {
    pub name: String,
    pub inner: Box<Node>,
    pub info: Info,
}

impl ToNode UnOp {
    to_node(self) {
        UnOpNode(self)
    }
    get_info(self) {
        self.info
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct BinOp {
    pub name: String,
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub info: Info,
}

impl ToNode BinOp {
    to_node(self) {
        BinOpNode(self)
    }
    get_info(self) {
        self.info
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Loc {
    filename: Option<String>;
    line: i32;
    col: i32;
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Info {
    loc: Option<Loc>;
}

impl Default Info {
    fn default() -> Info {
        Info {loc: None}
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Node {
    Error(String),
    SymNode(Sym),
    PrimNode(Prim),
    ApplyNode(Apply),
    LetNode(Let),
    UnOpNode(UnOp),
    BinOpNode(BinOp),
}

pub trait ToNode {
    fn to_node(self: Self) -> Node;
    fn get_info(self: Self) -> Info;
}

pub fn get_loc<T: ToNode>(n: T) -> Option<Loc> {
    n.get_info().loc
}

pub trait Visitor<State, Res, Final, Err> {
    fn visit_root(&mut self, e: &Node) -> Result<Final, Err>;

    fn handle_error(&mut self, state: &mut State, e: &String) -> Result<Res, Err>;
    fn visit_sym(&mut self, state: &mut State, e: &Sym) -> Result<Res, Err>;
    fn visit_prim(&mut self, e: &Prim) -> Result<Res, Err>;
    fn visit_apply(&mut self, state: &mut State, e: &Apply) -> Result<Res, Err>;
    fn visit_let(&mut self, state: &mut State, e: &Let) -> Result<Res, Err>;
    fn visit_un_op(&mut self, state: &mut State, e: &UnOp) -> Result<Res, Err>;
    fn visit_bin_op(&mut self, state: &mut State, e: &BinOp) -> Result<Res, Err>;

    fn visit(&mut self, state: &mut State, e: &Node) -> Result<Res, Err> {
        // println!("{:?}", e);
        use Node::*;
        match e {
            Error(n) => self.handle_error(state, n),
            SymNode(n) => self.visit_sym(state, n),
            PrimNode(n) => self.visit_prim(n),
            ApplyNode(n) => self.visit_apply(state, n),
            LetNode(n) => self.visit_let(state, n),
            UnOpNode(n) => self.visit_un_op(state, n),
            BinOpNode(n) => self.visit_bin_op(state, n),
        }
    }
}
