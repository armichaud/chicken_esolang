# chicken_esolang

An interpreter for the esolang [chicken](https://esolangs.org/wiki/Chicken), built in Rust. As you'll read below, [the web archive of the original interpreter](https://web.archive.org/web/20180420010853/http://torso.me/chicken) was critical for understanding some of the implementation details.

WIP as of Aug 27, 2023

Major TODOS (all pertain to enabling backwards compatibility):
- Figure out why 99 chickens prepends an extra `&#32;chicken&#115;&#10;` to all numeric inputs greater than 1.
- Figure out why there's no space between `1` and `chicken`.
- Get Deadfish working

## Running

```
cargo run -- -f filename [--debug] [--backwards_compatible] [-i user_input] 
```

## The Javascript Interpreter or: How I Learned to Stop Worrying and Chicken Chicken Chicken

tl;dr if you want to run this against any example other than hello world, use the --backwards_compatible flag.

With a language like this, it feels silly to complain about obtuse design – that's largely the point. However, it's important to understand just how much the example programs are dependent on quirks of Javascript, in which the original interpreter is written, in order to run properly. Making sense of the original interpreter nearly drove me mad, but it was necessary for debugging behavior that seemed inexplicable. See the `js_interpreter/demystified.js.txt` file for the breakdown.

Below are the counter-intuitive behaviors of the original interpreter and how I've chosen to approach them in this reimplementation. In general, I've only implemented JS-specific behavior when an example program relies on it.

### COMPARE

Instruction 34 of the 99 chickens program will compare 0 against the value loaded from the user input register. The program relies on this at some point being true in order to JUMP to the end and build the final `no chickens`. Sane programming languages return false when comparing 0 and an empty string, but not Javascript! And so, when `backwards_compatibility` is enabled, the comparison mimics the behavior of the JS loose equality operator, and `'' == 0` evaluates to `true`.

### ADD

The addition operator in Javascript also serves as string concatenation, automatically casting integers to strings. I've chosen to emulate this universally, since the structure of the program relies on the final output being stored in a single stack token.

In theory, Javascript would also cast `NaN` and `undefined` as strings if they were on either side of an addition operation. See the entries for SUB and LOAD below for when those values become an issue and how I'm handling them. 

### SUB

In JS, a string minus a number is equal to `NaN`, which can be concatenated to other strings. 

TODO: Figure out why 99 Chickens, when user input is a string, sets register 1 to NaN after subtraction, but is still able to finish program.

### CHAR

The ASCII conversion in the JS interpreter just interpolates the char code in between `&#` and `;`, effectively building the HTML character code. This plays an important role in 99 chickens, which is explained in more detail in the section about the LOAD instruction. 

### LOAD

99 chickens and Deadfish both have instruction sequences that load from stack indices other than 0 and 1. This is a hack employed to more quickly retrieve chars from strings that are stored in overwritten instruction registers. 

Notably, in order to get the `n` at the beginning of the `no chickens`, the 11th char is loaded from the value at index 2, i.e. `&#32;chicken&#115;&#10;`. Ironically, CHAR _has_ to push HTML character codes in order for this to work – the 11th index is out of bounds in `" chickens\n"` – but the `n` itself is just prepended as is, so the output looks like `n&#111;&#32;chicken&#115;&#10;`.

If the index is greater than or equal to the length of the token, in JS it returns the special `undefined` value. Deadfish relies on this behavior in multiple places, so in backwards compatibility mode, we return `"undefined".to_string()`, but we're otherwise erroring, because this shouldn't be a thing. 

TODO: Figure out if Deadfish actually uses `undefined` as a string, or as some kind of weird falsy value.

### The Stack

The first item in the stack is supposed to be initialized with a reference to the stack itself. This sort of thing is trivial for a language like Javascript, but Rust is designed to make this sort of thing difficult. I could implement this by making the stack a `RefCell<Vec<Token>>`, adding another value to the Token enum, i.e. `StackReference(RefCell<Vec<Token<'a>>>)`, and making use of `borrow_mut()` whenever I wanted to push or pop the stack. However, this struck me as excessive given that it's only relevant in the `LOAD` instruction, not to mention that it forfeits compile-time enforcement of borrow-checking. I instead opted to use the first value of the stack as the instruction register, and just make the double-wide LOAD instruction two separate functions, `load_from_stack` and `load_from_token`.
