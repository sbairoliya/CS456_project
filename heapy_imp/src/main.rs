use std::collections::HashMap;

use crate::imp::typeCheck;
use crate::imp::typeCheckExp;
use crate::imp::ExType;
use crate::imp::Expression;
use crate::imp::Statement;
mod imp;

fn runNegationExamples(
    empty_hashmap: HashMap<String, ExType>,
    hashmapWithHeapRead: HashMap<String, ExType>,
) {
    let validNegate_bool = Expression::Negation {
        ex1: Box::new(Expression::BoolConstant { b: true }),
    };

    let validNegate_recursive = Expression::Negation {
        ex1: Box::new(Expression::Negation {
            ex1: Box::new(Expression::BoolConstant { b: true }),
        }),
    };

    let validNegate_comparison = Expression::Negation {
        ex1: Box::new(Expression::Comparision {
            ex1: Box::new(Expression::NatConstant { n: 5 }),
            ex2: Box::new(Expression::NatConstant { n: 5 }),
        }),
    };

    let invalidNegate_heapRead = Expression::Negation {
        ex1: Box::new(Expression::HeapRead { x: "h".to_string() }),
    };

    let invalidNegate_nat = Expression::Negation {
        ex1: Box::new(Expression::NatConstant { n: 4 }),
    };

    assert!(typeCheckExp(validNegate_recursive, &mut empty_hashmap.clone()).is_ok());
    assert!(typeCheckExp(validNegate_bool, &mut empty_hashmap.clone()).is_ok());
    assert!(typeCheckExp(validNegate_comparison, &mut empty_hashmap.clone()).is_ok());

    assert!(typeCheckExp(invalidNegate_nat, &mut empty_hashmap.clone()).is_err());
    assert!(typeCheckExp(invalidNegate_heapRead, &mut hashmapWithHeapRead.clone()).is_err());
}

fn runConjunctionExamples(
    empty_hashmap: HashMap<String, ExType>,
    hashmapWithHeapRead: HashMap<String, ExType>,
) {
    let validConj_boolExplicit = Expression::Conjunction {
        ex1: Box::new(Expression::BoolConstant { b: true }),
        ex2: Box::new(Expression::BoolConstant { b: false }),
    };

    let validConj_boolWithNegation = Expression::Conjunction {
        ex1: Box::new(Expression::BoolConstant { b: true }),
        ex2: Box::new(Expression::Negation {
            ex1: Box::new(Expression::BoolConstant { b: true }),
        }),
    };

    let invalidConj_nat = Expression::Conjunction {
        ex1: Box::new(Expression::BoolConstant { b: true }),
        ex2: Box::new(Expression::NatConstant { n: 5535 }),
    };

    let invalidConj_heapVarRead = Expression::Conjunction {
        ex1: Box::new(Expression::BoolConstant { b: true }),
        ex2: Box::new(Expression::HeapRead { x: "h".to_string() }),
    };

    assert!(typeCheckExp(validConj_boolExplicit, &mut empty_hashmap.clone()).is_ok());
    assert!(typeCheckExp(validConj_boolWithNegation, &mut empty_hashmap.clone()).is_ok());

    assert!(typeCheckExp(invalidConj_nat, &mut empty_hashmap.clone()).is_err());
    assert!(typeCheckExp(invalidConj_heapVarRead, &mut hashmapWithHeapRead.clone()).is_err());
}

