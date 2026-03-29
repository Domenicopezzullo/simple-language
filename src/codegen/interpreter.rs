use crate::ir::{Condition, IR, IRArg};

use std::collections::HashMap;

pub struct Interpreter {
    functions: HashMap<String, Vec<IR>>,
    variables: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    fn resolve(&self, arg: &IRArg) -> IRArg {
        match arg {
            IRArg::Var(name) => {
                if let Some(n) = self.variables.get(name) {
                    IRArg::Int(*n)
                } else {
                    eprintln!("Error: undefined variable '{}'", name);
                    IRArg::Int(0)
                }
            }
            other => other.clone(),
        }
    }

    pub fn run(&mut self, nodes: &[IR]) {
        for node in nodes {
            self.eval(node);
        }
    }
    fn eval(&mut self, node: &IR) {
        match node {
            IR::FuncDef { name, body } => {
                self.functions.insert(name.clone(), body.clone());
            }
            IR::If { condition, body } => {
                let result = match condition {
                    Condition::Equals(a, b) => self.resolve(a) == self.resolve(b),
                };
                if result {
                    self.run(body);
                }
            }
            IR::Funcall { name, args } => {
                if name == "@println" {
                    for arg in args {
                        match arg {
                            IRArg::Var(name) => {
                                if let Some(n) = self.variables.get(name) {
                                    println!("{}", n)
                                } else {
                                    eprintln!("Undefined variable '{}'", name);
                                }
                            }
                            IRArg::Int(i) => println!("{}", i),
                            IRArg::Str(e) => println!("{}", e),
                        }
                    }
                    return;
                }
                if let Some(body) = self.functions.get(name).cloned() {
                    self.run(&body);
                } else {
                    eprintln!("Error: undefined function '{}'", name)
                }
            }
            IR::VarAssignment { name, value } => {
                self.variables.insert(name.clone(), *value);
            }
        }
    }
}
