<div align="center">
<h3>Rust RotoZoom to Wasm</h3>
<img src="https://img.shields.io/badge/zero-copy_Wasm↔Canvas-green.svg">
<img src="https://img.shields.io/badge/zero-dependency-yellow.svg">
<img src="https://img.shields.io/badge/no-std-orange.svg">
<img src="https://img.shields.io/badge/arm-ready-cyan.svg">
<a href="https://github.com/karl-rousseau/rotozoom-wasm/blob/main/LICENSE"><img src="http://img.shields.io/badge/license-MIT-blue.svg"></a>
</div>

---
## Preamble

`rotozom-wasm` is a cross-platform reimplementation of the [SDL rotozoom example](https://github.com/tuupola/sdl2_effects/blob/master/rotozoom.c) in [RUST](http://www.rust-lang.org) language.
While this program have been originally implemented in C language for old framebuffer video cards, nowadays inside current browsers, we can achieve the same effect within a [Canvas 2D](https://caniuse.com/?search=%3Ccanvas%3E) object using:
- A bitmap pointer as shared memory (i.e. **without** copying data)
- An exposed native function in WASM (**without** using [Wasm-Bindgen](https://rustwasm.github.io/wasm-bindgen/) dependency library)
- Pure array of bytes as a framebuffer (i.e. **without** using the extra Rust [std](https://doc.rust-lang.org/std/) library)
- Avoid using Rust Wasm [wee_alloc](https://github.com/rustwasm/wee_alloc) as the global allocator (to save extra KB in Wasm)
- Isolated bytecode (inside browsers) being cross-platform (i.e. **ARM ready** even with [SVE](https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/webassembly-bitmask-operations))

### Build

Building this project using Cargo is easy because the process is the same as for every other Rust program:

```shell
cargo build --release --target wasm32-unknown-unknown
```

PS: If above default target is not already installed, you can add the given `wasm32 target` in your toolchain:
> rustup target add wasm32-unknown-unknown

### Roadmap

- [x] Raw pointer for **zero-copy** feature (optimal transfert between JS ⇔ RUST)
- [ ] Faster Mathematical functions using **lookup tables** (with fixed-width integers?)
- [ ] Split **parallel pixels** computational rotation calculus using [Wasm SIMD extension](https://chromestatus.com/feature/6533147810332672)
- [ ] Separate core loop **algorithm into pieces** being handled by [Wasm Thread extension](https://chromestatus.com/feature/5724132452859904)
- [ ] Implement a defined **texture interpolation** to improve the rendering (when zoomed)

### Dev and testing

You need to serve the generated files with an HTTP server using such command:

```shell
python -m http.server 1234
```

### Reverse engineering

A lot of emphasis has been put on code simplicity (and for size) when writing this rotozoom effect.
Therefore if you want to see what has been generated inside the WASM bytecode, just run:

```shell
cargo install wasm-tools
wasm-tools print target/wasm32-unknown-unknown/release/rotozoom_wasm_rs.wasm
```

### Algorithm

The whole effect is based on 2D transformation used in Computer Graphics and specifically on this rotation matrix calculus in cartesian coordinate system using this formula:<br/>
<img src="https://latex.codecogs.com/svg.image?\begin{bmatrix}X_{new}\\Y_{new}\end{bmatrix}=\begin{bmatrix}cos(\Theta)&-sin(\Theta)\\sin(\Theta)&cos(\Theta)\\\end{bmatrix}x\begin{bmatrix}X_{old}\\Y_{old}\end{bmatrix}"><br/>
For the algorithm code, previous equation can be written and simplified into discrete math such as:<br/>
<img src="https://latex.codecogs.com/svg.image?X_{new}=X_{old}*cos(\Theta)-Y_{old}*sin(\Theta)"><br/>
<img src="https://latex.codecogs.com/svg.image?Y_{new}=X_{old}*sin(\Theta)&plus;Y_{old}*cos(\Theta)">

## Reference

This project is a wink and a tribute to the best demoscene group that I've seen, called [Future Crew](https://en.wikipedia.org/wiki/Future_Crew) which did release [2nd Reality](https://en.wikipedia.org/wiki/Second_Reality) in 1993 at the Assembly demo party. It contains this original rotozoom effect and texture (thank you [Pixel](https://en.wikipedia.org/wiki/Misko_Iho)). By the way, their original source code has been [released in C + ASM](https://github.com/mtuomi/SecondReality/blob/master/GLENZ/ZOOMER.C) and Fabien Sanglard did a great analysis [here](https://fabiensanglard.net/second_reality/).
<div align="center">
  <a href="https://www.youtube.com/watch?v=iw17c70uJes&t=278"><img src="https://img.youtube.com/vi/GL5vb3D-8_I/0.jpg" alt="2nd Reality (1993) by Future Crew"></a><br/>
  YouTube video emulated on a <a href="https://en.wikipedia.org/wiki/I486">486DX</a> PC running under MS/DOS™
</div>

## License

`rotozom-wasm` is licensed under the MIT License - see the `LICENSE` file for more details.
