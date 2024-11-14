# Stupid Shader Language
A "shader-language" that always compiles, for better or worse.

It also does not run on the GPU... yet...

Check out the [live demo](https://balderholst.github.io/SSL/)!

## Idea
An SSL program, is an expression that maps an (x, y) coordinate to an RGB value (like a shader). A simple program may look like this
```text
{x, y, 0.0}
```
When this expression is evaluated on every pixel of an image, it creates a nice gradient where the red channel changes with `x` and the green channel changes with `y`. The blue channel is constant. `x` and `y` are in the space from -1 to 1.

The output of an expression can be *any* floating point number. To find the RGB value of each pixel, the expression output it clamped between 0 and 1 with a function defined in [the evaluator](./src/evaluator.rs). I do not guarantee that this function stays the same in the future.

Check out the [examples](./examples/) to get an idea of the language syntax and operations.

### Any Program is a Valid Program!
You know how JavaScript generates a semicolon if you forget to add one after a statement? SSL does the same, but for every situation where it encounters an invalid token. It then uses the value of the token it found to deterministtically pick a valid parsing path that. This means that you do not have to know the SSL language to generate a nice image, just give it some text and out pops an image.


## Running the Code
SSL runs in locally as well as in the browser with web assembly. Running locally is faster, as it utilizes multi threading when rendering the image.

To run SSL locally, clone the repository and compile with cargo like so:
```bash
git clone https://github.com/BalderHolst/ssl ssl
cd ssl
cargo build --release
cp target/release/ssl ssl
./ssl
```

## Inspiration
This project was inspired by the ["Implementing Scientific Paper in C"](https://www.youtube.com/watch?v=3D_h2RE0o0E) livestream by [tsoding](https://github.com/tsoding). Which was about trying to implement "Hash Visualization" as described in [this scientific paper](http://users.ece.cmu.edu/~adrian/projects/validation/validation.pdf). I have taken some liberties and this project is not a replication of the paper.

