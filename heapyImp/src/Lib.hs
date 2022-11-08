module Lib where

-- m, n ∈ N natural numbers
-- x ∈ Id variables
-- l ∈ Loc locations
-- b ::= true | false boolean constants

-- e ::= x read from store
--     | !x read from heap
--     | N | b constants
--     | e + e addition
--     | ¬ e boolean negation
--     | e ∧ e boolean conjunction
--     | e ≤ e comparison

data BoolCons = TrueCons | FalseCons

ppBoolCons :: BoolCons -> String
ppBoolCons TrueCons = "true"
ppBoolCons FalseCons = "false"

instance Show BoolCons where show = ppBoolCons

data Expression =
    StackVar String
    | HeapVar String
    | NatConstant Int
    | BoolConstant BoolCons
    | Add Expression Expression
    | Negation Expression
    | Conjunction Expression Expression
    | Comparision Expression Expression

ppExp :: Expression -> String

ppExp ex = case ex of
        StackVar x        -> x ++ " "
        HeapVar x         -> "!" ++ x
        NatConstant n     -> show n
        BoolConstant b    -> show b
        Add e1 e2         -> "(" ++ ppExp e1 ++ ") + (" ++ ppExp e2 ++ ")"
        Negation b        -> "not (" ++ ppExp b ++ ")"
        Conjunction e1 e2 -> "(" ++ ppExp e1 ++ ") and (" ++ ppExp e2 ++ ")"
        Comparision e1 e2 -> "(" ++ ppExp e1 ++ ") <= (" ++ ppExp e2 ++ ")"

instance Show Expression where show = ppExp

-- s ::= x := e assignment to store
--     | !x := e update heap location
--     | x = y alias heap location
--     | x := new(e) put new value on heap
--     | s; s sequencing
--     | if e then s else s conditional
--     | skip no - op
--     | while e do s loop

data Statement =
    StackAssignment String Expression
    | HeapUpdate String Expression
    | HeapAlias String String
    | HeapNew String Expression
    | Sequence Statement Statement
    | IfThenElse Expression Statement Statement
    | Skip
    | While Expression Statement


ppStat :: Statement -> String
ppStat s = case s of
        StackAssignment st ex ->  st ++ " = " ++ show ex
        HeapUpdate st ex -> "!" ++  st ++ " = " ++ show ex
        HeapAlias st1 st2 -> st1 ++ " = " ++ st2
        HeapNew st ex -> st ++ " = new(" ++ show ex ++ ")"
        Sequence s1 s2 -> ppStat s1 ++ "; " ++ ppStat s2
        IfThenElse cond thenB elseB -> "if (" ++ show cond ++ ") then " ++ ppStat thenB ++ " else " ++ ppStat elseB
        Skip -> "skip"
        While ex1 st -> "while (" ++ show ex1 ++ ")" ++ " do " ++ ppStat st

instance Show Statement where show = ppStat


data ExType =
    NatType | BoolType

