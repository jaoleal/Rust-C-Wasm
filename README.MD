# Rust C Wasm

This is some experimental tests with Rust Wasm and its power

The point is to use C functions on web apps trough Rust and [Yew](https://yew.rs/)...
but would be awesome to just use htmx or html+js to use the C functions as a static web page. See [Todo](#Todo)

## what i've done so far ?
So i casted the math.h to rust in a separate crate and casted that crate on my wep app.
The proof of work of this is calling PoW C function in math.h directly on my [main.rs](/src/main.rs) that is wrapped with yew.

yew is here just to help. Its not necessary, but it keep me away from yavascript.

## I wanna se it working.
Nice! I aprecciate your interest.

```bash
# Right now i am using nix package manager on my nixos to manage my dependencies...
$ nix develop
# After loading your deps, you can go in main.rs
# and change the PoW function to try it on another number.
$ cargo build

$ trunk serve

```

### Todo
[ ]- Try Static Web Page that call a C Function. 
[ ]- Try other C Libraries such as [rmath.h](https://svn.r-project.org/R/trunk/src/include/Rmath.h0.in) that is more complex than math.h.
[ ]- Find Something useful for this. XD