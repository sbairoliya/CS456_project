use std::collections::HashMap;

use crate::imp::printExpression;
use crate::imp::printStatement;
use crate::imp::printType;
use crate::imp::typeCheck;
use crate::imp::typeCheckExp;
use crate::imp::ExType;
use crate::imp::Expression;
use crate::imp::Statement;

mod imp;

fn printHashMap(hashmap_to_pass: HashMap<String, ExType>) {
    print!("[ ");
    for (key, value) in &hashmap_to_pass {
        print!("\"{}\": {}, ", key, printType(value.clone()));
    }
    print!(" ]\n")
}

fn checkHashMap(hash_original: HashMap<String, ExType>, hash_expected: HashMap<String, ExType>) {
    if hash_expected.is_empty() {
        return;
    } else {
        print!("Context expected to contain: ")
    }
    print!("[ ");
    for (key, value) in &hash_expected {
        print!("\"{}\": {}, ", key, printType(value.clone()));
        assert!(hash_original.contains_key(key));
        assert_eq!(hash_original.get(key), Some(value));
    }
    print!(" ]\n");
    print!("Context is correct\n");
}

fn typeCheckExpressionPass(syntax_tree: Expression, hashmap_to_pass: HashMap<String, ExType>) {
    print!(
        "Type Checking Valid Expression: \n{}\n",
        printExpression(syntax_tree.clone())
    );
    print!("Current Typing Context: ");
    printHashMap(hashmap_to_pass.clone());
    let hashmap_to_pass = &mut hashmap_to_pass.clone();
    assert!(typeCheckExp(syntax_tree, hashmap_to_pass).is_ok());
    print!("Type Check Passed\n\n")
}

fn typeCheckStatementPass(
    syntax_tree: Statement,
    hashmap_to_pass: HashMap<String, ExType>,
    hash_expected: HashMap<String, ExType>,
) {
    print!(
        "Type Checking Valid Statement: \n{}\n",
        printStatement(syntax_tree.clone())
    );
    print!("Current Typing Context: ");
    printHashMap(hashmap_to_pass.clone());
    let hashmap_to_pass = &mut hashmap_to_pass.clone();
    assert!(typeCheck(syntax_tree, hashmap_to_pass).is_none());
    print!("Typing Context after type check: ");
    printHashMap(hashmap_to_pass.clone());
    checkHashMap(hashmap_to_pass.clone(), hash_expected);
    print!("Type Check Passed\n\n")
}

fn typeCheckExpressionFail(syntax_tree: Expression, hashmap_to_pass: HashMap<String, ExType>) {
    print!(
        "Type Checking Invalid Expression: \n{}\n",
        printExpression(syntax_tree.clone())
    );
    print!("Current Typing Context: ");
    printHashMap(hashmap_to_pass.clone());
    let hashmap_to_pass = &mut hashmap_to_pass.clone();
    let tp = typeCheckExp(syntax_tree, hashmap_to_pass);
    assert!(tp.is_err());
    print!("Type Check Failed as Expected the error message is:\n");
    match tp {
        Ok(_) => (),
        Err(x) => print!("{}\n\n", x),
    }
}

