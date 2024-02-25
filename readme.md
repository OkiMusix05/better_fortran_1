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
- *MAT*
  - Keyword to signal said variable is a matrix, should specify the dimension as showed in the example at *src/test.btf*
- *print()*
  - The print function, that takes in "" arguments and variables using commas
- *use*
  - Imports a library, like math, which introduces constants like pi, e, etc.
# Version Log
- **v0.0**: Initial commit, the transpiler can take a path to a .BTF file and accurately translate the main program, comments, variable declarations including matrices, and prints.
- **v0.01**, **v0.011**: Repository initializer
- **v0.012**: Added the math module to be able to use pi, e, and G for now