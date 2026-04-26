---
title: Transitioning to rust-gpu
---

# Introduction

As mentioned in the previous chapter, I want to work more extensively on more complex topics and ideas.
However, I was heavily since all my calculations were done on the CPU.
I did not feel like extending using [Nannou](https://nannou.cc/) and its [wgpu](https://wgpu.rs/) API, therefore I decided to go with a cleaner option.
There exists a library, originally developed by [Embark](https://www.embark-studios.com/) (<3), called [rust-gpu](https://rust-gpu.github.io/).
This library allows us to write shader code directly using rust and its range of amazing systems.
In this section I will cover how I was able to transition to using shaders to render my simulation more efficiently, as well as re-creating the electric field by applying Coulomb's Law.

# Shaders

What even are shaders? Well, to understand what shaders truly are and how we use them, lets first look at how the GPU renders our data.
Any model or shape, be it 3D or 2D is made up of fragments.
Each of these tiny fragments is basically a triangle, consisting of 3 vertices arranged in a counter clockwise manner to form that surface.
These fragments are then rasterized to form pixels which are rendered onto your screen.

<div class="svg-container">
  <img src="./assets/fragment.svg" class="invert-on-light" alt="Field Diagram" />
</div>

We can easily manipulate these fragments and vertices to produce complex shapes and colors using shaders. A shader in principle is just code that is run on the GPU.
There are mainly 3 types of shaders:

- **Vertex Shader**: Responsible for drawing or manipulating the individual vertices.
- **Fragment Shader**: Responsible for coloring in and assigning a color to an individual fragment.
- **Compute Shader**: Splits up complex computations into workgroups and works on them in parallel.

Shaders work differently from normal processes and calculations done on the CPU.
For example, shaders are not able to having sequential instructions that are executed once, like draw a line from point $A$ to point $B$.
Instead, the shader runs the same code for _every_ pixel, meaning all functions and all calculations have to be generalised to work on any individual element.
In this specific case, by using vectors and algebra, we can work out the shortest distance from any pixel to a line segment, and then use that to decide if we have to display something.

## Coordinate System

When using Nannou, it was really easy to work with its coordinate system, since there was only 1 single global method of defining coordinates,
where a scale was chosen centered around the origin. Here, there are multiple different coordinate systems, such as:

- **Normal Device Coordinates (NDC)**: What most GPU's understand nativly, $(0,0)$ being the middle of screen, and range for $x$ & $y$ spanning $[-1.0, 1.0]$, while $z$ $[0.0, 1.0]$.
- **Clip Space**: The vertex shader will output this type of coordinate, being a `vec4<f32>(x,y,z,w)`. Where $w$ is used to give perspective to objects, like depth. Usually this system gets converted directly to NDC.
- **Screen Space**: Most of the time this uses real pixel values to denote physical dimensions on your screen, $(0, 0)$ being top-left corner, commonly used in fragment shaders. Has also an additional $0.5$ pixel displacement to account for the center of each pixel.
- **UV Mapping**: Used for mapping textures onto geometry, centered in top-left corner, having a range of $[0.0,1.0]$. Where $U$ is increasing rightwards and $V$ decreases downwards.

In summary, inside the vertex shader we will be getting our **Clip Space** coordinates, doing calculations on them and outputting an **NDC**.
Then, inside our fragment shader we will recieve **Screen Space**, which gets mapped to a color.

## Signed Distance Fields

The previously mentioned case is a common example of a Signed Distance Field (SDF). An SDF works by calculating the distance from any point on the screen to an object, be it a line, a rectangle or a circle.
This technique also extends into 3 dimensions, however this whole blog will mainly focus on working in 2D. Now, we will look at how a few common SDF's are derived, calculated and applied in my recreation.
