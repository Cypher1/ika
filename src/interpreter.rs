use super::ast::*;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum InterpreterError {
    UnknownInfixOperator(String, Info),
    UnknownPrefixOperator(String, Info),
    FailedParse(String, Info),
    TypeMismatch(String, Prim, Info),
    TypeMismatch2(String, Prim, Prim, Info),
}

type Frame = HashMap<String, Node>;

// Walks the AST interpreting it.
pub struct Interpreter {
}

impl Default for Interpreter {
    fn default() -> Interpreter {
        Interpreter {}
    }
}

type Res = Result<Prim, InterpreterError>;
type State = Vec<Frame>;
impl Visitor<State, Prim, Prim, InterpreterError> for Interpreter {

    fn visit_root(&mut self, expr: &Node) -> Res {
        let mut state = vec![Frame::new()];
        self.visit(&mut state, expr)
    }

    fn visit_sym(&mut self, state: &mut State, expr: &Sym) -> Res {
        use Prim::*;
        let info = expr.clone().get_info();
        match expr.name.as_str() {
            "true" => return Ok(Bool(true, info)),
            "false" => return Ok(Bool(false, info)),
            n => {
                for frame in state.iter().rev() {
                    match frame.get(n) {
                        Some(val) => {
                            let mut next = state.clone();
                            next.push(Frame::new());
                            let result = self.visit(
                            &mut next,
                            &val.clone());
                            return result
                        }, // This is the variable
                        None => {},
                    }
                    // Not in this frame, go back up.
                }
                panic!(format!("{:?} could not be found in scope.", n))
            }
        }
    }

    fn visit_prim(&mut self, expr: &Prim) -> Res {
        Ok(expr.clone())
    }

    fn visit_apply(&mut self, state: &mut State, expr: &Apply) -> Res {
        println!("apply: {:?} to {:?}", expr.args, expr.inner);
        // Add a new scope
        state.push(Frame::new());
        for arg in expr.args.iter() {
            let n = Node::LetNode(arg.clone());
            println!("def: {:?}", n);
            self.visit(state, &n)?;
        }
        // Visit the expr.inner
        let res = self.visit(state, &*expr.inner);
        state.pop();
        res
    }

    fn visit_let(&mut self, state: &mut State, expr: &Let) -> Res {
        println!("let: {:?}", expr);
        use Prim::*;
        match (state.last_mut(), expr.value.as_ref()) {
            (None, _) => panic!("there is no stack frame"),
            (_, None) => {},
            (Some(frame), Some(val)) => {
                frame.insert(expr.name.clone(), *val.clone());
            }
        }
        return Ok(Unit(expr.clone().get_info()));
    }

    fn visit_un_op(&mut self, state: &mut State, expr: &UnOp) -> Res {
        use Prim::*;
        let i = self.visit(state, &expr.inner)?;
        let info = expr.clone().get_info();
        match expr.name.as_str() {
            "!" => match i {
                Bool(n, _) => Ok(Bool(!n, info)),
                _ => Err(InterpreterError::TypeMismatch("!".to_string(), i, info))
            },
            "+" => match i {
                I32(n, _) => Ok(I32(n, info)),
                _ => Err(InterpreterError::TypeMismatch("+".to_string(), i, info))

            }
            "-" => match i {
                I32(n, _) => Ok(I32(-n, info)),
                _ => Err(InterpreterError::TypeMismatch("-".to_string(), i, info))
            },
            op => Err(InterpreterError::UnknownPrefixOperator(op.to_string(), info)),
        }
    }

