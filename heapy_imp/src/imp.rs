/*
m, n ∈ N natural numbers
x ∈ Id variables
l ∈ Loc locations
b ::= true | false boolean constants

e ::= x read from store
    | !x read from heap
    | N | b constants
    | e + e addition
    | ¬ e boolean negation
    | e ∧ e boolean conjunction
    | e ≤ e comparison
*/

use std::{collections::HashMap, marker};

#[derive(Clone)]
pub enum Expression {
    StackVar {
        x: String,
    },
    HeapVar {
        x: String,
    },
    NatConstant {
        n: i32,
    },
    BoolConstant {
        b: bool,
    },
    Add {
        ex1: Box<Expression>,
        ex2: Box<Expression>,
    },
    Negation {
        ex1: Box<Expression>,
    },
    Conjunction {
        ex1: Box<Expression>,
        ex2: Box<Expression>,
    },
    Comparision {
        ex1: Box<Expression>,
        ex2: Box<Expression>,
    },
}

/*
s ::= x := e assignment to store
    | !x := e update heap location
    | x = y alias heap location
    | x := new(e) put new value on heap
    | s; s sequencing
    | if e then s else s conditional
    | skip no - op
    | while e do s loop
*/

#[derive(Clone)]
pub enum Statement {
    StackAssignment {
        x: String,
        ex1: Box<Expression>,
    },
    HeapUpdate {
        x: String,
        ex1: Box<Expression>,
    },
    HeapAlias {
        x: String,
        y: String,
    },
    HeapNew {
        x: String,
        ex1: Box<Expression>,
    },
    Sequence {
        st1: Box<Statement>,
        st2: Box<Statement>,
    },
    IfThenElse {
        condition: Box<Expression>,
        then_branch: Box<Statement>,
        else_branch: Box<Statement>,
    },
    Skip,
    While {
        condition: Box<Expression>,
        st: Box<Statement>,
    },
}

#[derive(Clone)]
pub enum ExType {
    NatType,
    BoolType,
    PointerType
}

fn printExpression(exp: Expression) -> String {
    match exp {
        Expression::StackVar { x } => String::from(x) + " ",
        Expression::HeapVar { x } => String::from("!") + &x + " ",
        Expression::NatConstant { n } => n.to_string(),
        Expression::BoolConstant { b } => b.to_string(),
        Expression::Add { ex1, ex2 } => {
            let t1 = printExpression(*ex1);
            let t2 = printExpression(*ex2);
            String::from("(") + &t1.to_owned() + ") + (" + &t2.to_owned() + ")"
        }
        Expression::Negation { ex1 } => {
            let t1 = printExpression(*ex1);
            String::from("not (") + &t1.to_owned() + ")"
        }
        Expression::Conjunction { ex1, ex2 } => {
            let t1 = printExpression(*ex1);
            let t2 = printExpression(*ex2);
            String::from("(") + &t1.to_owned() + ") and (" + &t2.to_owned() + ")"
        }
        Expression::Comparision { ex1, ex2 } => {
            let t1 = printExpression(*ex1);
            let t2 = printExpression(*ex2);
            String::from("(") + &t1.to_owned() + ") <=> (" + &t2.to_owned() + ")"
        }
    }
}

fn printStatement(st: Statement) -> String {
    match st {
        Statement::StackAssignment { x, ex1 } => {
            let t1 = printExpression(*ex1);
            String::from("") + &x + " = " + &t1.to_owned()
        }
        Statement::HeapUpdate { x, ex1 } => {
            let t1 = printExpression(*ex1);
            String::from("!") + &x + " = " + &t1.to_owned()
        }
        Statement::HeapAlias { x, y } => String::from("") + &x + &y,
        Statement::HeapNew { x, ex1 } => {
            let t1 = printExpression(*ex1);
            String::from("") + &x + " = new(" + &t1.to_owned() + ")"
        }
        Statement::Sequence { st1, st2 } => {
            let t1 = printStatement(*st1);
            let t2 = printStatement(*st2);
            String::from("") + &t1.to_owned() + "; " + &t2.to_owned()
        }
        Statement::IfThenElse {
            condition,
            then_branch,
            else_branch,
        } => {
            let cnd = printExpression(*condition);
            let thenb = printStatement(*then_branch);
            let elseb = printStatement(*else_branch);
            String::from("if (")
                + &cnd.to_owned()
                + ") then { "
                + &thenb.to_owned()
                + " } else { "
                + &elseb.to_owned()
                + "}"
        }
        Statement::Skip => String::from("skip"),
        Statement::While { condition, st } => {
            let cnd = printExpression(*condition);
            let t1 = printStatement(*st);
            String::from("while (") + &cnd.to_owned() + ")" + " do { " + &t1.to_owned() + "}"
        }
    }
}