fn typeCheckStatementFail(syntax_tree: Statement, hashmap_to_pass: HashMap<String, ExType>) {
    print!(
        "Type Checking Invalid Statement: \n{}\n",
        printStatement(syntax_tree.clone())
    );
    print!("Current Typing Context: ");
    printHashMap(hashmap_to_pass.clone());
    let hashmap_to_pass = &mut hashmap_to_pass.clone();
    let tp = typeCheck(syntax_tree, hashmap_to_pass);
    assert!(tp.is_some());
    print!("Type Check Failed as Expected the error message is:\n");
    match tp {
        Some(x) => print!("{}\n\n", x),
        None => (),
    }
}

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

    // assert!(typeCheckExp(validNegate_recursive, &mut empty_hashmap.clone()).is_ok());
    typeCheckExpressionPass(validNegate_recursive, empty_hashmap.clone());
    // assert!(typeCheckExp(validNegate_bool, &mut empty_hashmap.clone()).is_ok());
    typeCheckExpressionPass(validNegate_bool, empty_hashmap.clone());
    // assert!(typeCheckExp(validNegate_comparison, &mut empty_hashmap.clone()).is_ok());
    typeCheckExpressionPass(validNegate_comparison, empty_hashmap.clone());
    // assert!(typeCheckExp(invalidNegate_nat, &mut empty_hashmap.clone()).is_err());
    typeCheckExpressionFail(invalidNegate_nat, empty_hashmap.clone());
    // assert!(typeCheckExp(invalidNegate_heapRead, &mut hashmapWithHeapRead.clone()).is_err());
    typeCheckExpressionFail(invalidNegate_heapRead, hashmapWithHeapRead.clone());
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

    // assert!(typeCheckExp(validConj_boolExplicit, &mut empty_hashmap.clone()).is_ok());
    typeCheckExpressionPass(validConj_boolExplicit, empty_hashmap.clone());
    // assert!(typeCheckExp(validConj_boolWithNegation, &mut empty_hashmap.clone()).is_ok());
    typeCheckExpressionPass(validConj_boolWithNegation, empty_hashmap.clone());
    // assert!(typeCheckExp(invalidConj_nat, &mut empty_hashmap.clone()).is_err());
    typeCheckExpressionFail(invalidConj_nat, empty_hashmap.clone());
    // assert!(typeCheckExp(invalidConj_heapVarRead, &mut hashmapWithHeapRead.clone()).is_err());
    typeCheckExpressionFail(invalidConj_heapVarRead, hashmapWithHeapRead.clone());
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

    // assert!(typeCheckExp(validAdd_nats, &mut empty_hashmap.clone()).is_ok());
    typeCheckExpressionPass(validAdd_nats, empty_hashmap.clone());
    // assert!(typeCheckExp(validAdd_heapVar, &mut hashmapWithHeapRead.clone()).is_ok());
    typeCheckExpressionPass(validAdd_heapVar, hashmapWithHeapRead.clone());
    // assert!(typeCheckExp(invalidAdd_bools, &mut empty_hashmap.clone()).is_err());
    typeCheckExpressionFail(invalidAdd_bools, empty_hashmap.clone());
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

    // assert!(typeCheckExp(validRead_heapPtr, &mut hashmapWithHeapRead.clone()).is_ok());
    typeCheckExpressionPass(validRead_heapPtr, hashmapWithHeapRead.clone());
    // assert!(typeCheckExp(invalidRead_nat, &mut hashmapWithNatStackVar.clone()).is_err());
    typeCheckExpressionFail(invalidRead_nat, hashmapWithNatStackVar.clone());
    // assert!(typeCheckExp(invalidRead_bool, &mut hashmapWithBoolStackVar.clone()).is_err());
    typeCheckExpressionFail(invalidRead_bool, hashmapWithBoolStackVar.clone());
}