    fn visit_bin_op(&mut self, state: &mut State, expr: &BinOp) -> Res {
        use Prim::*;
        let info = expr.clone().get_info();
        let l = self.visit(state, &expr.left)?;
        let r = self.visit(state, &expr.right)?;
        match expr.name.as_str() {
            ";" => match (&l, &r) {
                (_, r) => Ok(r.clone()),
            },
            "+" => match (&l, &r) {
                (Bool(l, _), Bool(r, _)) => Ok(I32(if *l {1} else {0} + if *r {1} else {0}, info)),
                (Bool(l, _), I32(r, _)) => Ok(I32(if *l {1} else {0} + r, info)),
                (Bool(l, _), Str(r, _)) => Ok(Str(l.to_string() + &r.to_string(), info)),
                (I32(l, _), Bool(r, _)) => Ok(I32(l + if *r {1} else {0}, info)),
                (I32(l, _), I32(r, _)) => Ok(I32(l + r, info)),
                (I32(l, _), Str(r, _)) => Ok(Str(l.to_string() + &r.to_string(), info)),
                (Str(l, _), Bool(r, _)) => Ok(Str(l.to_string() + &r.to_string(), info)),
                (Str(l, _), I32(r, _)) => Ok(Str(l.to_string() + &r.to_string(), info)),
                (Str(l, _), Str(r, _)) => Ok(Str(l.to_string() + &r.to_string(), info)),
                _ => Err(InterpreterError::TypeMismatch2("+".to_string(), l, r, info))
            },
            "==" => match (&l, &r) {
                (Bool(l, _), Bool(r, _)) => Ok(Bool(*l == *r, info)),
                (I32(l, _), I32(r, _)) => Ok(Bool(l == r, info)),
                (Str(l, _), Str(r, _)) => Ok(Bool(l.to_string() == r.to_string(), info)),
                _ => Err(InterpreterError::TypeMismatch2("==".to_string(), l, r, info))
            },
            "!=" => match (&l, &r) {
                (Bool(l, _), Bool(r, _)) => Ok(Bool(*l != *r, info)),
                (I32(l, _), I32(r, _)) => Ok(Bool(l != r, info)),
                (Str(l, _), Str(r, _)) => Ok(Bool(l.to_string() != r.to_string(), info)),
                _ => Err(InterpreterError::TypeMismatch2("!=".to_string(), l, r, info))
            },
            ">" => match (&l, &r) {
                (Bool(l, _), Bool(r, _)) => Ok(Bool(*l > *r, info)),
                (I32(l, _), I32(r, _)) => Ok(Bool(l > r, info)),
                (Str(l, _), Str(r, _)) => Ok(Bool(l.to_string() > r.to_string(), info)),
                _ => Err(InterpreterError::TypeMismatch2(">".to_string(), l, r, info))
            },
            "<" => match (&l, &r) {
                (Bool(l, _), Bool(r, _)) => Ok(Bool(*l < *r, info)),
                (I32(l, _), I32(r, _)) => Ok(Bool(l < r, info)),
                (Str(l, _), Str(r, _)) => Ok(Bool(l.to_string() < r.to_string(), info)),
                _ => Err(InterpreterError::TypeMismatch2("<".to_string(), l, r, info))
            },
            ">=" => match (&l, &r) {
                (Bool(l, _), Bool(r, _)) => Ok(Bool(*l >= *r, info)),
                (I32(l, _), I32(r, _)) => Ok(Bool(l >= r, info)),
                (Str(l, _), Str(r, _)) => Ok(Bool(l.to_string() >= r.to_string(), info)),
                _ => Err(InterpreterError::TypeMismatch2(">=".to_string(), l, r, info))
            },
            "<=" => match (&l, &r) {
                (Bool(l, _), Bool(r, _)) => Ok(Bool(*l <= *r, info)),
                (I32(l, _), I32(r, _)) => Ok(Bool(l <= r, info)),
                (Str(l, _), Str(r, _)) => Ok(Bool(l.to_string() <= r.to_string(), info)),
                _ => Err(InterpreterError::TypeMismatch2("<=".to_string(), l, r, info))
            },
            "-" => match (&l, &r) {
                (I32(l, _), Bool(r, _)) => Ok(I32(l - if *r {1} else {0}, info)),
                (I32(l, _), I32(r, _)) => Ok(I32(l - r, info)),
                _ => Err(InterpreterError::TypeMismatch2("-".to_string(), l, r, info))
            },
            "*" => match (&l, &r) {
                (Bool(l, _), I32(r, _)) => Ok(I32(if *l {*r} else {0}, info)),
                (Bool(l, _), Str(r, _)) => Ok(Str(if *l {r.to_string()} else {"".to_string()}, info)),
                (I32(l, _), Bool(r, _)) => Ok(I32(if *r {*l} else {0}, info)),
                (I32(l, _), I32(r, _)) => Ok(I32(l * r, info)),
                // (I32(l, _), Str(r, _)) => Ok(Str(l.to_string() * r, info)),
                (Str(l, _), Bool(r, _)) => Ok(Str(if *r {l.to_string()} else {"".to_string()}, info)),
                // (Str(l, _), I32(r, _)) => Ok(Str(l * r.to_string(), info)),
                _ => Err(InterpreterError::TypeMismatch2("*".to_string(), l, r, info))
            },
            "/" => match (&l, &r) {
                (I32(l, _), I32(r, _)) => Ok(I32(l / r, info)),
                _ => Err(InterpreterError::TypeMismatch2("/".to_string(), l, r, info))
            },
            "%" => match (&l, &r) {
                (I32(l, _), I32(r, _)) => Ok(I32(l % r, info)),
                _ => Err(InterpreterError::TypeMismatch2("%".to_string(), l, r, info))
            },
            "&&" => match (&l, &r) {
                (Bool(l, _), Bool(r, _)) => Ok(Bool(*l&&*r, info)),
                _ => Err(InterpreterError::TypeMismatch2("&&".to_string(), l, r, info))

            },
            "||" => match (&l, &r) {
                (Bool(l, _), Bool(r, _)) => Ok(Bool(*l||*r, info)),
                _ => Err(InterpreterError::TypeMismatch2("||".to_string(), l, r, info))

            },
            "^" => match (&l, &r) {
                (I32(l, _), Bool(r, _)) => Ok(I32(if *r {*l} else {1}, info)),
                (I32(l, _), I32(r, _)) => Ok(I32(i32::pow(*l, *r as u32), info)), // TODO: require pos pow
                _ => Err(InterpreterError::TypeMismatch2("^".to_string(), l, r, info))
            },
            op => Err(InterpreterError::UnknownInfixOperator(op.to_string(), info)),
        }
    }

