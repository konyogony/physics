---
title: A vector field simulation
---

This section will cover chapters 1-3, as well as the building process of a vector field visualiser, in more particular a velocity and acceleration field simulator.
The first chapter has opened up with a brief overview of the electromagnetic course, which covers what electric and magnetic fields are and how they interact with matter,
bringing up important concepts such as superposition,

$$
    \vec{E} = \vec{E_1} + \vec{E_2}
$$

the Maxwell's equations,

$$
\begin{aligned}
    \vec{\nabla} \cdot \vec{E} &= \frac{\rho}{\epsilon_0} \\
    \vec{\nabla} \cdot \vec{B} &= 0 \\
    \vec{\nabla} \times \vec{E} &= - \frac{\partial \vec{B}}{\partial t} \\
    \vec{\nabla} \times \vec{B} &= \frac{\partial \vec{E}}{\partial t} + \frac{\vec{j}}{\epsilon_0}
\end{aligned}
$$

and some simple examples of electric effects.
However, before we I dove into potentials and the Coulombs law, the concept of vector fields and vector calculus had to be explored.
Chapters 2 \& 3 have some great concepts of vector calculus. This includes the use of the Nabla ($\vec{\nabla}$) operator to acquire the gradient, diverge or curl of a field.
In addition, these chapters have covered important concepts such as flux, circulation and important characteristics of vectors.
Let me begin by summarising my findings and important concepts learnt in this chapter, in a way that is easier to understand.

<div class="vid-grid">
  <div class="vid-card">
    <div class="vid-shroud">
      <video muted loop playsinline src="https://static.konyogony.dev/vel-field-1.mp4"></video>
    </div>
    <label>Velocity Field 1</label>
  </div>
  <div class="vid-card">
    <div class="vid-shroud">
      <video muted loop playsinline src="https://static.konyogony.dev/vel-field-2.mp4"></video>
    </div>
    <label>Velocity Field 2</label>
  </div>
  <div class="vid-card">
    <div class="vid-shroud">
      <video muted loop playsinline src="https://static.konyogony.dev/vel-field-3.mp4"></video>
    </div>
    <label>Velocity Field 3</label>
  </div>
  <div class="vid-card">
    <div class="vid-shroud">
      <video muted loop playsinline src="https://static.konyogony.dev/acc-field-1.mp4"></video>
    </div>
    <label>Acceleration Field 1</label>
  </div>
</div>

<style>
  .vid-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    margin: 20px 0;
  }
  .vid-card {
    background: var(--lightgray);
    border: 1px solid var(--gray);
    border-radius: 8px;
    overflow: hidden;
  }
  .vid-shroud {
    width: 100%;
    aspect-ratio: 16 / 9;
    background: #000;
  }
  .vid-shroud video {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }
  .vid-card label {
    display: block;
    padding: 8px;
    text-align: center;
    font-size: 0.85rem;
    font-weight: bold;
    border-top: 1px solid var(--gray);
  }
  @media (max-width: 600px) {
    .vid-grid { grid-template-columns: 1fr; }
  }
</style>
