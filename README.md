# chicken_esolang

An  interpreter for the esolang [chicken](https://esolangs.org/wiki/Chicken), built in Rust. The esolang wiki is fairly terse, so I had to consult [this Python implementation](https://github.com/kosayoda/chickenpy) for a lot of the details.

## Running

```
cargo run -- -f filename [-i user_input] [--debug]
```

## Implementaiton choices

The first item in the stack is supposed to be initialized with a reference to the stack itself. I could implement this by making the stack a `RefCell<Vec<Token>>`, adding another value to the Token enum, i.e. `StackReference(RefCell<Vec<Token<'a>>>)`, and making use of `borrow_mut()` whenever I wanted to push or pop the stack. However, this struck me as excessive given that it's only used in the `LOAD` instruction, not to mention that it forfeits compile-time enforcement of borrow-checking. I instead opted to use the first value of the stack as the instruction register, and to just use a conditional to load the stack or the user input given a 0 or 1, respectively, rather than using them as the actual index at which to retrieve the `LOAD` source.