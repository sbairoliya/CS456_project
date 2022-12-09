# CS456 Project (Idea 1 from project ideas given in class) - Aniket & Shivam

## Quick intro
The inference rules are in `CS_456_Project_Inference_Rules.pdf` in the outer-most directory. The code for the tests and the type-checker are inside the directory `heapy_imp`. `heapy_imp/src/imp.rs` contains the type-system related code (pretty-printer, type-checker, definitions) and `heapy_imp/src/main.rs` contains the tests that we wrote.

## How to run
Clone the directory, `cd` into the folder `heapy_imp` and hit `cargo run`.

## Few notes about the project
* The tests include a lot of unit tests, testing specific structure of the code, and a couple of big examples (such as Fibonacci) to show how the program works.
* In the inference rules, the rule `S-newptrnat` creates an instance of `ptrnat`. We have tried to ensure that there can be no null pointers.
* All variables are global. So, any variable declared within an if-else branch or a while loop will be propagated globally.
* No missing features
* The test suite is well-documented under the file specified above with reasonable names.
* Test suite results:
```Type Checking Valid Expression: 
(2) + ((49) + (23))
Current Typing Context: [  ]
Type Check Passed

Type Checking Valid Expression: 
(2) + (!h )
Current Typing Context: [ "h": PointerType,  ]
Type Check Passed

Type Checking Invalid Expression: 
(2) + (not (true))
Current Typing Context: [  ]
Type Check Failed as Expected the error message is:
Expression: 2 should be of NatType

Type Checking Valid Expression: 
not (not (true))
Current Typing Context: [  ]
Type Check Passed

Type Checking Valid Expression: 
not (true)
Current Typing Context: [  ]
Type Check Passed

Type Checking Valid Expression: 
not ((5) <= (5))
Current Typing Context: [  ]
Type Check Passed

Type Checking Invalid Expression: 
not (4)
Current Typing Context: [  ]
Type Check Failed as Expected the error message is:
Expression: 4 should be of BoolType

Type Checking Invalid Expression: 
not (!h )
Current Typing Context: [ "h": PointerType,  ]
Type Check Failed as Expected the error message is:
Expression: !h  should be of BoolType

Type Checking Valid Expression: 
(true) and (false)
Current Typing Context: [  ]
Type Check Passed

Type Checking Valid Expression: 
(true) and (not (true))
Current Typing Context: [  ]
Type Check Passed

Type Checking Invalid Expression: 
(true) and (5535)
Current Typing Context: [  ]
Type Check Failed as Expected the error message is:
Expression: 5535 should be of BoolType

Type Checking Invalid Expression: 
(true) and (!h )
Current Typing Context: [ "h": PointerType,  ]
Type Check Failed as Expected the error message is:
Expression: !h  should be of BoolType

Type Checking Valid Expression: 
(2) <= ((49) + (23))
Current Typing Context: [  ]
Type Check Passed

Type Checking Valid Expression: 
(2) <= (!h )
Current Typing Context: [ "h": PointerType,  ]
Type Check Passed

Type Checking Invalid Expression: 
(2) <= (not (true))
Current Typing Context: [  ]
Type Check Failed as Expected the error message is:
Expression: 2 should be of NatType

Type Checking Valid Expression: 
!h 
Current Typing Context: [ "h": PointerType,  ]
Type Check Passed

Type Checking Invalid Expression: 
!n 
Current Typing Context: [ "n": NatType,  ]
Type Check Failed as Expected the error message is:
variable: n is not a pointer

Type Checking Invalid Expression: 
!b 
Current Typing Context: [ "b": BoolType,  ]
Type Check Failed as Expected the error message is:
variable: b is not a pointer

Type Checking Invalid Statement: 
x = new((4) <= (9))
Current Typing Context: [  ]
Type Check Failed as Expected the error message is:
Expression: (4) <= (9) should be of NatType

Type Checking Invalid Statement: 
x = new(true)
Current Typing Context: [  ]
Type Check Failed as Expected the error message is:
Expression: true should be of NatType

Type Checking Valid Statement: 
x = new(5)
Current Typing Context: [  ]
Typing Context after type check: [ "x": PointerType,  ]
Context expected to contain: [ "x": PointerType,  ]
Context is correct
Type Check Passed

Type Checking Valid Statement: 
x = new((4) + (21))
Current Typing Context: [  ]
Typing Context after type check: [ "x": PointerType,  ]
Context expected to contain: [ "x": PointerType,  ]
Context is correct
Type Check Passed

Type Checking Valid Statement: 
x = new(!h )
Current Typing Context: [ "h": PointerType, "x": PointerType,  ]
Typing Context after type check: [ "h": PointerType, "x": PointerType,  ]
Context expected to contain: [ "h": PointerType, "x": PointerType,  ]
Context is correct
Type Check Passed

Type Checking Valid Statement: 
x = new(!h )
Current Typing Context: [ "h": PointerType,  ]
Typing Context after type check: [ "h": PointerType, "x": PointerType,  ]
Context expected to contain: [ "h": PointerType, "x": PointerType,  ]
Context is correct
Type Check Passed

Type Checking Valid Statement: 
x = 5
Current Typing Context: [  ]
Typing Context after type check: [ "x": NatType,  ]
Context expected to contain: [ "x": NatType,  ]
Context is correct
Type Check Passed

Type Checking Valid Statement: 
x = (true) and ((true) and (false))
Current Typing Context: [  ]
Typing Context after type check: [ "x": BoolType,  ]
Context expected to contain: [ "x": BoolType,  ]
Context is correct
Type Check Passed

Type Checking Valid Statement: 
x = not (true)
Current Typing Context: [  ]
Typing Context after type check: [ "x": BoolType,  ]
Context expected to contain: [ "x": BoolType,  ]
Context is correct
Type Check Passed

Type Checking Invalid Statement: 
x = h 
Current Typing Context: [ "h": PointerType,  ]
Type Check Failed as Expected the error message is:
Cannot Assign pointer type to a stack variable

Type Checking Valid Statement: 
if ((n ) <= (n )) then { n = 10 } else { n = 12 }
Current Typing Context: [ "n": NatType,  ]
Typing Context after type check: [ "n": NatType,  ]
Context expected to contain: [ "n": NatType,  ]
Context is correct
Type Check Passed

Type Checking Valid Statement: 
if ((n ) <= (n )) then { skip } else { n = 12 }
Current Typing Context: [ "n": NatType,  ]
Typing Context after type check: [ "n": NatType,  ]
Context expected to contain: [ "n": NatType,  ]
Context is correct
Type Check Passed

Type Checking Valid Statement: 
if ((n ) <= (n )) then { n = 10 } else { skip }
Current Typing Context: [ "n": NatType,  ]
Typing Context after type check: [ "n": NatType,  ]
Context expected to contain: [ "n": NatType,  ]
Context is correct
Type Check Passed

Type Checking Invalid Statement: 
if ((n ) <= (n )) then { x = 10 } else { skip }
Current Typing Context: [ "n": NatType,  ]
Type Check Failed as Expected the error message is:
Stack or Heap after the If Then Else are not identical

Type Checking Invalid Statement: 
if (10) then { n = 10 } else { skip }
Current Typing Context: [ "n": NatType,  ]
Type Check Failed as Expected the error message is:
Expression: 10 should be of BoolType

Type Checking Valid Statement: 
fibonacci_index = 50; fibonacci_number = 1; prev = 0; curr = 1; counter = 2; while ((counter ) <= (fibonacci_index )) do { counter = (counter ) + (1); fibonacci_number = (curr ) + (prev ); prev = curr ; curr = fibonacci_number }
Current Typing Context: [  ]
Typing Context after type check: [ "counter": NatType, "fibonacci_index": NatType, "fibonacci_number": NatType, "curr": NatType, "prev": NatType,  ]
Context expected to contain: [ "fibonacci_number": NatType,  ]
Context is correct
Type Check Passed
