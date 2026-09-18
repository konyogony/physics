---
order: 6
title: Charged Plates and more
---

# Introduction

Some time has passed since my last update on this topic, and there are more new exciting features to implement. This chapter will mainly focus on implementing charged plates.

So, what is actually the current problem? The problem right now is that I only have static point charges which exert the electric fields, however, to act as a demonstration for class, I also now need to make charged plates.
As of right now, the simulation follows a simplified pipeline where we contain the information about all the point charges and for every pixel, in the compute shader, we calculate the electric potential using $$\phi = -kq_1/r$$.
As we sum up the potentials for every charge in relation to a single point, we store that info in a buffer and send it onto a different shader. This second compute shader, then, calculates the electric field using $$\vec{E} = -\vec{\nabla}\phi$$ and also stores it inside a buffer.
This electric field buffer is then directly used for the visualisations of the grid, and the movement of particles.

Adding metal plates into this pipeline seems simple, just alter the electric potential at the point where plate resides and _et voilà!_ Unfortunetly, everything is not as simple. Metal plates are not simple point charges,
they have boundaries, and electrons and their own field inside the plate...

# General Solution

How do we go about dealing with this then? Well, let's recap some conductor knowledge. A conductor, usually a metal plate, is full of 'free' electrons. If an electric field were to be exerted on the conductor,
these electrons will percieve a force and start accelerating due to $$\vec{F}=q\vec{E}$$. In general, these electrons can either be moving, or be static. For the purpose of electrostatics, we will consider only the latter.
This means that **inside** the conductor itself, the electric potential will be $$\phi = 0$$. Since the electric field is defined as the negative gradient of the electric potential, that will turn out to be zero too, due to $$\vec{E}=-\vec{\nabla}\phi=0$$.
This also means that all of the conductor is an equipotential surface. By Gauss' law we know that $$\vec{\nabla} \cdot \vec{E} = \rho/\epsilon_0$$, where if $$E$$ is set to zero will mean that no charges are present.
However, if no charges can be present, how does a metal plate become charged in the first place? Well, the real answer is that the actual charge resides only on the surface of the conductor, bound by forces of attraction, making them not completely 'free'.

Now that we have recaped how conductors and metal plates work, lets think about what and how to change our simulation. Since our electric field $$\vec{E}$$ directly correlates to the potential, that is what we will be altering.
In addition to the already exisitng sum of potential from charges we will now consider new regions. These regions will be passed in as a set of coordinates, defining the metal conductor boundary.
This now means that for any point $$(x,y)$$ on the screen, we will have to first determine if its inside, outside or on the boundary with the region. If the point is inside, we set the potential to be a constant value, like $$10V$$.
For any point on the boundary or outside the metal plate, only on initialisation, we set the potential to be zero. Afterwards, we can imagine blurring out the edges of this box, where with every compute pass the edges
become more and more smeared out, creating that correct potential look. To account for these new regions we will have to make a ping-pong model of the potential buffer, since we will have to read, adjust values and write back.

## Mathematical Derivation

We know Maxwell's first equation to be

$$
\vec{\nabla} \cdot \vec{E} = \frac{\rho}{\epsilon_0}
$$

Since we also know that $$\vec{E} = -\vec{\nabla}\phi$$, we can combine the two forming

$$
\vec{\nabla} \cdot (-\vec{\nabla}\phi) = \frac{\rho}{\epsilon_0}
$$

by expanding $$\nabla$$ and the dot product within, we can make the following steps

$$
\begin{aligned}
\vec{\nabla} \cdot (-\vec{\nabla}\phi) &= \frac{\rho}{\epsilon_0} \\
\langle \frac{\partial}{\partial x}, \frac{\partial}{\partial y}, \frac{\partial}{\partial z} \rangle \cdot \langle \frac{-\partial \phi}{\partial x}, \frac{-\partial \phi}{\partial y}, \frac{-\partial \phi}{\partial z} \rangle &= \frac{\rho}{\epsilon_0} \\
\frac{-\partial^2 \phi}{\partial x^2} + \frac{-\partial^2 \phi}{\partial y^2} + \frac{-\partial^2 \phi}{\partial z^2}  &= \frac{\rho}{\epsilon_0} \\
\frac{\partial^2 \phi}{\partial x^2} + \frac{\partial^2 \phi}{\partial y^2} + \frac{\partial^2 \phi}{\partial z^2} &= -\frac{\rho}{\epsilon_0} \\
\nabla^2 \phi &= -\frac{\rho}{\epsilon_0}
\end{aligned}
$$

The $$\nabla^2$$ is called the _Laplacian operator_, which is -- important to note -- not a vector. In simple terms, this operator measures the difference the value of $$\phi$$ at a point and the average value of $$\phi$$ in an infinitesimal circle around that point.
Now, we also know that inside the conductor, there are no charges, meaning charge density $$\rho$$ will be zero.

$$
\nabla^2 \phi = 0
$$

This gives us the final equation, which is best approximated using the previousely mentioned 'blur'. This so called 'blur' is achieved by simply getting the average of the 4 neighbours

# Implementation

Now that we have defined the general setup and solution of the problem, we can start working our way towards solving it.

## Data Input & Passing

Let's think first about the how our data will be defined, processed and passed around. For a metal conductor we can define the coordinates of the four verticies, along with the charge on its surface:

```rs
struct Plate {
    // We will go in anti-clockwise direction starting at the bottom-left,
    // e.g. [0.0, 20.0, 10.0, 20.0, 50.0, 60.0, -10.0, 50.0] will give us:
    //
    // D(-10.0, 50.0) ---- C(50.0, 60.0)
    // |                   |
    // |                   |
    // |                   |
    // |                   |
    // |                   |
    // A(0.0, 20.0) ------ B(10.0, 20.0)
    //
    edges: [f32; 8],
    charge: f32
}
```

Along with our static charges, we will also store a `Vec<Plate>`; it will have to be passed along with other constants to the all the shaders.

Now, consider a point $$P(x,y)$$ -- how do we know if's inside or outside the boundary defined by the 4 verticies? We can use a great analogy of us taking a train arond the boundary of our surface.
For every edge, we can use a simplification of a 2D cross product with points test point $P$, and line connecting $A$ and $B$ -- $$ \left[ (B_x - A_x) \times (P_y - A_y) - (B_y - A_y) \times (P_x - A_x) \right]$$ -- to find out if the point is on the left side (positive) or on the right side (negative).
If the point is inside our shape, as we go along all of our edges, we will find that the direction is always the same, either always positive, or always negative. If one of the values ends up being zero, that means the point is on the boundary itself.
