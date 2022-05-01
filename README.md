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

Exp := Int | Exp BinOp Exp | UnOp Exp | Let Id = Exp | Id = Exp | If Exp then Exp else Exp | { Exp ; ... ; Exp } |  
Int := -3 | -2 | -1 | 0 | 1 | 2 | 3 | ...  
BinOp := + | - | * | / | > | < | <= | >= | =  
UnOp := -  
Id := x | y | z | ...



*The grammar will be displayed here, once it is ready. The grammar is a work in progress.*

### Remaining Work:
 * Construct ~~(minimal)~~ Grammar
 * Implement ~~(minimal)~~ lexer
 * Implement ~~(minimal)~~ parser
 * Implement ~~(minimal)~~ interpreter
 * Design terrible syntax 
 * Expand Lexer, Parser, Interpreter
 * Optimization/Benchmarking work
    * Ownership / Borrowing of values
    * Efficient implementations
    * Interpreter optimization
        * Eval of zero-child, ...

## Running & Testing
The project supports testing of provided sample files. Simply put the program in the /samples directory and name it *[FILENAME]*.tsl. Then put the expected lexing/parsing/runtime file in the respective directly, and call them *[FILENAME]*.lex, .par or .int (for runtime) respectively.   
Run the interpreter with:
```
cargo run [FILENAME].tsl
```
which will invoke the main.rs file, and execute the lexing, parsing and interpretation of the program. For testing, run the command
```
cargo run [FILENAME].tsl --test/-t -[MODE]
```
where *[MODE]* is either *lex*, *par* or *int*, for the respective test.  
**EXAMPLE**: 
```
cargo run arith.tsl --test -lex 
cargo run arith.tsl --test -par
cargo run arith.tsl --test -int
```
will first run the lexing, and compare the result with the file in */expected/lexing/arith.lex* and then likewise for parsing with */expected/parsing/arith.par* and finally for interpretation with */expected/runtime/arith.int*  

One can also choose to run all tests of a given type. For now, this still requires specifying a specific file, but every file will be ran. 
**EXAMPLE**
```
cargo run arith-tsl --test -lex --all
cargo run arith-tsl --test -par --all
cargo run arith-tsl --test -int --all
```
will run all samples files and compare them against their expected in the */lexing* directory, and likewise for parsing and interpretation.