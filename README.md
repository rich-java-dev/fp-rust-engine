# funphysics-rust-engine
(Integrating Rust/WASM in an existing React App)

# Why Rust?
Rust is a general purpose systems programming language designed for high performance, safety, reliability, and productivity. 
Originally developed by Mozilla (2010), was meant to overcome many issues in concurrency, memory, and thread safety.

"Rust has been voted the "most loved programming language" in the Stack Overflow Developer Survey every year since 2016."

# Major Rust Concepts:

### Native Runtime
compiles to binaries vs. VM byte-code like Java/C#, doesn't need an interpretter like Python/JS)
### No Garbage Collector (Requires deeper understanding of resource life-time/scope clean-up)
### immutable by default

### Ownership / Borrowing
When passing a variable (instead of a reference to the variable) to another function, you are giving up ownership. The other function is now the owner of this variable and you can’t use it anymore
When passing references to a variable (lending it out), you can have either as many immutable borrows as you want or a single mutable borrow. Once you start borrowing mutably, there can be only one

### Borrow Checker
Compile-time checking that no 2 pieces of code (pointers or functions) have ownership/access at the same time to the same address in memory. eg: No 2 pointers can point to the same address in memory at the same time.

PROBLEM
```
fn read_value(string: String) {
    println!("{}", string);
}
pub fn main() {
    let test = String::from("Hello World!");
    read_value(test);
    println!("{}", test); // BORROW ERROR - read_value took ownership of 'test'
}
```
FIX
```
fn read_value(string: &String) {
    println!("{}", string);
}
pub fn main() {
    let test = String::from("Hello World!");
    read_value(&test); // Borrowed vs. Taking Ownership
    println!("{}", test); // No issues with borrowing
}
```

### Macros
Macros are an important aspect of Rust, providing a natural form for meta-programming, analgous to annotation/decorator patterns. Macros in Rust are highly expressive, based on their high flexibility with concise syntax by operating over AST (Abstract Syntax Tree) syntax.

Example:
```
  #[wasm_bindgen]
  pub fn set_params(params: &JsValue) {...}
```
Here we see wasm_bindgen decorating a function, which allows the was_bindgen to dynamically modify/implement/call the 'set_params' function, providing a compilation directive to export the API/functions to be transpiled into raw Web Assembly modules to be consumed by Node applications.


### Cargo
Rust package manager
### Crates
A crate is a binary or library


# Why Wasm?
Wasm (Web Assembly) was adopted in 2019 by W3 consortium as the 4th official language of the web (after HTML, CSS, JS)
Web Assembly is a binary file layout. The aim is to eliminate the need of an interpretter in the web-browser to increase performance/reduce overhead for fast applications.

# Approach

### wasm-bindgen
Facilitates high-level interactions between Wasm modules and JavaScript. 
Using this tool to allow allow easy adoption of Rust into existing web apps.

https://github.com/rustwasm/wasm-bindgen

### wasm-pack (build)
CLI for generating nodejs packages for deployment to web apps.
Compiles the Rust app into WASM, and builds TypeScript stubs, which can be called directly in nodejs/ JS Runtime.
the "stubs" are an interface which acts as a middle-ware between the JS and WASM calls.
https://rustwasm.github.io/wasm-pack/installer/

### serde 
framework for serializing and deserializing Rust data structures efficiently and generically.
https://serde.rs/

### lazy-static
A macro for declaring lazily evaluated statics.
Allowed for Singleton pattern impl for "Global State"/Params.
https://docs.rs/lazy_static/1.4.0/lazy_static/
>>>>>>> updates on rust eng
