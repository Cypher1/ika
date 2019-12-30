#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Apply {
    pub inner: Box<Node>,
    pub args: Vec<Let>,
    pub info: Info,
}

impl ToNode for Apply {
    fn to_node(self) -> Node {
        Node::ApplyNode(self)
    }
    fn get_info(self) -> Info {
        self.info
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Sym {
    pub name: String,
    pub info: Info,
}

impl ToNode for Sym {
    fn to_node(self) -> Node {
        Node::SymNode(self)
    }
    fn get_info(self) -> Info {
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

impl ToNode for Prim {
    fn to_node(self) -> Node {
        Node::PrimNode(self)
    }
    fn get_info(self) -> Info {
        use Prim::*;
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

impl ToNode for Let {
    fn to_node(self) -> Node {
        Node::LetNode(self)
    }
    fn get_info(self) -> Info {
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

impl ToNode for UnOp {
    fn to_node(self) -> Node {
        Node::UnOpNode(self)
    }
    fn get_info(self) -> Info {
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

impl ToNode for BinOp {
    fn to_node(self) -> Node {
        Node::BinOpNode(self)
    }
    fn get_info(self) -> Info {
        self.info
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Loc {
    filename: Option<String>,
    line: i32,
    col: i32,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Info {
    pub loc: Option<Loc>,
}

impl Default for Info {
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

impl ToNode for Node {
    fn to_node(self) -> Node {
        self
    }
    fn get_info(self) -> Info {
        use Node::*;
        match self {
            Error(_) => Info::default(),
            SymNode(n) => n.get_info(),
            PrimNode(n) => n.get_info(),
            ApplyNode(n) => n.get_info(),
            LetNode(n) => n.get_info(),
            UnOpNode(n) => n.get_info(),
            BinOpNode(n) => n.get_info(),
        }
    }
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
