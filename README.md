# chicken_esolang

An interpreter for the esolang [chicken](https://esolangs.org/wiki/Chicken), built in Rust. As you'll read below, [the web archive of the original interpreter](https://web.archive.org/web/20180420010853/http://torso.me/chicken) was critical for understanding some of the implementation details.

## CLI Usage 

```
cargo run -- -f filename [--debug] [--backwards_compatible] [-i optional_input] 
```

## Crate Usage 

```
use chicken_esolang::Chicken;

let code = "chicken";
let input = "";
let debug = false;
let backwards_compatible = false;

let mut chicken = Chicken::new(code, input, debug, backwards_compatible);
chicken.run()
```

## The Javascript Interpreter or: How I Learned to Stop Worrying and Chicken Chicken Chicken

_tl;dr The 99 Bananas and DeadFish programs require the --backwards_compatible flag to work the way they do when run through the original interpreter._

With an esolang like this, it feels silly to complain about obtuse design – that's largely the point. However, it's important to understand just how much the example programs are dependent on quirks of Javascript, in which the original interpreter is written, in order to run properly. Making sense of the original interpreter nearly drove me mad, but it was necessary for debugging behavior that seemed inexplicable. See the `js_interpreter/demystified.js.txt` file for the breakdown.

The beauty of Rust is that it forces developers to confront potentially erratic program behavior through strict typing (among other rules and guarantees). In that spirit, if you want to write programs in chicken – and, to be clear, you shouldn't, but if you insist on it – this interpreter will, by default, throw an error in cases where the original interpreter would just do something weird. Backwards compatibility mode is designed specifically to emulate the behavior of the original interpreter in the interest of runnning the legacy examples. 

Below are the counter-intuitive behaviors of the original interpreter and how I've chosen to handle them. In general, I've only implemented JS-specific behavior when an example program relies on it. 

### COMPARE

Instruction 34 of the 99 chickens program will compare 0 against the value loaded from the user input register. The program relies on this at some point being true in order to JUMP to the end and build the final `no chickens` (for certain inputs). Sane programming languages return false when comparing 0 and an empty string, but not Javascript! And so, when `backwards_compatibility` is enabled, the comparison mimics the behavior of the JS loose equality operator, and `'' == 0` evaluates to `true`.

### ADD

The addition operator in Javascript also serves as string concatenation, automatically casting integers to strings. I've chosen to emulate this universally, since the structure of the program relies on the final output being stored in a single stack token.

In theory, Javascript would also cast `NaN` and `undefined` as strings if they were on either side of an addition operation. See the entries for SUB and LOAD below for when those values become an issue and how I'm handling them. 

### SUB

If you give 99 chickens a string as input, it will return the following pattern: `OriginalInputString&#32;chicken&#115;&#10;1&#32;chicken&#10;n&#111;&#32;chicken&#115;&#10;`, i.e. `OriginalInputString chickens\n1 chicken\nno chickens\n`. What's happening is this: On two occasions, the program will try to decrement the input and then make a jump conditional on the result (the JUMPs are instructions 36 and 59). In JS, a string minus a number is equal to `NaN`, which is falsy. Therefore, the results of both SUBs fail the condition for the forward jump that is supposed to account for when the input is 1, as well as that of the backwards jump that's supposed to create a loop until the count has been decremented to 1.

In order to replicate this in backwards compatibility mode, I've coded a special version of SUB that returns `Token::Chars("NaN".to_string())` when the operands mismatch. COMPARE then checks for `"NaN"` as a special case.

### CHAR

The ASCII conversion in the JS interpreter just interpolates the char code in between `&#` and `;`, effectively building the HTML character code. This plays an important role in 99 chickens, which is explained in more detail in the section about the LOAD instruction. 

### LOAD

99 chickens and Deadfish both have instruction sequences that load from stack indices other than 0 and 1. This is a hack employed to more quickly retrieve chars from strings that are stored in overwritten instruction registers. 

Notably, in order to get the `n` at the beginning of the `no chickens`, the 11th char is loaded from the value at index 2, i.e. `&#32;chicken&#115;&#10;`. Ironically, CHAR _has_ to push HTML character codes in order for this to work – the 11th index is out of bounds in `" chickens\n"` – but the `n` itself is just prepended as is, so the output looks like `n&#111;&#32;chicken&#115;&#10;`.

If the index is greater than or equal to the length of the token, Javascript returns the special `undefined` value. Deadfish relies on this behavior in multiple places, so in backwards compatibility mode, we return `"undefined".to_string()`.

TODO: Figure out if Deadfish actually uses `undefined` as a string, or as some kind of weird falsy value.

### The Stack

The first item in the stack is supposed to be initialized with a reference to the stack itself. This is trivial for a language like Javascript, but Rust is designed to make this sort of thing difficult. I could implement this by making the stack a `RefCell<Vec<Token>>`, adding another value to the Token enum, i.e. `StackReference(RefCell<Vec<Token<'a>>>)`, and making use of `borrow_mut()` whenever I wanted to push or pop the stack. However, this struck me as excessive given that it's only relevant in the LOAD instruction, not to mention that it forfeits compile-time enforcement of borrow-checking. I instead opted to use the first value of the stack as the instruction register, and just make the double-wide LOAD instruction two separate functions, `load_from_stack` and `load_from_token`.
