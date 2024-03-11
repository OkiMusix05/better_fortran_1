# Why Better FORTRAN?
I'm a Physics student who **loves** FORTRAN's speed but **hates** FORTRAN's syntax.
Therefore, I created this FORTRAN transpile that translates my custom scripting language (.BTF) to FORTRAN, so I can get the benefits and speed of FORTRAN while coding in a programming language I like more.
As I am a physics student and not a Computer Science student, and making a transpiler is really, really hard, this project literally just uses REGEX to translate lines, but it gets the job done wonderfully
# Keywords
- *fn main() {}*
  - The main function under which everything should go, except functions and subroutines
- *let*
  - Creates a variable
- *const*
  - Creates a constant
- *:*
  - Specifies the type of variables, constants or arguments, as follows:
  - | BTF | FORTRAN               |
    |-----|-----------------------|
    | int | integer               |
    | f4  | real                  |
    | f8  | double precision real |
- *<m,n>*
  - Added at the end of the type to make it a matrix. Can use ':' to allocate that memory later, or do <m> to make that a vector, as showed in the example at *src/test.btf*
- *print()*
  - The print function, that takes in "" arguments and variables using commas
- *use*
  - Imports a library, like math, which introduces constants like pi, e, etc.
- *if () {*
  - An if statement, inside the parenthesis goes the condition
- *} elif () {*
  - An else if statement. The elif can go after or under the *}*
- *} else {*
  - No condition is needed. Same special treatment as the *elif*
- *while () {}*
  - do while loop in fortran. Takes a condition
- *for(i=1, n, 1) {}*
  - do loop in fortran. First parameter is the variable and its initial value, second one is the top value, and third one is the increment/decrement. Should be separated by a comma. 
# Version Log
- **v0.0**: Initial commit, the transpiler can take a path to a .BTF file and accurately translate the main program, comments, variable declarations including matrices, and prints.
- **v0.01**, **v0.011**: Repository initializer
- **v0.012**: Added the math module to be able to use pi, e, and G for now
- **v0.02**: Added if/elif/else, while/for loops and the ability to nest them. Also made a new syntax for matrices and vectors that I like more. Now the program keeps track of matrices (unsafe rust).
Support for allocation is partially added, since the basis is there but function calling is not part of the program yet.
- **v0.021**: Added the *str* and *bool* types, as wel as logical operators. Also added breaks for loops