# rlisp #

rlisp is a simple CL-Lisp evaluator written in Rust.

## Installation ##

$ git clone https://github.com/truzenzuzex/rlisp

## Quick start ##

  rlisp provides a minimal REPL to start with CL-Lisp instructions.

  So far the following CL-Lisp instructions are implemented:

  * <em>function</em>

  * <em>lambda</em>

  * <em>funcall</em>

  * <em>block</em>

  * <em>return-from</em>

  * <em>progn</em>

  * <em>defun</em>

  * <em>defmacro</em> with comma instructions <em>","</em> and <em>",@"</em>
  * <em>backquote</em>

  * <em>quote</em>

  * <em>defparameter</em>

  * <em>list</em>

  * <em>car</em>

  * <em>cdr</em>

  * <em>cons</em>

  * <em>concatenate</em> for list and strings

  * simple math functions like: <em>+, -, *, / </em>

  * <em>eval</em>

**Some examples of REPL actions:**

`
RLisp> (defun foo (x y) (+ x y))

FOO

RLisp> (foo 1 2)

3

RLisp> (defparameter some_list '(1 2 3))

SOME_LIST

RLisp> some_list

(1 2 3)

RLisp> (defmacro bar (x y) `(list ,@some_list ,x ,y))

BAR

RLisp> (bar 4 5)

(1 2 3 4 5)
`
## Running the build ##

cargo run RUST_BACKTRACE = 1 -p rlisp

## To Do ##

RLisp was written with the intention to make myself a bit more familiar with Rust.<br> It is completely experimental and is probably (for sure!) still error prone.

Nevertheless it is somehow a "living" system and could be a playground for many<br> further development activities. There are several places where the current imple-<br>mentation can be build up or improved:

  * Revise of the Rust code in general, e.g:
    * Search for better Rust patterns...
    * Stop the actual debug noise...
    * other...

  * General design review.

  * Actually the Parser looks for key words and doesn't parse character wise.<br> This is not according to the CL-Lisp spec, see CL-Lisp <em>read</em> command.

  * Make the Lisp-Environment thread safe.

  * Improve the handling and preparation of parameters.

  * Extend the macro functionality in terms of realizing all<br>backquote combinations, e.g.: <em>",," ",,@"</em>.

  * Implement all special and other operators in rust.

  * Improve the whole environment through e.g. a load function, error function,<br>print function.

  * Extend functionality of the REPL with the help of Rust Clap.

  * Develop a compiler instead of the evaluator to follow the Lisp spirit that Lisp<br> compiles Lisp.<br> Model for that could be the CL-Lisp CLASP with CLEAVIR solution to produce<br> CLEAVIR-BIR and/or Rust-MIR as an intermediate format.

  * and so on...

## Licence ##
rlisp is licensed under the **MIT Lincense**