fn runAddExamples(
    empty_hashmap: HashMap<String, ExType>,
    hashmapWithHeapRead: HashMap<String, ExType>,
) {
    let validAdd_nats = Expression::Add {
        ex1: Box::new(Expression::NatConstant { n: 2 }),
        ex2: Box::new(Expression::Add {
            ex1: Box::new(Expression::NatConstant { n: 49 }),
            ex2: Box::new(Expression::NatConstant { n: 23 }),
        }),
    };

    let validAdd_heapVar = Expression::Add {
        ex1: Box::new(Expression::NatConstant { n: 2 }),
        ex2: Box::new(Expression::HeapRead { x: "h".to_string() }),
    };

    let invalidAdd_bools = Expression::Add {
        ex1: Box::new(Expression::NatConstant { n: 2 }),
        ex2: Box::new(Expression::Negation {
            ex1: Box::new(Expression::BoolConstant { b: true }),
        }),
    };

    assert!(typeCheckExp(validAdd_nats, &mut empty_hashmap.clone()).is_ok());
    assert!(typeCheckExp(validAdd_heapVar, &mut hashmapWithHeapRead.clone()).is_ok());

    assert!(typeCheckExp(invalidAdd_bools, &mut empty_hashmap.clone()).is_err());
}

fn runHeapReadExamples(
    empty_hashmap: HashMap<String, ExType>,
    hashmapWithHeapRead: HashMap<String, ExType>,
    hashmapWithBoolStackVar: HashMap<String, ExType>,
    hashmapWithNatStackVar: HashMap<String, ExType>,
) {
    let validRead_heapPtr = Expression::HeapRead { x: "h".to_string() };

    let invalidRead_nat = Expression::HeapRead { x: "n".to_string() };
    let invalidRead_bool = Expression::HeapRead { x: "b".to_string() };

    assert!(typeCheckExp(validRead_heapPtr, &mut hashmapWithHeapRead.clone()).is_ok());

    assert!(typeCheckExp(invalidRead_nat, &mut hashmapWithNatStackVar.clone()).is_err());
    assert!(typeCheckExp(invalidRead_bool, &mut hashmapWithBoolStackVar.clone()).is_err());
}

fn runComparisonExamples(
    empty_hashmap: HashMap<String, ExType>,
    hashmapWithHeapRead: HashMap<String, ExType>,
) {
    let validAdd_nats = Expression::Add {
        ex1: Box::new(Expression::NatConstant { n: 2 }),
        ex2: Box::new(Expression::Add {
            ex1: Box::new(Expression::NatConstant { n: 49 }),
            ex2: Box::new(Expression::NatConstant { n: 23 }),
        }),
    };
    let validAdd_heapVar = Expression::Add {
        ex1: Box::new(Expression::NatConstant { n: 2 }),
        ex2: Box::new(Expression::HeapRead { x: "h".to_string() }),
    };

    let invalidAdd_bools = Expression::Add {
        ex1: Box::new(Expression::NatConstant { n: 2 }),
        ex2: Box::new(Expression::Negation {
            ex1: Box::new(Expression::BoolConstant { b: true }),
        }),
    };

    assert!(typeCheckExp(validAdd_nats, &mut empty_hashmap.clone()).is_ok());
    assert!(typeCheckExp(validAdd_heapVar, &mut hashmapWithHeapRead.clone()).is_ok());

    assert!(typeCheckExp(invalidAdd_bools, &mut empty_hashmap.clone()).is_err());
}

