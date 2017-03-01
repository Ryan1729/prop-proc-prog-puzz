#prop_proc_prog_puzz

prop_proc_prog_puzz <=> Property-based Procedurally generated Programming Puzzles

##Overview

This Cargo project produces an executable that produces Cargo projects which contain property based tests and function templates for you to fill in.

THat is, first a Rust function is procedurally generated, as well as a function template with the same signature.

Then a cargo project is produced containg a property-based test which calls the generated function.

Then the puzzle is to fill in the function template such that it passes the test. That is, you need to produce a function that produces (enough of) the same outputs on random inputs that (according to the test) they are the same function; without seeing what the generated function is! No peeking!

## Requirements

Cargo must be installed and on your PATH

`mkdir -p` must be able to make a directory.

## Usage

Build the release version:

```
cargo build --release
```

Make sure the executable (in `./target/release/`) is somewhere you don't mind folders being created.
Where it is right now probably suffices.

Run the executable:

```
./prop_proc_prog_puzz
```

Move into the created folder, (replace 42 with the number your folder used.)

```
cd puzzle_42/
```

Run the test. (The long compile is only on the first time.)

```
cargo test
```

If all goes well the output shuld contain the following:

```
running 1 test
test prop ... FAILED

failures:

---- prop stdout ----
        thread 'prop' panicked at 'not yet implemented', src/lib.rs:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
thread 'prop' panicked at 'not yet implemented', src/lib.rs:5
thread 'prop' panicked at '[quickcheck] TEST FAILED (runtime error). Arguments: (0)
Error: "not yet implemented"', /home/ryan/.cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-0.3.2/src/tester.rs:118


failures:
    prop

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

error: test failed
```

Open `src/lib.rs` in your editor of choice.

That file should contain (among other things) a function template something like the following:

```rust
#[allow(unused_variables)]
pub fn fill_me_in (input : u8) -> Colour {
    unimplemented!();
}
```

Replace `unimplemented!();` with a valid body for the function. In out example `Blue` would work.

```rust
#[allow(unused_variables)]
pub fn fill_me_in (input : u8) -> Colour {
    Blue
}
```

If you didn't get lucky with your first try you will see an error message containg something like the following :

```
Error: "given an input of (0) we expeceted a result of (Green) but
we recieved (Blue) instead!"
```

In this case it looks like we should change the `Blue` we entered in earlier to `Green`.
Make a change to your function based on your error message.

You might get multiple eroor messages like so (along with some line numbers etc.):

```
given an input of (15) we expeceted a result of (Blue) but we recieved (Green) instead!
given an input of (8) we expeceted a result of (Blue) but we recieved (Green) instead!
given an input of (4) we expeceted a result of (Blue) but we recieved (Green) instead!
given an input of (3) we expeceted a result of (Blue) but we recieved (Green) instead!
```

Make a change to the function that satisfies these errors and all previous ones.

Eventually you should be able to write an expression that passes the test, like this one in our example :

```
if input > 0 { Blue } else { Green }
```

When your function passes the test the output will be like this:

```
running 1 test
test prop ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests puzzle_42

```

If you like, you can run `prop_proc_prog_puzz` again and get another puzzle!

## TODO list

  * Improve generation
    * more variety
      * boolean expressions
      * loops
      * recursive functions
    * make functions that simplify down to a constant much less likely
  * Optionally encode generated function in base64 to prevent causual cheating.
  * Allow choosing the type of the generated function
    * include Random option
    * "campaign mode" with increasing difficulty?
