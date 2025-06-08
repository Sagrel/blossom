# What is this repo?

One of my dreams is to create my own programming language, and I have made some attempts before (you can find them in my profile), but they were failures (for my standard)

In this repo I plan to fully specify the blossom language 🌸! I want to have a complete specification and set of tests before writing a single line of code.
My hope is that having a target goal and a set of tests that work as progress trackers and a TODO list, I will be able to make steady progress on it this time 🤞

# Feature of the language

In this section I will list, in no particular order, all the feature that I would like the language to support
- Ownership and borrowing semantics (like rust)
- Linear types as descrived [here](https://without.boats/blog/ownership/)
- Pattern matching 
	- Exhaustibvity and non redundancy checks [like implemented in rust](https://doc.rust-lang.org/beta/nightly-rustc/rustc_pattern_analysis/usefulness/index.html)
	- Everything is a pattern mentality
- Some support for [dependent types](https://stackoverflow.com/questions/9338709/what-is-dependent-typing)
- UFCS
- Function overloading
- Generics
	- Some constraint system for generics (contracts in C++)
- Compile time meta programing
	- Being able to get information about types (kind of like zig)
	- Being able to create new types (kind of like zig)
	- Being able to take tokens and generate code (rust macros) or is a string enough (so just like zig?)
- Yield and I gues async
- Variant types and records with consistent syntax
- Everything is a literal mentality
- Everything is annonymous mentality
- Structural and nominal typing
- Everything is an expresion mentality
- Consistent and clean syntax
- Clear naming (int -> Int, uint -> Nat, float -> Real, String -> Text, Vec -> List, && -> and, || -> or)
- Enforced capitalization
- Inmutable by default



# Features of the tooling
We all know that a language is only as usefull as it's tooling, so let's define my goals for perfect tooling

- Fast, parallel compiler
- Incremental compilation by default
- Hot reloading
- Compiles to native code
- LSP from day 1
	- Just installing the LSP should be enough, it can compile your code!
- Great debugger experience thanks to [DAP](https://microsoft.github.io/debug-adapter-protocol/)
- Auto formating with no options
- Package manager
- Profilers