fn runHeapNewExamples(
    empty_hashmap: HashMap<String, ExType>,
    hashmapWithHeapRead: HashMap<String, ExType>,
    hashmapWithBoolStackVar: HashMap<String, ExType>,
    hashmapWithNatStackVar: HashMap<String, ExType>,
) {
    let validPtr_natConstant = Statement::HeapNew {
        x: "x".to_string(),
        ex1: Expression::NatConstant { n: 5 },
    };
    let validPtr_natAdd = Statement::HeapNew {
        x: "x".to_string(),
        ex1: Expression::Add {
            ex1: Box::new(Expression::NatConstant { n: 4 }),
            ex2: Box::new(Expression::NatConstant { n: 21 }),
        },
    };
    let validPtr_readHeapReadiable = Statement::HeapNew {
        x: "x".to_string(),
        ex1: Expression::HeapRead { x: "h".to_string() },
    };
    let invalidPtr_comparison = Statement::HeapNew {
        x: "x".to_string(),
        ex1: Expression::Comparision {
            ex1: Box::new(Expression::NatConstant { n: 4 }),
            ex2: Box::new(Expression::NatConstant { n: 9 }),
        },
    };
    let invalidPtr_boolConstant = Statement::HeapNew {
        x: "x".to_string(),
        ex1: Expression::BoolConstant { b: true },
    };
    assert!(typeCheck(invalidPtr_comparison, &mut empty_hashmap.clone()).is_some());
    assert!(typeCheck(invalidPtr_boolConstant, &mut empty_hashmap.clone()).is_some());

    assert!(typeCheck(validPtr_natConstant, &mut empty_hashmap.clone()).is_none());
    assert!(typeCheck(validPtr_natAdd, &mut empty_hashmap.clone()).is_none());

    let hashmap2 = &mut hashmapWithHeapRead.clone();
    hashmap2.insert("x".to_string(), ExType::PointerType);
    assert!(typeCheck(validPtr_readHeapReadiable.clone(), hashmap2).is_none());
    assert!(hashmap2.contains_key(&"x".to_string()));
    assert_eq!(hashmap2.get(&"x".to_string()), Some(&ExType::PointerType));

    assert!(typeCheck(
        validPtr_readHeapReadiable.clone(),
        &mut hashmapWithHeapRead.clone()
    )
    .is_none());
    assert!(hashmap2.contains_key(&"x".to_string()));
    assert_eq!(hashmap2.get(&"x".to_string()), Some(&ExType::PointerType));
}

fn runAssignmentStoreExamples(
    empty_hashmap: HashMap<String, ExType>,
    hashmapWithHeapRead: HashMap<String, ExType>,
    hashmapWithBoolStackVar: HashMap<String, ExType>,
    hashmapWithNatStackVar: HashMap<String, ExType>,
) {
    let validAssignment_nat = Statement::StackAssignment {
        x: "x".to_string(),
        ex1: Expression::NatConstant { n: 5 },
    };

    let validAssignment_boolConjunction = Statement::StackAssignment {
        x: "x".to_string(),
        ex1: Expression::Conjunction {
            ex1: Box::new(Expression::BoolConstant { b: true }),
            ex2: Box::new(Expression::Conjunction {
                ex1: Box::new(Expression::BoolConstant { b: true }),
                ex2: Box::new(Expression::BoolConstant { b: false }),
            }),
        },
    };

    let validAssignment_boolNegation = Statement::StackAssignment {
        x: "x".to_string(),
        ex1: Expression::Negation {
            ex1: Box::new(Expression::BoolConstant { b: true }),
        },
    };

    let invalidAssignment_pointer = Statement::StackAssignment {
        x: "x".to_string(),
        ex1: Expression::StackVar { x: "h".to_string() },
    };

    assert!(typeCheck(validAssignment_nat, &mut empty_hashmap.clone()).is_none());
    assert!(typeCheck(validAssignment_boolConjunction, &mut empty_hashmap.clone()).is_none());
    assert!(typeCheck(validAssignment_boolNegation, &mut empty_hashmap.clone()).is_none());

    assert!(typeCheck(invalidAssignment_pointer, &mut hashmapWithHeapRead.clone()).is_some());
}

