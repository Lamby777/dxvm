# `dxvm`

> Oh, what's this?

Don't worry about it.

> No really, what is it?

No, really, don't worry about it.

# Come on, just tell me!

Alright, this project is essentially just me trying to screw around with making a virtual machine using low-level programming. `dxvm` has its own instruction set and all of that sort of stuff, but I kinda have no idea what I'm doing currently.

...but yeah, this is mostly just a learning project for fun. Don't go write your own video games in DxVM machine code and then become disappointed that I stopped taking the project seriously. The JVM already exists; go learn Java or something. ;-;

---

If I DO happen to go somewhere with this project, though, I'll make sure to add calls for r/w on files, cross-platform dialog boxes, and more.

# Contributions

Don't. Just don't. It's that simple. Go contribute to something more important. 😆

# Building
Just compile the code like any other Rust project

```sh
git clone https://github.com/Lamby777/dxvm.git
cd dxvm
cargo build --release
```

# Usage
Keep in mind that as of me writing this, `dxvm` does not yet support actually interpreting code. Currently, the only way I'm running DxVM machine code is through slice literals in `tests.rs`. Certified TDD moment, am I right?

HOWEVER, when it DOES eventually get support (aka when I stop being lazy and/or when I get less homework), this will work:

```sh
dxvm example-bytecode.dxr
```
