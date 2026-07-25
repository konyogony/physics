---
title: Fundamentals
order: 2
---

Alright, so I have said _Laplace Transform_ quite a few times now, but what really is it?

# Transforms

As you can see, _Laplace Transform_ comes from two words -- Laplace, and transform.
Laplace stems from a famous polmathematician, Pierre-Simon Laplace, who was born in 1749 and was famous for his foundation work in probability theory and classical mechanics.
However, _transform_ is a more vague description of an internal mechanism. As you may know, we have functions. These functions map numbers from one domain to a different one, well, same can be applied for functions.
We can map (transform) a function to behave like a different one.

# Description

Now let us answer fundamental question of what is it? The primary function of a Laplace Transform is to break down a any continous semi-infinite $\left(t\in [0,\infty)\right)$ natural function in the time domain, into one or more exponentials in the s-domain.

```tikz
\begin{scope}[scale=1.8, transform shape]
    \begin{scope}
        % Axes
        \draw[thick, ->] (-0.2,0) -- (2.5,0) node[right] {$t$};
        \draw[thick, ->] (0,-0.2) -- (0,1.8) node[above] {$f(t)$};
        \draw[very thick, black] plot[domain=0:2.2, samples=100]
            (\x, {0.8 + 0.7*sin(600*\x)*exp(-1.1*\x)});
        \node[scale=0.8, font=\sffamily] at (1.1, -0.4) {Time Domain};
    \end{scope}
    \draw[-{Stealth[length=4mm]}, very thick, black] (2.8, 0.8) -- (4.2, 0.8)
        node[midway, above=2pt, font=\large] {$\mathcal{L}$};
    \begin{scope}[xshift=5cm]
        \draw[thick, ->] (-0.2,0) -- (2.5,0) node[right] {$$};
        \draw[thick, ->] (0,-0.2) -- (0,1.8) node[above] {$$};
        \draw[very thick, black] (0.2, 1.5) .. controls (0.4, 0.3) and (1.5, 0.1) .. (2.2, 0.05);
        \draw[thick, black, opacity=0.8] (0.2, 1.2) .. controls (0.6, 0.2) and (1.5, 0.05) .. (2.2, 0.02);
        \draw[thick, black, opacity=0.8] (0.2, 0.6) .. controls (0.8, 0.1) and (2, 0.02) .. (2.2, 0.01);
        \node[scale=0.8, font=\sffamily] at (1.1, -0.4) {$s$-Domain};
        \node[scale=0.6, black] at (1.5, 1.2) {Exponential};
        \node[scale=0.6, black] at (1.5, 1.0) {Components};
    \end{scope}
\end{scope}
```

What the hell is the S-Domain and the S-Plane. Well, before we can answer that properly, lets look at the algebraic representation of the Laplace transform. Below, the function $f(t)$ is transformed to the function $F(s)$ by the Laplace Transform.

$$
\mathcal{L} \{f(t)\} = F(s) =  \int_0^{\infty}f(t)\,e^{-st}dt
$$

As you can see, the laplace transform can be expressed in a simple integral, which for some can closely resemble the Fourier Transform:

$$
\mathcal{F} \{g(t)\} = G(f) = \int_{-\infty}^{+\infty}g(t)\,e^{-2i\pi ft}dt
$$

Now, back to the Laplace, lets talk about what it actually does. Our main goal as previously mentioned is to find certain exponentials that a function can be broken up into. This is done by looking at the s-plane.
The s-plane consists of every possible complex number. There we will have to look for poles, the location of each pole, or a point of divergence, shows us one of the exponents. Why does this happen? Well, lets look at an example together.

$$
\mathcal{L} \{\cos(t)\} = \int_0^{\infty}\cos(t)\,e^{-st}dt\\
$$

For each point on the s-plane, we compute this integral right here, which can be actually quite easily solved using Integration By Parts (IBP). This means that for every point we make a sum of the product of $\cos(t)$ and $e^{-st}$.
The bounds of infinity and the negative inside the exponential tell us that most likely this integral will converge to a certain location. The points at which the integral is not able to converge, we will get a pole.

```tikz
\begin{scope}[scale=1.5, transform shape]
    \fill[black, opacity=0.1] (0,-3) rectangle (3.5,3);
    \draw[black, opacity=0.1, step=1cm] (-2.9,-2.9) grid (3.4,2.9);
    \draw[thick, ->] (-3,0) -- (3.5,0) node[right] {$\Re{\{s\}}$};
    \draw[thick, ->] (0,-3) -- (0,3) node[above] {$\Im{\{s\}}$};
    \begin{scope}[very thick]
        \draw[black] (-0.15, 1.15) -- (0.15, 0.85);
        \draw[black] (-0.15, 0.85) -- (0.15, 1.15);
        \node[anchor=west, xshift=3pt, scale=0.8] at (0, 1) {$s = i$};

        \draw[black] (-0.15, -0.85) -- (0.15, -1.15);
        \draw[black] (-0.15, -1.15) -- (0.15, -0.85);
        \node[anchor=west, xshift=3pt, scale=0.8] at (0, -1) {$s = -i$};
    \end{scope}

    \node[scale=0.8, black, align=center] at (1.8, 2) {Region of\\Convergence};
    \node[scale=0.8, black, align=center] at (-1.5, 2) {Divergence\\Zone};
    \node[below left, scale=0.6] at (0,0) {0};
\end{scope}
```

Why is only the right side defined? Well, to answer that, we will have to understand what the term $e^{-st}$ actually does. In general, $e^{st}$ is a special exponential, which can decay, grow and osciallte in time.
For example, the Imaginary component of $s$ is directly responsible for oscillations and their directation, as shown below.