fn runComparisonExamples(
    empty_hashmap: HashMap<String, ExType>,
    hashmapWithHeapRead: HashMap<String, ExType>,
) {
    let validAdd_nats = Expression::Comparision {
        ex1: Box::new(Expression::NatConstant { n: 2 }),
        ex2: Box::new(Expression::Add {
            ex1: Box::new(Expression::NatConstant { n: 49 }),
            ex2: Box::new(Expression::NatConstant { n: 23 }),
        }),
    };
    let validAdd_heapVar = Expression::Comparision {
        ex1: Box::new(Expression::NatConstant { n: 2 }),
        ex2: Box::new(Expression::HeapRead { x: "h".to_string() }),
    };

    let invalidAdd_bools = Expression::Comparision {
        ex1: Box::new(Expression::NatConstant { n: 2 }),
        ex2: Box::new(Expression::Negation {
            ex1: Box::new(Expression::BoolConstant { b: true }),
        }),
    };

    // assert!(typeCheckExp(validAdd_nats, &mut empty_hashmap.clone()).is_ok());
    typeCheckExpressionPass(validAdd_nats, empty_hashmap.clone());
    // assert!(typeCheckExp(validAdd_heapVar, &mut hashmapWithHeapRead.clone()).is_ok());
    typeCheckExpressionPass(validAdd_heapVar, hashmapWithHeapRead.clone());
    // assert!(typeCheckExp(invalidAdd_bools, &mut empty_hashmap.clone()).is_err());
    typeCheckExpressionFail(invalidAdd_bools, empty_hashmap.clone());
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
    // assert!(typeCheck(invalidPtr_comparison, &mut empty_hashmap.clone()).is_some());
    typeCheckStatementFail(invalidPtr_comparison, empty_hashmap.clone());
    // assert!(typeCheck(invalidPtr_boolConstant, &mut empty_hashmap.clone()).is_some());
    typeCheckStatementFail(invalidPtr_boolConstant, empty_hashmap.clone());
    let hashmap1 = &mut empty_hashmap.clone();
    hashmap1.insert("x".to_string(), ExType::PointerType);
    // assert!(typeCheck(validPtr_natConstant, &mut empty_hashmap.clone()).is_none());
    typeCheckStatementPass(
        validPtr_natConstant,
        empty_hashmap.clone(),
        hashmap1.clone(),
    );
    // assert!(typeCheck(validPtr_natAdd, &mut empty_hashmap.clone()).is_none());
    typeCheckStatementPass(validPtr_natAdd, empty_hashmap.clone(), hashmap1.clone());

    let hashmap2 = &mut hashmapWithHeapRead.clone();
    hashmap2.insert("x".to_string(), ExType::PointerType);
    // assert!(typeCheck(validPtr_readHeapReadiable.clone(), hashmap2).is_none());
    typeCheckStatementPass(
        validPtr_readHeapReadiable.clone(),
        hashmap2.clone(),
        hashmap2.clone(),
    );
    // assert!(hashmap2.contains_key(&"x".to_string()));
    // assert_eq!(hashmap2.get(&"x".to_string()), Some(&ExType::PointerType));
    // assert!(typeCheck(
    //     validPtr_readHeapReadiable.clone(),
    //     &mut hashmapWithHeapRead.clone()
    // )
    // .is_none());
    typeCheckStatementPass(
        validPtr_readHeapReadiable.clone(),
        hashmapWithHeapRead.clone(),
        hashmap2.clone(),
    );

    // assert!(hashmap2.contains_key(&"x".to_string()));
    // assert_eq!(hashmap2.get(&"x".to_string()), Some(&ExType::PointerType));
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

    let hashmap1 = &mut empty_hashmap.clone();
    hashmap1.insert("x".to_string(), ExType::NatType);
    // assert!(typeCheck(validAssignment_nat, &mut empty_hashmap.clone()).is_none());
    typeCheckStatementPass(
        validAssignment_nat.clone(),
        empty_hashmap.clone(),
        hashmap1.clone(),
    );
    let hashmap2 = &mut empty_hashmap.clone();
    hashmap2.insert("x".to_string(), ExType::BoolType);
    // assert!(typeCheck(validAssignment_boolConjunction, &mut empty_hashmap.clone()).is_none());
    typeCheckStatementPass(
        validAssignment_boolConjunction.clone(),
        empty_hashmap.clone(),
        hashmap2.clone(),
    );
    // assert!(typeCheck(validAssignment_boolNegation, &mut empty_hashmap.clone()).is_none());
    typeCheckStatementPass(
        validAssignment_boolNegation.clone(),
        empty_hashmap.clone(),
        hashmap2.clone(),
    );

    // assert!(typeCheck(invalidAssignment_pointer, &mut hashmapWithHeapRead.clone()).is_some());
    typeCheckStatementFail(invalidAssignment_pointer, hashmapWithHeapRead.clone());
}