fn typeEvalExp(exp: Expression, stack: HashMap<String, ExType>, heap: HashMap<String, ExType>) -> Result<ExType, String> {
    match exp {
        Expression::StackVar { x } => {
            if stack.contains_key(&x) {
                Result::Ok(stack.get(&x).unwrap().clone())
            } else {
                Result::Err(format!("undeclared stack variable: {}", x))
            }
        },
        Expression::HeapVar { x } => {
            if stack.contains_key(&x) {
                match stack.get(&x).unwrap().clone() {
                    ExType::PointerType => Result::Ok(heap.get(&x).unwrap().clone()),
                    _ => Result::Err(format!("variable: {} is not a pointer", x))
                }
            } else {
                Result::Err("Null Pointer Exception".to_string())
            }
        },
        Expression::NatConstant { n } => Result::Ok(ExType::NatType),
        Expression::BoolConstant { b } => Result::Ok(ExType::BoolType),
        Expression::Add { ex1, ex2 } => {
            let p1 = printExpression(*ex1.clone());
            let p2 = printExpression(*ex2.clone());
            match (typeEvalExp(*ex1, stack.clone(), heap.clone()), typeEvalExp(*ex2, stack, heap)) {
                (Ok(ExType::NatType), Ok(ExType::NatType)) => Result::Ok(ExType::NatType),
                (Err(e), _) => Result::Err(e),
                (_, Err(e)) => Result::Err(e),
                (Ok(ExType::NatType), Ok(_)) => {
                    Result::Err(format!("Expression: {} should be of NatType", p1))
                },
                (Ok(_), Ok(ExType::NatType)) => {
                    Result::Err(format!("Expression: {} should be of NatType", p2))
                },
                (Ok(_), Ok(_)) => Result::Err(format!("Expression: {} & Expression: {} should be of NatType", p1, p2))
            }
        },
        Expression::Negation { ex1 } => {
            let p1 = printExpression(*ex1.clone());
            match typeEvalExp(*ex1, stack, heap) {
                Ok(tp) => {
                    match tp {
                        ExType::BoolType => Result::Ok(ExType::BoolType),
                        _ => Result::Err(format!("Expression: {} should be of BoolType", p1)),
                    }
                },
                Err(e) => Result::Err(e),
            }
        },
        Expression::Conjunction { ex1, ex2 } => {
            let p1 = printExpression(*ex1.clone());
            let p2 = printExpression(*ex2.clone());
            match (typeEvalExp(*ex1, stack.clone(), heap.clone()), typeEvalExp(*ex2, stack, heap)) {
                (Ok(ExType::BoolType), Ok(ExType::BoolType)) => Result::Ok(ExType::BoolType),
                (Err(e), _) => Result::Err(e),
                (_, Err(e)) => Result::Err(e),
                (Ok(ExType::NatType), Ok(_)) => {
                    Result::Err(format!("Expression: {} should be of BoolType", p1))
                },
                (Ok(_), Ok(ExType::NatType)) => {
                    Result::Err(format!("Expression: {} should be of BoolType", p2))
                },
                (Ok(_), Ok(_)) => Result::Err(format!("Expression: {} & Expression: {} should be of BoolType", p1, p2))
            }
        },
        Expression::Comparision { ex1, ex2 } => {
            let p1 = printExpression(*ex1.clone());
            let p2 = printExpression(*ex2.clone());
            match (typeEvalExp(*ex1, stack.clone(), heap.clone()), typeEvalExp(*ex2, stack, heap)) {
                (Ok(ExType::NatType), Ok(ExType::NatType)) => Result::Ok(ExType::BoolType),
                (Err(e), _) => Result::Err(e),
                (_, Err(e)) => Result::Err(e),
                (Ok(ExType::NatType), Ok(_)) => {
                    Result::Err(format!("Expression: {} should be of NatType", p1))
                },
                (Ok(_), Ok(ExType::NatType)) => {
                    Result::Err(format!("Expression: {} should be of NatType", p2))
                },
                (Ok(_), Ok(_)) => Result::Err(format!("Expression: {} & Expression: {} should be of NatType", p1, p2))
            }
        },
    }
}


fn typeCheck(st: Statement, stack: HashMap<String, ExType>, heap: HashMap<String, ExType>) {
    match st {
        Statement::StackAssignment { x, ex1 } => todo!(),
        Statement::HeapUpdate { x, ex1 } => todo!(),
        Statement::HeapAlias { x, y } => todo!(),
        Statement::HeapNew { x, ex1 } => todo!(),
        Statement::Sequence { st1, st2 } => todo!(),
        Statement::IfThenElse { condition, then_branch, else_branch } => todo!(),
        Statement::Skip => todo!(),
        Statement::While { condition, st } => todo!(),
    }
}