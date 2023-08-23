### ROX: Rust Implementation of The LOX Programming Language

This repository contains the Rust implementation of the LOX programming language as described in the book
["Crafting Interpreters" by Robert Nystrom](https://craftinginterpreters.com/). 

LOX is a dynamically typed scripting language designed for learning and exploration of interpreter 
and compiler concepts. This implementation aims to faithfully follow the language specifications 
outlined in the book.

This repository consist of two crates

- [rox_script](./rox_script): The Tree-Walk Interpreter implementation of the lox language
- [rox_lang](./rox_lang): The Bytecode Virtual Machine implementation of the lox language