    fn handle_error(&mut self, _state: &mut State, expr: &Err) -> Res {
        Err(InterpreterError::FailedParse(expr.msg.to_string(), expr.get_info()))
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;
    use super::Res;
    use super::super::parser;
    use super::super::ast::*;
    use Prim::*;
    use Node::*;

    #[test]
    fn eval_num() {
        let mut interp = Interpreter::default();
        let tree = PrimNode(I32(12, Info::default()));
        assert_eq!(interp.visit_root(&tree), Ok(I32(12, Info::default())));
    }

    fn eval_str(s: String) -> Res {
        let ast = parser::parse(s);
        let mut interp = Interpreter::default();
        interp.visit_root(&ast)
    }

    #[test]
    fn parse_and_eval_bool() {
        assert_eq!(eval_str("true".to_string()), Ok(Bool(true, Info::default())));
    }

    #[test]
    fn parse_and_eval_bool_and() {
        assert_eq!(eval_str("true&&true".to_string()), Ok(Bool(true, Info::default())));
        assert_eq!(eval_str("false&&true".to_string()), Ok(Bool(false, Info::default())));
        assert_eq!(eval_str("true&&false".to_string()), Ok(Bool(false, Info::default())));
        assert_eq!(eval_str("false&&false".to_string()), Ok(Bool(false, Info::default())));
    }

    #[test]
    fn parse_and_eval_bool_or() {
        assert_eq!(eval_str("true||true".to_string()), Ok(Bool(true, Info::default())));
        assert_eq!(eval_str("false||true".to_string()), Ok(Bool(true, Info::default())));
        assert_eq!(eval_str("true||false".to_string()), Ok(Bool(true, Info::default())));
        assert_eq!(eval_str("false||false".to_string()), Ok(Bool(false, Info::default())));
    }

    #[test]
    fn parse_and_eval_bool_eq() {
        assert_eq!(eval_str("true==true".to_string()), Ok(Bool(true, Info::default())));
        assert_eq!(eval_str("false==true".to_string()), Ok(Bool(false, Info::default())));
        assert_eq!(eval_str("true==false".to_string()), Ok(Bool(false, Info::default())));
        assert_eq!(eval_str("false==false".to_string()), Ok(Bool(true, Info::default())));
    }

    #[test]
    fn parse_and_eval_i32() {
        assert_eq!(eval_str("32".to_string()), Ok(I32(32, Info::default())));
    }

    #[test]
    fn parse_and_eval_i32_eq() {
        assert_eq!(eval_str("0==0".to_string()), Ok(Bool(true, Info::default())));
        assert_eq!(eval_str("-1==1".to_string()), Ok(Bool(false, Info::default())));
        assert_eq!(eval_str("1==123".to_string()), Ok(Bool(false, Info::default())));
        assert_eq!(eval_str("1302==1302".to_string()), Ok(Bool(true, Info::default())));
    }

    #[test]
    fn parse_and_eval_i32_pow() {
        assert_eq!(eval_str("2^3".to_string()), Ok(I32(8, Info::default())));
        assert_eq!(eval_str("3^2".to_string()), Ok(I32(9, Info::default())));
        assert_eq!(eval_str("-4^2".to_string()), Ok(I32(16, Info::default())));
        assert_eq!(eval_str("2^3^2".to_string()), Ok(I32(512, Info::default())));
    }

    #[test]
    fn parse_and_eval_str() {
        assert_eq!(eval_str("\"32\"".to_string()), Ok(Str("32".to_string(), Info::default())));
    }

    #[test]
    fn parse_and_eval_let() {
        assert_eq!(eval_str("x=3;x".to_string()), Ok(I32(3, Info::default())));
    }
}
