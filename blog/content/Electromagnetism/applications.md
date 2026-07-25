---
order: 5
title: Applications of Gauss Law
---

# Balancing particles with fixed charges

Right, so we talked about this Gauss Law ($\vec{\nabla} \cdot \vec{E} = \frac{\rho}{\epsilon_0}$), but in what type of situations can we apply it? What does it tell us, and how can we use it?

Well, in principle let's recap what it means. The divergence of an electric field is equal to the charge density inside. The divergence measures how much is flowing in or out of a point.
As an example, this statement tells us that we are not able to balance a test charge inside our field.
Even if we surrounded a test charge by positive charges, Gauss' Law does not permit the test charge to remain balanced and stable.
In this scenario, stable means that if the particle was to be displaced in any direction, there would be a force counteracting that movement, returning the particle to the original position.
Now, how does Gauss' Law tell us that this is in fact not possible? Well, consider a test charge, which does not influence the electric field, enclosed by a surface in a vacuum with no other charges.
As per our requirements, for the charge to be stable, there has to be a force acting upon it. This means that the surface integral would need to be negative, as the point acts as a sink.
However, that would directly contradict our statement that the particle is in empty space with no other particles surrounding it, meaning the charge density is 0! And as we know, $0$ is not less than $0$.
For that effect to happen, there has to be a non-electric external force acting upon the test charge, like the walls of a tube, or the particle has to be placed directly over a charge.
This can be confirmed with our simulation, where by using the coordinate tool, we can spawn in multiple positive charges in a triangle formation and then place test charges and see...

<div class="img-container">
  <img src="./assets/balance.webp" alt="Balancing a charge" />
</div>

Oh wait, the charges ARE balanced, they are directly returning to the center.
But I just said it is mathematically impossible, how does this happen?

Turns out simulations are very capable of giving a general feel, but sometimes they just can't show the true physical properties.
This usually happens for a variety of reasons, such as floating point number inaccuracy, approximation techniques, and most importantly, damping effects.
All of these factors add friction and basically slow down the particle, causing it to remain relatively stable.

# The atom

Another interesting field to apply this law to is the atom, of course.
In the early days of physics, the atom was believed to be a ball of constant positive charge with small negative charges sprinkled in it.
This static arrangement of charges is of course not possible and would collapse, which was later proven wrong by the Rutherford experiment, where it was confirmed that most of the positive charges were concentrated in a small dense nucleus. This now means that the electrons are orbiting the nucleus, however, that means that as electrons accelerate, they lose kinetic energy. Wouldn't this cause collapse and chaos too?

This is where quantum physics comes in, where the uncertainty principle does not allow the electrons to come too close to the nucleus. We will not, however, go into too much detail on quantum physics today, as this is a topic for a different chapter.
