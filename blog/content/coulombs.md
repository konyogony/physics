---
order: 3
title: Coulombs Law
---

# Introduction

Now that we have succesfully converted to using `rust-gpu` and shaders for our simulations, lets look at new examples and ideas that we could not do before.
In this section we will look at the simulation and application of Coulomb's Law on electrostatic charges.

# Electrostatics and the Coulombs Law

In contrast to normal electromagnetism, the study of electrostatics involves charges, electric and magnetic fieds that dont alternate over time,
hence the suffix statics. This means that the 4 Maxwell's equations simplfy to:

$$
\begin{aligned}
\vec{\nabla} \cdot \vec{E} &= \frac{\rho}{\epsilon_0} \\
\vec{\nabla} \cdot \vec{B} &= 0 \\
\vec{\nabla} \times \vec{E} &= 0 \\
\vec{\nabla} \times \vec{B} &= \frac{\vec{j}}{\epsilon_0}
\end{aligned}
$$

What this means in practice is that it is much easier to compute and deal with charges that are not moving.
Let us focus on the Coulombs Law now. The Coulomb law talks about the force on exerted on two charges, and is equal to the following expression.

$$
\vec{F_1} = \frac{1}{4\pi \epsilon_0} \, \frac{q_1 \, q_2}{r^2_{12}} \, \hat{e_{12}} = -\vec{F_2}
$$

Where $\hat{e_{12}}$ represents the unit vector from $q_1$ to $q_2$. An electric field is defined as the force per unit charge, therefore if we take $q_1$ as the reference, the electric field becomes:

$$
\vec{E} = \frac{1}{4\pi \epsilon_0} \, \frac{q_2}{r^2_{12}} \, \hat{e_{12}}
$$

This electric field can also be generalised for containing multiple charges, where we simply iterate over every charge.

$$
\vec{E} = \frac{1}{4\pi \epsilon_0} \, \sum_j \frac{q_j}{r^2_{1j}} \, \hat{e_{1j}}
$$

However, we can define the electric field in terms of a scalar value, the electric potential. This is usually prefered since you will only have to compute a single
scalar value instead of multiple separate directions. The electric potential is defined as

$$
\phi = \frac{1}{4\pi \epsilon_0} \, \sum_j \frac{q_j}{r_j}
$$

and the negative gradient of $\phi$ relates directly to the electric field.

$$
\vec{E} = - \vec{\nabla} \phi
$$

# Project

There is another way of calculating $\phi$ which involves electric density at every point, however this will force us to loop through every single pixel for every pixel we check,
which raises the complexity of the algorithm up to $O(N^2)$. Therefore, first of all we create a new buffer which hold all the charges currently present in the system. This buffer can
then be modified to dynamically add or remove charges. Afterwards, a compute shader is called which uses this buffer of charges to initialise data inside the potential buffer.
However, we need to represent the whole two dimensional grid, why and how are we not using a texture instead?

Well, textures are generaly really useful and are highly efficient because most modern GPUs have built in cache modules for handling textures, as well textures being able to provide direct UV coordinates to work with,
they are usually prefered. For this use case although, we have a chain of multiple compute shaders, hence it will be a pain using `rust-gpu`s `Image!()` macros and handle the read-write permissions across 3 different layouts.
Therefore, we will resort back to buffers. To represent a two dimensional plane in a single buffer, we will convert the current pixel coordinates into the index by basically getting the pixel number if we were to start
counting from top left and wrapped around everytime we reached the end. The formula for the index could also be represented with this equation:

$$
i = x_{px} + y_{px} \cdot w_{px}
$$

where $w_px$ is the width of the screen and $x_px, y_px$ are the current pixel positions from the top left corner in pixels.
After we have calculated the electrical potential for every pixel and stored it inside the buffer, we can then use that buffer in a second compute shader which will actually convert the potential to a field.
The result is then used in particle and grid shader to correctly orrient the arrows and make 'test charges' (our particles) move through the electric field.

## Electric Potential

For every new module of my program, such as particles, or electrostatics in this case, we create a new `Manager` and a new `Pipeline`.
