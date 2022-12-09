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

use std::{collections::HashMap, fmt::format, marker};

#[derive(Clone)]
pub enum Expression {
    StackVar {
        x: String,
    },
    HeapRead {
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
        ex1: Expression,
    },
    HeapUpdate {
        x: String,
        ex1: Expression,
    },
    HeapAlias {
        x: String,
        y: String,
    },
    HeapNew {
        x: String,
        ex1: Expression,
    },
    Sequence {
        st1: Box<Statement>,
        st2: Box<Statement>,
    },
    IfThenElse {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Box<Statement>,
    },
    Skip,
    While {
        condition: Expression,
        st: Box<Statement>,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub enum ExType {
    NatType,
    BoolType,
    PointerType,
}

pub fn printExpression(exp: Expression) -> String {
    match exp {
        Expression::StackVar { x } => String::from(x) + " ",
        Expression::HeapRead { x } => String::from("!") + &x + " ",
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
            String::from("(") + &t1.to_owned() + ") <= (" + &t2.to_owned() + ")"
        }
    }
}

pub fn printStatement(st: Statement) -> String {
    match st {
        Statement::StackAssignment { x, ex1 } => {
            let t1 = printExpression(ex1);
            String::from("") + &x + " = " + &t1.to_owned()
        }
        Statement::HeapUpdate { x, ex1 } => {
            let t1 = printExpression(ex1);
            String::from("!") + &x + " = " + &t1.to_owned()
        }
        Statement::HeapAlias { x, y } => String::from("") + &x + &y,
        Statement::HeapNew { x, ex1 } => {
            let t1 = printExpression(ex1);
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
            let cnd = printExpression(condition);
            let thenb = printStatement(*then_branch);
            let elseb = printStatement(*else_branch);
            String::from("if (")
                + &cnd.to_owned()
                + ") then { "
                + &thenb.to_owned()
                + " } else { "
                + &elseb.to_owned()
                + " }"
        }
        Statement::Skip => String::from("skip"),
        Statement::While { condition, st } => {
            let cnd = printExpression(condition);
            let t1 = printStatement(*st);
            String::from("while (") + &cnd.to_owned() + ")" + " do { " + &t1.to_owned() + "}"
        }
    }
}

pub fn printType(tp: ExType) -> String {
    match tp {
        ExType::NatType => String::from("NatType"),
        ExType::BoolType => String::from("BoolType"),
        ExType::PointerType => String::from("PointerType"),
    }
}

pub fn typeCheckExp(exp: Expression, stack: &HashMap<String, ExType>) -> Result<ExType, String> {
    match exp {
        Expression::StackVar { x } => {
            if stack.contains_key(&x) {
                Result::Ok(stack.get(&x).unwrap().clone())
            } else {
                Result::Err(format!("undeclared stack variable: {}", x))
            }
        }
        Expression::HeapRead { x } => {
            if stack.contains_key(&x) {
                match stack.get(&x).unwrap().clone() {
                    ExType::PointerType => Result::Ok(ExType::NatType),
                    _ => Result::Err(format!("variable: {} is not a pointer", x)),
                }
            } else {
                Result::Err("Null Pointer Exception".to_string())
            }
        }
        Expression::NatConstant { n } => Result::Ok(ExType::NatType),
        Expression::BoolConstant { b } => Result::Ok(ExType::BoolType),
        Expression::Add { ex1, ex2 } => {
            let p1 = printExpression(*ex1.clone());
            let p2 = printExpression(*ex2.clone());
            match (typeCheckExp(*ex1, stack), typeCheckExp(*ex2, stack)) {
                (Ok(ExType::NatType), Ok(ExType::NatType)) => Result::Ok(ExType::NatType),
                (Err(e), _) => Result::Err(e),
                (_, Err(e)) => Result::Err(e),
                (Ok(ExType::NatType), Ok(_)) => {
                    Result::Err(format!("Expression: {} should be of NatType", p1))
                }
                (Ok(_), Ok(ExType::NatType)) => {
                    Result::Err(format!("Expression: {} should be of NatType", p2))
                }
                (Ok(_), Ok(_)) => Result::Err(format!(
                    "Expression: {} & Expression: {} should be of NatType",
                    p1, p2
                )),
            }
        }
        Expression::Negation { ex1 } => {
            let p1 = printExpression(*ex1.clone());
            match typeCheckExp(*ex1, stack) {
                Ok(tp) => match tp {
                    ExType::BoolType => Result::Ok(ExType::BoolType),
                    _ => Result::Err(format!("Expression: {} should be of BoolType", p1)),
                },
                Err(e) => Result::Err(e),
            }
        }
        Expression::Conjunction { ex1, ex2 } => {
            let p1 = printExpression(*ex1.clone());
            let p2 = printExpression(*ex2.clone());
            match (typeCheckExp(*ex1, stack), typeCheckExp(*ex2, stack)) {
                (Ok(ExType::BoolType), Ok(ExType::BoolType)) => Result::Ok(ExType::BoolType),
                (Err(e), _) => Result::Err(e),
                (_, Err(e)) => Result::Err(e),
                (Ok(ExType::NatType), Ok(_)) => {
                    Result::Err(format!("Expression: {} should be of BoolType", p1))
                }
                (Ok(_), Ok(ExType::NatType)) => {
                    Result::Err(format!("Expression: {} should be of BoolType", p2))
                }
                (Ok(_), Ok(_)) => Result::Err(format!(
                    "Expression: {} & Expression: {} should be of BoolType",
                    p1, p2
                )),
            }
        }
        Expression::Comparision { ex1, ex2 } => {
            let p1 = printExpression(*ex1.clone());
            let p2 = printExpression(*ex2.clone());
            match (typeCheckExp(*ex1, stack), typeCheckExp(*ex2, stack)) {
                (Ok(ExType::NatType), Ok(ExType::NatType)) => Result::Ok(ExType::BoolType),
                (Err(e), _) => Result::Err(e),
                (_, Err(e)) => Result::Err(e),
                (Ok(ExType::NatType), Ok(_)) => {
                    Result::Err(format!("Expression: {} should be of NatType", p1))
                }
                (Ok(_), Ok(ExType::NatType)) => {
                    Result::Err(format!("Expression: {} should be of NatType", p2))
                }
                (Ok(_), Ok(_)) => Result::Err(format!(
                    "Expression: {} & Expression: {} should be of NatType",
                    p1, p2
                )),
            }
        }
    }
}

pub fn typeCheck(st: Statement, stack: &mut HashMap<String, ExType>) -> Option<String> {
    match st {
        Statement::StackAssignment { x, ex1 } => {
            let p1 = printExpression(ex1.clone());
            match typeCheckExp(ex1, stack) {
                Ok(tp) => {
                    if tp == ExType::PointerType {
                        Some("Cannot Assign pointer type to a stack variable".to_string())
                    } else {
                        if stack.contains_key(&x) {
                            if stack.get(&x).unwrap() != &tp {
                                Some(format!(
                                    "Variable {} already exists & does not match the type of {}",
                                    x, p1
                                ))
                            } else {
                                None
                            }
                        } else {
                            stack.insert(x, tp);
                            None
                        }
                    }
                }
                Err(e) => Some(e),
            }
        }
        Statement::HeapUpdate { x, ex1 } => {
            let p1 = printExpression(ex1.clone());
            if !stack.contains_key(&x) {
                Some(format!("Undefined reference to variable {}", x))
            } else {
                match stack.get(&x).unwrap() {
                    ExType::PointerType => match typeCheckExp(ex1, stack) {
                        Ok(ExType::NatType) => None,
                        Ok(_) => Some(format!("Expression: {} should be NatType", p1)),
                        Err(e) => Some(e),
                    },
                    _ => Some(format!("{} is not PointerType", x)),
                }
            }
        }
        Statement::HeapAlias { x, y } => {
            if !stack.contains_key(&y) {
                Some(format!("Undefined reference to variable {}", y))
            } else {
                match stack.get(&y).unwrap() {
                    ExType::PointerType => {
                        if stack.contains_key(&x) {
                            if stack.get(&x).unwrap().clone() != ExType::PointerType {
                                Some(format!(
                                    "Variable {} already exists and is not PointerType",
                                    x
                                ))
                            } else {
                                None
                            }
                        } else {
                            stack.insert(x, ExType::PointerType);
                            None
                        }
                    }
                    _ => Some(format!("Cannot alias {} since it is not PointerType", y)),
                }
            }
        }
        Statement::HeapNew { x, ex1 } => {
            let p1 = printExpression(ex1.clone());
            match typeCheckExp(ex1, stack) {
                Ok(ExType::NatType) => {
                    if stack.contains_key(&x) {
                        if stack.get(&x).unwrap().clone() != ExType::PointerType {
                            Some(format!(
                                "Variable {} already exists and is not PointerType",
                                x
                            ))
                        } else {
                            None
                        }
                    } else {
                        stack.insert(x, ExType::PointerType);
                        None
                    }
                }
                Ok(_) => Some(format!("Expression: {} should be of NatType", p1)),
                Err(e) => Some(e),
            }
        }
        Statement::Sequence { st1, st2 } => match typeCheck(*st1, stack) {
            Some(e) => Some(e),
            None => typeCheck(*st2, stack),
        },
        Statement::IfThenElse {
            condition,
            then_branch,
            else_branch,
        } => {
            let p1 = printExpression(condition.clone());
            match typeCheckExp(condition, stack) {
                Ok(ExType::BoolType) => {
                    //clone the hash map
                    let mut n_map = stack.clone();
                    let check_then_b = typeCheck(*then_branch, &mut n_map);
                    let check_else_b = typeCheck(*else_branch, stack);
                    match (check_then_b, check_else_b) {
                        (None, None) => {
                            let mut check = true;
                            if !n_map.len() == stack.len() {
                                check = false;
                            }
                            for (key, value) in n_map {
                                if stack.contains_key(&key) {
                                    if !(stack.get(&key).unwrap().clone() == value) {
                                        check = false;
                                    }
                                } else {
                                    check = false;
                                }
                            }

                            if !check {
                                Some(format!(
                                    "Stack or Heap after the If Then Else are not identical"
                                ))
                            } else {
                                None
                            }
                        }
                        (Some(e), _) => Some(e),
                        (_, Some(e)) => Some(e),
                    }
                }
                Ok(_) => Some(format!("Expression: {} should be of BoolType", p1)),
                Err(e) => Some(e),
            }
        }
        Statement::Skip => None,
        Statement::While { condition, st } => {
            let p1 = printExpression(condition.clone());
            match typeCheckExp(condition, stack) {
                Ok(ExType::BoolType) => typeCheck(*st, stack),
                Ok(_) => Some(format!("Expression: {} should be of BoolType", p1)),
                Err(e) => Some(e),
            }
        }
    }
}
