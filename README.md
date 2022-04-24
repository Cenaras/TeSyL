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

Exp := Int; | Exp OP Exp; | Let Exp = Exp; | If Exp then Exp else Exp; | Exp ; Exp | 
Int := 1 | 2 | 3 | ...  
OP := + | - | * | / | > | < | =

*The grammar will be displayed here, once it is ready. The grammar is a work in progress.*

### Remaining Work:
 * Construct (minimal) Grammar
 * Implement minimal lexer
 * Implement minimal parser
 * Implement minimal interpreter
 * Expand Lexer, Parser, Interpreter
 * Optimization/Benchmarking work
    * Ownership / Borrowing of values
    * Efficient implementations
    * Interpreter optimization
        * Eval of zero-child, ...
