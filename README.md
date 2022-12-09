# CS456 Project (Idea 1 from project ideas given in class)

## Quick intro
The inference rules are in `CS_456_Project_Inference_Rules.pdf` in the outer-most directory. The code for the tests and the type-checker are inside the directory `heapy_imp`. `heapy_imp/src/imp.rs` contains the type-system related code (pretty-printer, type-checker, definitions) and `heapy_imp/src/main.rs` contains the tests that we wrote.

## How to run
Clone the directory, `cd` into the folder `heapy_imp` and hit `cargo run`.

## Few notes about the project
* The tests include a lot of unit tests, testing specific structure of the code, and a couple of big examples (such as Fibonacci) to show how the program works.
* In the inference rules, the rule `S-newptrnat` creates an instance of `ptrnat`. We have tried to ensure that there can be no null pointers.
