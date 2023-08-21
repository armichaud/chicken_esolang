# chicken_esolang

[WIP] An  interpreter for the esolang, [chicken](https://esolangs.org/wiki/Chicken), built in Rust.

As described in the wiki documentation, the stack technically begins with the load register set to the address of the stack itself. Implementing this would require the use of RefCells, which seems unnecessary when I can just treat the load register like a vector index.