# Stupid Shader Language
A shader-language that always compiles, for better or worse.

It also does not run on the GPU...

This project was inspired by the ["Implementing Scientific Paper in C"](https://www.youtube.com/watch?v=3D_h2RE0o0E) livestream by [tsoding](https://github.com/tsoding). Which was about trying to implement "Hash Visualization" as described in [this scientific paper](http://users.ece.cmu.edu/~adrian/projects/validation/validation.pdf). I have taken some liberties and this project is not a replication of the paper.

## Idea
An ssl program, is an expression that maps an (x, y) coordinate to an RGB value (like a shader). A simple program may look like this
```text
{x, y, 0.0}
```
This creates a nice gradient where the red channel changes with `x` and the green channel changes with `y`. The blue channel is constant. Now you may ask what about and expression like this?
```
1000
```
This evaluates to a white image. In ssl, there are no invalid programs, so this does produce an output. When an expression evaluates to a scalar value, it determines the gray scale brightness of the pixel. Another strange thing: How is `1000` within the bounds of the RGB space? Clamp all evaluated expressions into the RGB color space, the output is run through the [sigmoid function](https://en.wikipedia.org/wiki/Sigmoid_function). This has the tradeoff, that adding color is not linear, and that a color of `0.0` is actually "half on" instead of the intuitive "off".
