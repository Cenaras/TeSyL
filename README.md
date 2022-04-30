# TeSyL
## **Te**rrible **Sy**ntax **L**anguage

*TeSyL* is a project concerning creating a interpreted language in *Rust*, with a terribly conveluted syntax. The project is intended as a learning experience to gain a fundemental understanding of the Rust programming Language, as well as to explore the design challenges of implementing a programming language.

The project consists of several parts, and may be updated even further as the projects is developed: 
* Lexing of .tsl program files
* Parsing the lexed tokens to build the AST
* Interpreting the provided AST
* Optimizations
* ...  

## Grammar
The grammar of TeSyL follows an functional style where everything is an expression, and thus the entire program is simply an expression.

Program := Exp

Exp := Int | Exp BinOp Exp | UnOp Exp | Let Exp = Exp | If Exp then Exp else Exp | ( Exp ; ... ; Exp ) |  
Int := 1 | 2 | 3 | ...  
BinOp := + | - | * | / | > | < | =  
UnOp := - 



*The grammar will be displayed here, once it is ready. The grammar is a work in progress.*

### Remaining Work:
 * Construct (minimal) Grammar
 * Implement (minimal) lexer
 * Implement (minimal) parser
 * Implement (minimal) interpreter
 * Design terrible syntax 
 * Expand Lexer, Parser, Interpreter
 * Optimization/Benchmarking work
    * Ownership / Borrowing of values
    * Efficient implementations
    * Interpreter optimization
        * Eval of zero-child, ...

## Running & Testing
The project supports testing of provided sample files. Simply put the program in the /samples directory and name it *[FILENAME]*.tsl. Then put the expected lexing/parsing file in the respective directly, and call them *[FILENAME]*.lex or .par respectively.   
Run the interpreter with:
```
cargo run [FILENAME].tsl
```
which will invoke the main.rs file, and execute the lexing, parsing and eventually the interpretation of the program. For testing, run the command
```
cargo run [FILENAME].tsl --test/-t -[MODE]
```
where *[MODE]* is either *-lex* og *-par*, for the respective test.  
**EXAMPLE**: 
```
cargo run arith.tsl -lex 
cargo run arith.tsl -par
```
will first run the lexing, and compare the result with the file in */expected/lexing/arith.lex* and then likewise for parsing with */expected/parsing/arith.par*