fn runFibonacci(empty_hashmap: HashMap<String, ExType>) {
    let fibonnaci_tree = Statement::Sequence {
        st1: Box::new(Statement::StackAssignment {
            x: "fibonacci_index".to_string(),
            ex1: Expression::NatConstant { n: 50 },
        }),
        st2: Box::new(Statement::Sequence {
            st1: Box::new(Statement::StackAssignment {
                x: "fibonacci_number".to_string(),
                ex1: Expression::NatConstant { n: 1 },
            }),
            st2: Box::new(Statement::Sequence {
                st1: Box::new(Statement::StackAssignment {
                    x: "prev".to_string(),
                    ex1: Expression::NatConstant { n: 0 },
                }),
                st2: Box::new(Statement::Sequence {
                    st1: Box::new(Statement::StackAssignment {
                        x: "curr".to_string(),
                        ex1: Expression::NatConstant { n: 1 },
                    }),
                    st2: Box::new(Statement::Sequence {
                        st1: Box::new(Statement::StackAssignment {
                            x: "counter".to_string(),
                            ex1: Expression::NatConstant { n: 2 },
                        }),
                        st2: Box::new(Statement::While {
                            condition: Expression::Comparision {
                                ex1: Box::new(Expression::StackVar {
                                    x: "counter".to_string(),
                                }),
                                ex2: Box::new(Expression::StackVar {
                                    x: "fibonacci_index".to_string(),
                                }),
                            },
                            st: Box::new(Statement::Sequence {
                                st1: Box::new(Statement::StackAssignment {
                                    x: "counter".to_string(),
                                    ex1: Expression::Add {
                                        ex1: Box::new(Expression::StackVar {
                                            x: "counter".to_string(),
                                        }),
                                        ex2: Box::new(Expression::NatConstant { n: 1 }),
                                    },
                                }),
                                st2: Box::new(Statement::Sequence {
                                    st1: Box::new(Statement::StackAssignment {
                                        x: "fibonacci_number".to_string(),
                                        ex1: Expression::Add {
                                            ex1: Box::new(Expression::StackVar {
                                                x: "curr".to_string(),
                                            }),
                                            ex2: Box::new(Expression::StackVar {
                                                x: "prev".to_string(),
                                            }),
                                        },
                                    }),
                                    st2: Box::new(Statement::Sequence {
                                        st1: Box::new(Statement::StackAssignment {
                                            x: "prev".to_string(),
                                            ex1: Expression::StackVar {
                                                x: "curr".to_string(),
                                            },
                                        }),
                                        st2: Box::new(Statement::StackAssignment {
                                            x: "curr".to_string(),
                                            ex1: Expression::StackVar {
                                                x: "fibonacci_number".to_string(),
                                            },
                                        }),
                                    }),
                                }),
                            }),
                        }),
                    }),
                }),
            }),
        }),
    };
    print!("{}", imp::printStatement(fibonnaci_tree.clone()));
    assert!(typeCheck(fibonnaci_tree, &mut empty_hashmap.clone()).is_none());
}

fn main() {
    let empty_hashmap = HashMap::new();

    let mut hashmapWithHeapRead = HashMap::new();
    hashmapWithHeapRead.insert("h".to_string(), ExType::PointerType);

    let mut hashmapWithBoolStackVar = HashMap::new();
    hashmapWithBoolStackVar.insert("b".to_string(), ExType::BoolType);

    let mut hashmapWithNatStackVar = HashMap::new();
    hashmapWithNatStackVar.insert("n".to_string(), ExType::NatType);

    runHeapNewExamples(
        empty_hashmap.clone(),
        hashmapWithHeapRead.clone(),
        hashmapWithBoolStackVar.clone(),
        hashmapWithNatStackVar.clone(),
    );
    runAssignmentStoreExamples(
        empty_hashmap.clone(),
        hashmapWithHeapRead.clone(),
        hashmapWithBoolStackVar.clone(),
        hashmapWithNatStackVar.clone(),
    );
    runAddExamples(empty_hashmap.clone(), hashmapWithHeapRead.clone());
    runNegationExamples(empty_hashmap.clone(), hashmapWithHeapRead.clone());
    runConjunctionExamples(empty_hashmap.clone(), hashmapWithHeapRead.clone());
    runComparisonExamples(empty_hashmap.clone(), hashmapWithHeapRead.clone());
    runHeapReadExamples(
        empty_hashmap.clone(),
        hashmapWithHeapRead.clone(),
        hashmapWithBoolStackVar.clone(),
        hashmapWithNatStackVar.clone(),
    );

    runFibonacci(empty_hashmap.clone());
}
