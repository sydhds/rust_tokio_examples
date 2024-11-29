# Using rust-gdb

## Debugging binary

* Build the bin (not in release mode)
    * cargo build

### First debug

* rust-gdb ../target/debug/rustgdb_1
    * Add a breakpoint on our program main function
        * break rustgdb_1::main
    * Run the program
        * run
    * Display current source:
        * layout src
    * Display variables:
        * print hero1

### More advanced debug

* Display current variables:
    * info locals
* Display current function arguments:
    * info args
* Breakpoint on a function
    * break rustgdb_1::create_wizard
* Breakpoint on a struct method
    * break rustgdb_1::Character::take_damages
    * Print self value:
        * print *self
        * print self.first_name
        * print self.is_dead()

TODO:

* break on a generic struct?
* break on struct method with condition?

## Debugging example

* Build all examples
    * cargo build --examples

* Debug:
    * rust-gdb ../target/debug/examples/hero_try_1
        * break hero_try_1::main

## Debugging unit test

* Build unit tests:
    * cargo test --no-run
    * Please check the console output with info like:
        * ` Executable unittests src/main.rs (../target/debug/deps/rustgdb_1-a8611619000a999)`

* Debug
    * rust-gdb ../target/debug/deps/rustgdb_1-a8611619000a999
    * b rustgdb_1::tests::attack_1

## Debugging with rust-lldb

TODO