fn testIfThenElse(
    empty_hashmap: HashMap<String, ExType>,
    hashmapWithHeapRead: HashMap<String, ExType>,
    hashmapWithBoolStackVar: HashMap<String, ExType>,
    hashmapWithNatStackVar: HashMap<String, ExType>,
) {
    let if_else_st_basic_valid = Statement::IfThenElse {
        condition: Expression::Comparision {
            ex1: Box::new(Expression::StackVar { x: "n".to_string() }),
            ex2: Box::new(Expression::StackVar { x: "n".to_string() }),
        },
        then_branch: Box::new(Statement::StackAssignment {
            x: "n".to_string(),
            ex1: Expression::NatConstant { n: 10 },
        }),
        else_branch: Box::new(Statement::StackAssignment {
            x: "n".to_string(),
            ex1: Expression::NatConstant { n: 12 },
        }),
    };

    typeCheckStatementPass(
        if_else_st_basic_valid,
        hashmapWithNatStackVar.clone(),
        hashmapWithNatStackVar.clone(),
    );

    let if_else_skip_if_valid = Statement::IfThenElse {
        condition: Expression::Comparision {
            ex1: Box::new(Expression::StackVar { x: "n".to_string() }),
            ex2: Box::new(Expression::StackVar { x: "n".to_string() }),
        },
        then_branch: Box::new(Statement::Skip),
        else_branch: Box::new(Statement::StackAssignment {
            x: "n".to_string(),
            ex1: Expression::NatConstant { n: 12 },
        }),
    };

    typeCheckStatementPass(
        if_else_skip_if_valid,
        hashmapWithNatStackVar.clone(),
        hashmapWithNatStackVar.clone(),
    );

    let if_else_skip_else_valid = Statement::IfThenElse {
        condition: Expression::Comparision {
            ex1: Box::new(Expression::StackVar { x: "n".to_string() }),
            ex2: Box::new(Expression::StackVar { x: "n".to_string() }),
        },
        then_branch: Box::new(Statement::StackAssignment {
            x: "n".to_string(),
            ex1: Expression::NatConstant { n: 10 },
        }),
        else_branch: Box::new(Statement::Skip),
    };

    typeCheckStatementPass(
        if_else_skip_else_valid,
        hashmapWithNatStackVar.clone(),
        hashmapWithNatStackVar.clone(),
    );

    let if_else_skip_else_invalid = Statement::IfThenElse {
        condition: Expression::Comparision {
            ex1: Box::new(Expression::StackVar { x: "n".to_string() }),
            ex2: Box::new(Expression::StackVar { x: "n".to_string() }),
        },
        then_branch: Box::new(Statement::StackAssignment {
            x: "x".to_string(),
            ex1: Expression::NatConstant { n: 10 },
        }),
        else_branch: Box::new(Statement::Skip),
    };

    typeCheckStatementFail(
        if_else_skip_else_invalid,
        hashmapWithNatStackVar.clone(),
    );

    let if_else_invalid = Statement::IfThenElse {
        condition: Expression::NatConstant { n: 10 },
        then_branch: Box::new(Statement::StackAssignment {
            x: "n".to_string(),
            ex1: Expression::NatConstant { n: 10 },
        }),
        else_branch: Box::new(Statement::Skip),
    };

    typeCheckStatementFail(
        if_else_invalid,
        hashmapWithNatStackVar.clone(),
    );

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

    let hashmap1 = &mut empty_hashmap.clone();
    hashmap1.insert("fibonacci_number".to_string(), ExType::NatType);
    typeCheckStatementPass(
        fibonnaci_tree.clone(),
        empty_hashmap.clone(),
        hashmap1.clone(),
    );
}

fn main() {
    let empty_hashmap = HashMap::new();

    let mut hashmapWithHeapRead = HashMap::new();
    hashmapWithHeapRead.insert("h".to_string(), ExType::PointerType);

    let mut hashmapWithBoolStackVar = HashMap::new();
    hashmapWithBoolStackVar.insert("b".to_string(), ExType::BoolType);

    let mut hashmapWithNatStackVar = HashMap::new();
    hashmapWithNatStackVar.insert("n".to_string(), ExType::NatType);

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

    testIfThenElse(
        empty_hashmap.clone(),
        hashmapWithHeapRead.clone(),
        hashmapWithBoolStackVar.clone(),
        hashmapWithNatStackVar.clone(),
    );

    runFibonacci(empty_hashmap.clone());
}
