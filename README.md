![Bitscalc](site/src/assets/logo.svg)

***

[Bitscalc](https://bitscalc.com) is a binary integer calculator for quickly evaluating programming expressions. It supports arithmetic, logical, and bitwise operators with C-like precedence. Step-by-step results are shown in decimal, hexadecimal, and binary. It is designed to be hosted as a standalone site. It is created using Svelte with a Rust WebAssembly package for business logic. 

You can check out the live version of the site at [bitscalc.com!](https://bitscalc.com) 

# Dev Manual

Bitscalc is primarily built using on [Rust](https://rust-lang.org/learn/get-started/), [Node.js](https://nodejs.org/en/download), and [Wasm-Pack](https://drager.github.io/wasm-pack/installer/). It also uses [Make](https://www.gnu.org/software/make/) as a build tool. Make sure you have the needed dependencies. Make and Wasm-Pack work best in Unix environments. 

### Start developing the site
```sh
make dev
```

### Build the site
```sh
make build
```

### Run the library tests
```
make test
```


### Try out the Rust CLI example
```sh
make cli
```

