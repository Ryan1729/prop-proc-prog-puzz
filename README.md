prop_proc_prog_puzz <=> Property-based Procedurally generated Programming Puzzles

#Plan

give the generator a deck of operations, and have it produce a function. Optionally encode in base64 to prevent causual cheating.

Then a cargo project is produced containg a property-based test which calls the generated function.

Then the challenge is to fill in the function template such that it passes the test. That is, you need to produce a function that produces (enough of) the same outputs on random inputs that (according to the test) they are the same function; without seeing what the generated function is!

# TODO list

actual geneeration rather than string literals
  -> generate using a context-free grammar

bool to 3 value enum
  -> make sure all 9 funtions are generated

3 value enum to bool
  -> make sure all 8 funtions are generated

u8 to bool
  -> far to big to confirm this one
