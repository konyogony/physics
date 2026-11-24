---
title: Superposition & Facts of Life
order: 1
---

# Intro

Welcome back! Today, I am starting a month-long commitment to studying Quantum Mechanics. I will be using MIT OpenCourseWare Quantum Mechanics I lectures, including problem sets and exams.
Over the course of a month, I will study the material, solve problems and make blog posts every few chapters. I have previously passively watched the first 7 lectures, but this time I am willing to make an effort.

# Superposition

This lesson has covered the basic principles of superposition using a simplified electron model. Each electron had two properties: Hardness and Color.
Now, this electron can be fed into a box. A measuring box. This box can measure either hardness or color, using magnetic fields ($\vec{B}=B_0\vec{e}_z$ \& $\vec{B}=B_0\vec{e}_x$) by separating the beam into separate directions.

The properties of an electron are persistent and uncorrelated. This means that knowing one of the properties (e.g. an electron being 'white') does not tell us the other (if it's 'hard').
In addition, consecutive measurements do not change the property value.

This allows us to construct pathways of electrons between boxes and measure probabilities at each end. The fun part comes when we imagine 'hard' electrons going into a hardness box. This hardness box then splits the output into two pathways, where using mirrors and a combining box they join back into a single stream.
Afterwards, the color is measured. The expected probability is 50%, which is right. However, once the 'hard' electron is changed to a 'white' electron, things get more interesting. Now, the output shows that 50% of the electrons are white. This means that once it has passed through the hardness box, something got changed.
Now, imagine blocking one of the pathways out of the hardness box, but before recombining. You would expect the probability to stay the same; however, now all electrons coming out are 'white'. Makes no sense, but this is the core principle of superposition.

When we had 2 separate paths that combined together, the electron did not take an individual path, it did not take both, and it did not take none. It was in a superposition of the two.

The uncertainty principle is also what made it not possible to know both hardness and color at the same time.

# Facts of Life

This chapter (or lecture) was focused on providing experimental evidence and facts that support quantum mechanics.

## Atoms exist

The fact that we have an atom, with a highly dense nucleus and electrons 'orbiting' it by itself already hints at a slightly different model of the world.
If we imagine an electron orbiting the nucleus, the electron would experience centripetal acceleration. As I have just learned, accelerating particles radiate EM waves.
As EM waves are radiated, energy is lost and hence the electron should eventually spiral and collapse into the nucleus. This, however, does not happen -- interesting!

We can also point our fingers to the Rutherford experiment with the gold foil and alpha particles, where we can look at deflection angles and the percentage of particles that do go through, and hence prove the existence of highly dense nuclei.

## Randomness exists

Two words: radioactive decay. We can set up a circuit in such a way that we have two plates, with a really high potential, ready to arc and spark but with an inert gas in between. When an atom decays, it will split itself and possibly release more particles.
These particles are then capable of going between the two plates, and hence cause a spark. The number of sparks are measured and the Geiger counter is made. As we know, the clicks and counts on it are totally random.

## Discrete and structured EM spectrum

Consider a different setup now: we fill a chamber with some gas such as $\text{H}_2$. Then construct a circuit with two metal plates inside the chamber, with high potential, exciting the gas.
As hydrogen (or a different element) gets excited, it releases EM waves -- light. Such light can then be directed at a prism to break it down into individual components. But, the pattern that we see is not a continuous range of colors -- instead it is a discrete set of wavelengths.
And this spectrum comes with a defining equation, meaning we can model and predict each of the peaks. e.g. $\lambda = 3646\frac{n^2}{n^2-4}$.

## The photoelectric effect

Take a sheet of metal. Shine a light on it. Sometimes, electrons will come flying off the sheet. Now, we can measure the energy of the electrons (and the light) using the following setup.
Connect two sheets of metal using a wire. One of the sheets of metal will have a light shining on it, whilst the other will not. Now, one of the sheets will have a deficit of electrons, or a positive charge, whilst the other a negative charge.
This forces electrons to flow through the circuit back to the sheet. Add in an ammeter and a power supply. Now, we can regulate the voltage on that power supply to make it either harder or easier for electrons to cross the gap.
By applying a potential difference ($\Delta V$) between two plates, we can work out that the energy required for a particle of charge $q$ is $E=q\Delta V$.

Now, where is the energy for the electron to jump coming from? The light. We can make a prediction that light with higher intensity will provide more energy, where it is independent of frequency $\nu$. But, as experiments are conducted, an equation is 'derived' to fit the data: $E=h\nu$.
This equation says that the light coming in is actually made up of packets, chunks, each with discrete energy which is proportional to the frequency. This is completely opposite of our prediction!
This means that the kinetic energy of the electrons would actually be $E_k = h\nu - W$. This means that $q\Delta V = h\nu - W$ where at some critical voltage $V_0$, we can work out $E_k$.

The photoelectric effect can also be linked to the wave-particle duality of electrons themselves. By Young's double-slit experiment we know that waves exhibit the property of being not localised and causing interference patterns. Whereas, if we replace the wave with a particle, we would get localised hits and no interference -- 2 normal distributions at each slit.
However, if we change the EM waves in Young's experiment to electrons, we arrive at the Hitachi experiment. There, the electrons are shot through the slits one by one, but the result is quite interesting. The electrons are localised, so they always hit at one particular point. However, as time goes on and the points are accumulated, an interference pattern is made up.
This links back nicely to the previous section, where the electron is not going from any individual slit (since that would produce a distinct single pattern), and it's not going through both or none -- it is in a superposition of both.

We can also experiment with some of the equations. We know that $E=h\nu$ and that $\lambda \nu = c$, as well as $E=Pc$ where $P$ is momentum. We can derive the following:

$$
E=h\nu \\[6pt]
E=h \frac{c}{\lambda} \\[6pt]
Pc=h\frac{c}{\lambda} \\[6pt]
P=\frac{h}{\lambda}
$$

## Diffraction in a solid

Let us now imagine a crystalline solid. As light shines onto the atoms, some of it is deflected at an angle $\theta$. The angle at which it is deflected can be modeled against the intensity of the light to come to a conclusion that it follows the laws mentioned previously and the equation:

$$
\lambda^{-1} = \frac{n}{2d\sin{\theta}}
$$

## Bell's (poor) Inequality

Let us use some logic now. Bell's inequality states that for any set with three binary properties: $A,B,C$, the equality $N(A,\bar{B}) + N(B,\bar{C}) \geq N(A,\bar{C})$.
This is true because if we break down each of the statements we get the following:

$$
N(A,\bar{B}) + N(B,\bar{C}) \geq N(A,\bar{C}) \\[3pt]
\left[ N(A,\bar{B},C) + N(A,\bar{B},\bar{C}) \right] + \left[ N(A,B,\bar{C}) + N(\bar{A},B,\bar{C}) \right] \geq N(A,B,\bar{C})+N(A,\bar{B},\bar{C}) \\[3pt]
N(A,\bar{B},C) + \cancel{N(A,\bar{B},\bar{C})} + N(A,B,\bar{C}) + N(\bar{A},B,\bar{C}) \geq N(A,B,\bar{C})+\cancel{N(A,\bar{B},\bar{C})} \\[3pt]
N(A,\bar{B},C) + \cancel{N(A,B,\bar{C})} + N(\bar{A},B,\bar{C}) \geq \cancel{N(A,B,\bar{C})} \\[3pt]
N(A,\bar{B},C) + N(\bar{A},B,\bar{C}) \geq 0
$$

And as we all know, when we count the number of elements, we cannot physically get below 0. However, when we link this to electrons things get weird.
It is now revealed to us that electrons have three properties: Color, Hardness and Whimsy. In reality, they mean the angular momentum $L$ along the 3 axes ($L_x, L_y, L_z$).
If we make an analogous statement for electrons: $N(\text{White}, \text{Not hard}) + N(\text{hard}, \text{Not whimsy}) \geq N(\text{white}, \text{Not Whimsy})$.
It does NOT hold true. In fact, the sign would be the exact opposite.

# Problem Set 1

After tackling the first two lectures, and blurting the material I will now attempt to do Problem Set 1. I will try my best to complete the question, without peeking at the answer. If I lack the knowledge required to answer, I will go away and research it.
