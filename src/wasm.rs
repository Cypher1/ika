use super::tree::*;

/*
(module
  (func (export "addTwo") (param i32 i32) (result i32)
    local.get 0                                             local.get 1
    i32.const 3
    i32.mul
    i32.add))

        //TokenType::Local => {
            // return vec!["locals.get ".to_string() + &expr.value.value];
        // }
*/

use super::ast::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum CompilerError {
    UnknownInfixOperator(String, Info),
    UnknownPrefixOperator(String, Info),
    FailedParse(String, Info),
}

// Walks the AST compiling it to wasm.
pub struct Compiler;

impl Default for Compiler {
    fn default() -> Compiler {
        Compiler {}
    }
}

type Res = Result<Vec<String>, CompilerError>;
type State = ();
impl Visitor<State, Vec<String>, Tree<String>, CompilerError> for Compiler {
    fn visit_root(&mut self, expr: &Node) -> Result<Tree<String>, CompilerError> {
        let mut state = ();
        let name = Tree {
            value: "\"run_main\"".to_string(),
            children: vec![],
        };
        let def = Tree {
            value: "export".to_string(),
            children: vec![name.clone()],
        };
        let node_i32 = Tree {
            value: "i32".to_string(),
            children: vec![],
        };
        let param = Tree {
            value: "param".to_string(),
            children: vec![node_i32.clone(), node_i32.clone()],
        };
        let result = Tree {
            value: "result".to_string(),
            children: vec![node_i32.clone()],
        };
        let mut children = vec![def, param, result];
        children.append(&mut to_tree(self.visit(&mut state, &expr)?));
        let func = Tree {
            value: "func".to_string(),
            children: children,
        };
        return Ok(Tree {
            value: "module".to_string(),
            children: vec![func],
        });
    }

    fn visit_sym(&mut self, _state: &mut State, _expr: &Sym) -> Res {
        panic!("Sym not implemented in wasm");
    }

    fn visit_prim(&mut self, expr: &Prim) -> Res {
        use Prim::*;
        match expr {
            I32(n, _) => Ok(vec!["i32.const ".to_string() + &n.to_string()]),
            _ => unimplemented!(),
        }
    }

    fn visit_apply(&mut self, _state: &mut State, _expr: &Apply) -> Res {
        panic!("Apply not implemented in wasm");
    }

    fn visit_let(&mut self, _state: &mut State, _expr: &Let) -> Res {
        panic!("Let not implemented in wasm");
    }

    fn visit_un_op(&mut self, state: &mut State, expr: &UnOp) -> Res {
        use Prim::*;
        let mut res = Vec::new();
        let mut inner = self.visit(state, &expr.inner)?;
        let info = expr.get_info();
        match expr.name.as_str() {
            "+" => {
                res.append(&mut inner);
            },
            "-" => {
                res.append(&mut self.visit_prim(&I32(0, expr.clone().get_info()))?);
                res.append(&mut inner);
                res.push("i32.sub".to_string());
            },
            op => return Err(CompilerError::UnknownPrefixOperator(op.to_string(), info)),
        };
        return Ok(res);
    }
    fn visit_bin_op(&mut self, state: &mut State, expr: &BinOp) -> Res {
        let info = expr.get_info();
        let mut res = Vec::new();
        res.append(&mut self.visit(state, &expr.left)?);
        res.append(&mut self.visit(state, &expr.right)?);
        // TODO: require 2 children
        let s = match expr.name.as_str() {
            "*" => "i32.mul".to_string(),
            "+" => "i32.add".to_string(),
            "/" => "i32.div_s".to_string(), // TODO: require divisibility
            "-" => "i32.sub".to_string(),
            "^" => "i32.pow".to_string(), // TODO: require pos pow
            op => return Err(CompilerError::UnknownInfixOperator(op.to_string(), info)),
        };
        res.push(s);
        return Ok(res);
    }

    fn handle_error(&mut self, _state: &mut State, expr: &Err) -> Res {
        Err(CompilerError::FailedParse(expr.msg.clone(), expr.get_info()))
    }
}
