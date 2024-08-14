use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::iter::{FlatMap, Map};
use std::rc::Rc;
use crate::ast::Expr;

#[derive(Debug, Clone)]
pub enum Obj {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    List(Vec<ObjRef>),
    Map(Map<String, ObjRef>),
    Closure {
        params: Vec<String>,
        body: Expr,
        env: Vec<HashMap<String, ObjRef>>,
    }
}

pub type ObjRef = Rc<RefCell<Obj>>;

impl Hash for Obj {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Obj::Number(n) => n.to_string().hash(state),
            Obj::String(s) => s.hash(state),
            Obj::Boolean(b) => b.to_string().hash(state),
            Obj::Nil => "nil".hash(state),
            Obj::List(l) => format!("{:?}", l).hash(state), // This is a hack, we should implement a better hash function
            Obj::Map(m) => format!("{:?}", m).hash(state),
            Obj::Closure { .. } => {
                //TODO: Implement a better hash function
                "closure".hash(state);
            }
        }
    }
}

impl PartialEq for Obj {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Obj::Number(a), Obj::Number(b)) => a == b,
            (Obj::String(a), Obj::String(b)) => a == b,
            (Obj::Boolean(a), Obj::Boolean(b)) => a == b,
            (Obj::Nil, Obj::Nil) => true,
            (Obj::List(a), Obj::List(b)) => a == b,
            (Obj::Map(a), Obj::Map(b)) => a.iter().eq(b),
            (Obj::Closure {..}, Obj::Closure{..}) => false,
            _ => false
        }
    }
}

impl Eq for Obj {}

impl Obj {
    pub fn is_truthy(&self) -> bool {
        match self {
            Obj::Number(n) => *n != 0.0,
            Obj::String(s) => !s.is_empty(),
            Obj::Boolean(b) => *b,
            Obj::Nil => false,
            Obj::List(l) => !l.is_empty(),
            Obj::Map(m) => !m.is_empty(),
            Obj::Closure {..} => true
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Obj::Number(n) => n.to_string(),
            Obj::String(s) => s.clone(),
            Obj::Boolean(b) => b.to_string(),
            Obj::Nil => "nil".to_string(),
            Obj::List(l) => format!("{:?}", l),
            Obj::Map(m) => format!("{:?}", m),
            Obj::Closure {..} => "lambda".to_string()
        }
    }
    
    pub fn as_ref(&self) -> ObjRef {
        return Rc::new(RefCell::new(self.clone()))
    }
}