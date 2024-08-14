use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::Expr;
use crate::obj::{Obj, ObjRef};
use crate::profile::Profile;
use crate::token::Token;

pub struct Evaluator {
    vars: Vec<HashMap<String, ObjRef>>,
    profile: Profile,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            vars: vec![HashMap::new()],
            profile: Profile::default(),
        }
    }
    
    pub fn eval(&mut self, expr: &Expr) -> ObjRef {
        match expr {
            Expr::Identifier(id) => {
                self.vars
                    .iter()
                    .rev()
                    .find_map(|scope| scope.get(id))
                    .cloned()
                    .unwrap_or_else(|| panic!("Undefined variable {}", id))
                    .clone()
            }
            Expr::Number(n) => Obj::Number(*n).as_ref(),
            Expr::StringLiteral(s) => Obj::String(s.clone()).as_ref(),
            Expr::Boolean(b) => Obj::Boolean(*b).as_ref(),
            Expr::Nil => Obj::Nil.as_ref(),
            Expr::List(l) => Obj::List(l.iter().map(|e| self.eval(e)).collect()).as_ref(),
            Expr::Map(m) => Obj::Map(
                m.iter()
                    .map(|(k, v)| (match self.eval(k).into_inner() {
                        Obj::String(s) => s,
                        _ => panic!("Map property must be string")
                    }, self.eval(v)))
                    .collect(),
            ).as_ref(),
            Expr::Call(callee, args) => {
                let callee = self.eval(callee);
                let args = args.iter().map(|e| self.eval(e)).collect();
                self.call(callee, args)
            }
            Expr::Assign(id, value) => {
                let value = self.eval(value);
                let id = match **id {
                    Expr::Identifier(id) => id,
                    _ => panic!("Expected identifier"),
                };
                self.vars.last_mut().unwrap().insert(id, &value);
                value
            }
            Expr::Block(exprs) => {
                let mut result = Obj::Nil;
                for expr in exprs {
                    result = self.eval(expr);
                }
                result
            }
            Expr::If(cond, then, else_) => {
                let cond = self.eval(cond);
                if cond.is_truthy() {
                    self.eval(then)
                } else if let Some(else_) = else_ {
                    self.eval(else_)
                } else {
                    Obj::Nil
                }
            }
            Expr::While(cond, body) => {
                let mut result = Obj::Nil;
                while self.eval(cond).is_truthy() {
                    result = self.eval(body);
                }
                result
            }
            Expr::
        }
    }
}
    