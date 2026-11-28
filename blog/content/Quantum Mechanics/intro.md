---
title: Superposition & Facts of Life
order: 1
---

# Intro

Welcome back! Today, I am starting a month-long commitment to studying Quantum Mechanics. I will be using [MIT OpenCourseWare Quantum Mechanics I](https://ocw.mit.edu/courses/8-04-quantum-physics-i-spring-2013/pages/calendar/) lectures, including problem sets and exams.
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
These particles are then capable of going between the two plates, and hence cause a spark. The number of sparks is measured and the Geiger counter is made. As we know, the clicks and counts on it are totally random.

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
E=h\nu \\
E=h \frac{c}{\lambda} \\
Pc=h\frac{c}{\lambda} \\
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

Click [here](https://ocw.mit.edu/courses/8-04-quantum-physics-i-spring-2013/cf1b32491611fe6ce5ff1a2a40d39a97_MIT8_04S13_ps1.pdf) to access the full PDF for the questions I am completing.

## Question 1

Imagine a hydrogen atom. A single proton with an electron orbiting around it. As the electron orbits, it radiates energy following the equation in cgs units:

$$
\frac{dE}{dt} = - \frac{2}{3} \frac{q^2 a^2}{c^3}
$$

### (a): Show that the energy lost per revolution is small compared to the electron’s kinetic energy.

We have to show that the ratio $E_{\text{lost}} / E_k$ is below $1$. Hence, first find the energy lost over the time period $T$:

$$
E_{\text{lost}} = T \cdot \left| \frac{dE}{dt} \right|
$$

We know that the time period $T$ for a single complete orbit is given by $T=2\pi r / v$. Now, let us combine the equations into the ratio.

$$
\frac{E_{\text{lost}}}{E_k} = T \, \frac{2}{3} \frac{q^2 a^2}{c^3} \frac{2}{mv^2} \\
\frac{E_{\text{lost}}}{E_k} = \frac{2\pi r}{v} \frac{2}{3} \frac{q^2 a^2}{c^3} \frac{2}{mv^2}
$$

We can also correlate that the electric force $F_E$ will be providing the centripetal force $F_C$ required for circular motion.

$$
F_E = F_C \\
\frac{q^2}{r^2} = \frac{mv^2}{r} \\
\frac{q^2}{r} = mv^2
$$

Hence,

$$
\frac{E_{\text{lost}}}{E_k} = \frac{2\pi r}{v} \frac{2}{3} \frac{q^2 a^2}{c^3} \frac{2r}{q^2} \\
\frac{E_{\text{lost}}}{E_k} = \frac{8\pi r^2}{3v}\frac{a^2}{c^3}
$$

We can also deduce that $a = v^2 / r \Longrightarrow a^2 = v^4 / r^2$, simplifying the equation to:

$$
\frac{E_{\text{lost}}}{E_k} = \frac{8\pi r^2}{3v}\frac{\frac{v^4}{r^2}}{c^3}\\
\frac{E_{\text{lost}}}{E_k} = \frac{8\pi}{3v}\frac{v^4}{c^3}\\
\frac{E_{\text{lost}}}{E_k} = \frac{8\pi}{3}\frac{v^3}{c^3}
$$

Since the formula for the loss of energy is non-relativistic, $v \ll c$, making $v^3 / c^3 \ll 1.0$. This means that the loss of energy is very small compared to the electron's kinetic energy.

### (b): Calculate how long it would take for the electron to spiral into the proton.

We are given that the size of an atom is $1\mathring{A}$, and the size of the nucleus to be $1\text{fm}$. If we consider the sizes to be diameters, we can work out the initial starting radius to be $r_0=0.5 \times 10^{-10}$m.

We can use the equation given before to work out the time taken by using the chain rule. This is also required because acceleration, velocity, and time period all depend on the radius, which is constantly changing.

$$
\frac{dE}{dt} = \frac{dE}{dr} \frac{dr}{dt}
$$

Let us first work out the change in energy $E$ w.r.t. the radius $r$. We can say that the (total) energy $E$ is given by the expression $U+K$, where $U$ is the potential and $K$ is the kinetic energy.
Potential energy is given by electric potential between the two charged objects, while kinetic can be related via circular motion as follows.

$$
U = -\frac{q^2}{r} \\
K = \frac{1}{2}mv^2 = \frac{1}{2}\frac{q^2}{r} \\
E = U + K = - \frac{q^2}{2r} \\
$$

By differentiating $E$, we can acquire $dE/dr$:

$$
\frac{dE}{dr} = \frac{d}{dr} \left[ - \frac{q^2}{2}r^{-1} \right] \\
\frac{dE}{dr} = -\frac{q^2}{2} \frac{d}{dr} \left[r^{-1} \right] \\
\frac{dE}{dr} = -\frac{q^2}{2} \cdot -r^{-2} \\
\frac{dE}{dr} =  \frac{q^2}{2r^2} \\
$$

We also know that $a=F_E / m$, hence $a = q^2 / mr^2$.
Substituting both equations into our differential, and expressing $a$ in terms of $r$ will yield us the following.

$$
\frac{dE}{dt} = \frac{dE}{dr} \frac{dr}{dt} \\
- \frac{2}{3} \frac{q^2 a^2}{c^3} = \frac{q^2}{2r^2} \frac{dr}{dt} \\
- \frac{4}{3} \frac{a^2}{c^3} = \frac{1}{r^2} \frac{dr}{dt} \\
- \frac{4}{3} \frac{q^4}{m^2 r^4 c^3} = \frac{1}{r^2} \frac{dr}{dt} \\
- \frac{4}{3} \frac{q^4}{m^2 c^3} = r^2 \frac{dr}{dt} \\
$$

Then, by separating variables and integrating both sides we can get to our final result.

$$
- \frac{4}{3} \frac{q^4}{m^2 c^3} = r^2 \frac{dr}{dt} \\
- \frac{4}{3} \frac{q^4}{m^2 c^3} dt = r^2 dr \\
- \frac{4}{3} \frac{q^4}{m^2 c^3} \int dt =  \int r^2 dr \\
- \frac{4}{3} \frac{q^4}{m^2 c^3} \cdot t =  \frac{r^3}{3} + C \\
$$

We can substitute our initial conditions at time $t=0$s to be $r=r_0$ m.

$$
- \frac{4}{3} \frac{q^4}{m^2 c^3} \cdot t =  \frac{r^3}{3} + C \\
0 = \frac{r_{0}^3}{3} + C \\
C = -\frac{r_{0}^3}{3} \\
\text{Plug back in} \Longrightarrow - \frac{4}{3} \frac{q^4}{m^2 c^3} \cdot t =  \frac{r^3}{3} -\frac{r_{0}^3}{3} \\
\frac{-4q^4}{m^2 c^3} \cdot t =  r^3 -r_{0}^3 \\
$$

Then, rearrange for time, and since we need to find when the electron spirals into the proton, the radius would be 0.

$$
\frac{-4q^4}{m^2 c^3} \cdot t =  r^3 -r_{0}^3 \\
t =  \frac{m^2 c^3(r^3 - r_{0}^3)}{-4q^4} \\
t_{1} = \frac{m_{e}^2 c^3 r_{0}^3}{4e^4} \\
$$

Using cgs units, we can work out that the time required for an electron to spiral into the proton of a Hydrogen atom is $1.32\times10^{-17}$ s.

### (c): Relativistic effects

Compare the velocity of the electron (assuming an orbital radius of $0.5\mathring{A}$) to the speed of light – will relativistic corrections materially alter your conclusions?

First, we find the velocity of the electron:

$$
mv^2 = \frac{q^2}{r} \\
v = \sqrt{\frac{q^2}{mr}}
$$

And when we plug in the required values, we get a velocity of $2.25\times 10^9 \text{cms}^{-1}$. Since this value is much less than $c$, relativistic corrections will not be required. However, as the electron comes closer to the proton, velocity would drastically increase, eventually coming close to $c$.
Hence, relativistic corrections might affect the conclusion, however, for most of the journey the velocity stays much less than $c$. In fact, we can find the radius at which we get $c$.

$$
r=\frac{q^2}{mc^2}
$$

This gives a value of $2.83\times 10^{-13}$ for $v=c$. If we vary velocity and try $v=\{.99,.95,.90,.10\}c$ instead, we get that $r=\{2.89\times 10^{-13}, 3.14\times 10^{-13}, 3.50\times 10^{-13}, 2.83\times 10^{-11}\}$. Now, if we take these radii and make a ratio of $r_i / r_0$, we can find the percentage of distance travelled before reaching said velocity.
This results in the following ratios: $\{0.00578, 0.00628, 0.007, 0.566\}$. At the lowest point ($v=0.1c$), where relativistic corrections start to matter, we are only about 56.6% through our journey. This means, that most of the time is accounted for and hence our measurement is accurate.

### (d): Minimum value of energy

As the electron approaches the proton, what happens to its energy? Is there a minimum value of the energy the electron can have?

As we know, energy is defined as

$$
E = U + K = - \frac{q^2}{2r}
$$

Which means that as radius decreases, the magnitude of the energy increases, but due to the negative sign, the energy decreases. This means the minimum energy would be when $r=0$, but that will not physically happen due to collisions.

## Question 2

### (a)(i) Dimensional Analysis for the characteristic energy

By using dimensional analysis and mass $m$, acceleration due to gravity $g$ and the reduced Planck's constant $\hbar$ find the expression for energy of a ball sitting on a table with only gravity acting on it in the form $E\sim m^\alpha g^\beta \hbar^\gamma$.

First, define our notation. Any variable used outside of square brackets $\left[a\right]$ is a unit. e.g. $[v]=ms^{-1}$ means that the units of velocity are meters per second. Hence, $[m]\neq m$ because $[m]=kg$.

To start this derivation, let us remind ourselves of units of $\hbar$. We were given that $\hbar = h/2\pi$, and from lecture two we can recall that $E=h\nu$. Since $\nu\lambda=c$, we can say that $E=hc/ \lambda$. This means that $h=E\lambda / c$ and $[h] = kgm^2s^{-1}$

Then, I compared those units with $[g]=ms^{-2}$ and decided to make them as similar to each other as possible. First, by removing mass we can get that $[hm^{-1}]=m^2s^{-1}$. Then, if we square both sides we get $[h^2m^{-2}]=m^4s^{-2}$.
Afterwards, by dividing one by the other we can get $[h^2m^{-2}]/[g]=m^3$. After cube rooting, we would have acquired the definition for a meter using such quantities: $[h^{2/3}m^{-1/3}g^{-1/3}]=m$.

Using this newly found definition for a meter, we can use the equation $E=mg\Delta x$ to find that $[E]=[mg][h^{2/3}m^{-1/3}g^{-1/3}]$, which simplifies to $[\hbar^{2/3}m^{1/3}g^{2/3}]$.

This means that $\alpha = 1/3, \beta = 2/3, \gamma = 2/3$.

Could this be done without $\hbar$? Most certainly an intriguing question, but I believe that the answer is no. The other two properties have no link to each other whatsoever, hence finding an equation would be quite difficult.

### (a)(ii) Dimensional Analysis for the characteristic length, time, velocity

Since we already found the definition of length, we can use a similar approach to both time and velocity, where velocity would be the ratio of length and time. The working out is not shown here because it is considered trivial.

Here is the tabular representation of the derived expressions.

| Quantity             | Derived Expression            |
| -------------------- | ----------------------------- |
| Energy ($J$)         | $\hbar^{2/3}m^{1/3}g^{2/3}$   |
| Length ($m$)         | $\hbar^{2/3}m^{-2/3}g^{-1/3}$ |
| Time ($s$)           | $\hbar^{1/3}m^{-1/3}g^{-2/3}$ |
| Velocity ($ms^{-1}$) | $\hbar^{1/3}m^{-1/3}g^{1/3}$  |

### (a)(iii) Uncertainty equation

For a system to be in the lowest energy state ($E=0$), the velocity $v$ and height $x$ would both be required to be $0$. However, this is not possible due to the uncertainty relation, $\Delta x \, \Delta p \ge \frac{\hbar}{2}$. (P.S. In this equation, the delta terms mean uncertainty)

If we think logically, to make sure that $x$ is **precisely** zero, we would need to have no uncertainty so $\Delta x \to 0$. However, to make sure the relation is still satisfied, $\Delta p \to \infty$, meaning we will never know the exact velocity value.

Since we are unable to know both precise position and precise velocity, we would never be able to reach the ground state in this system.

### (a)(iv) Ground state using derived expression

However, from our derived expression of the energy in terms of mass $m$, acceleration due to gravity $g$ and the reduced Planck's constant $\hbar$, we can see that there is no limit and that as $\hbar \to 0$, then $E \to 0$ too. This directly contradicts the previous statement... This, however, makes sense only when dealing with objects with a definite wavelength and/or frequency.

### (a)(v) Height above the surface.

Find the height above the surface for a neutron using our approximations.

We are given that $E=\frac{1}{2}mv^2 + mgx$. Rearranging for $x$ gives us:

$$
\frac{1}{mg}E - \frac{1}{2g}v^2 = x \\
x = m^{-1}g^{-1} \cdot \left( \hbar^{2/3}m^{1/3}g^{2/3} \right) - \frac{1}{2} g^{-1} \cdot \left( \hbar^{1/3}m^{-1/3}g^{1/3} \right)^2 \\
x = \hbar^{2/3}m^{-2/3}g^{-1/3} - \frac{1}{2} \hbar^{2/3}m^{-2/3}g^{-1/3}\\
x = (1.05\times 10^{-34})^{2/3}(1.7\times 10^{-27})^{-2/3}(9.81)^{-1/3} - \frac{1}{2} (1.05\times 10^{-34})^{2/3}(1.7\times 10^{-27})^{-2/3}(9.81)^{-1/3} \\
x = 3.65 \times 10^{-6} m
$$

### (b)(i) Planck's scale dimensional analysis

The scale at which both gravity and quantum mechanics co-exist is called Planck's scale. This means that it is characterised by $G_N$, $\hbar$ and $c$.

Find Planck's length $L_p$.

We know that $[G_N] = kg^{-1}m^3 s^{-2}$ and that $[\hbar]=kg  m^2 s^{-1}$. By combining the two, we cancel out the kilogram: $[G_N \hbar]=m^5s^{-3}$.
Then we can directly compare this to the speed of light $[c]=ms^{-1} \Longrightarrow [c^3] = m^3 s^{-3}$.

This means that $[G_N \hbar c^{-3}] = m^2$ and hence $L_p = G_{N}^{1/2} \hbar^{1/2}c^{-3/2}$.

### (b)(ii) Comparing Planck's scale to 1f

By using $L_p = G_{N}^{1/2} \hbar^{1/2}c^{-3/2}$, and plugging in $L_p = (6.67\times 10^{-11})^{1/2}(1.05\times 10^{-34})^{1/2} (3\times 10^8)^{-3/2}$ we can get that $L_p \approx 1.61 \times 10^{-35} m$.

Comparing this to 1 femtometer ($1\times 10^{-15}$), we can get that Planck's scale is $6.21 \times 10^{19}$ times smaller.

### (b)(iii) Planck's mass

We can do the same to acquire the mass. I started off by combining $[G_N \hbar^{-2}] = kg^{-3}m^{-1}$. Then, I noticed that we already have a definition for a meter, so I rearranged and substituted:

$$
[G_N^{1} \hbar^{-2}] = kg^{-3} m^{-1} \\
[G_N^{-1} \hbar^2] = kg^3 m \\
[G_N^{-1} \hbar^2] = kg^3 [G_{N}^{1/2} \hbar^{1/2}c^{-3/2}] \\
[G_N^{-3/2} \hbar^{3/2} c^{3/2}] = kg^3\\
[G_N^{-1/2} \hbar^{1/2} c^{1/2}] = kg
$$

This means that $M_p = G_N^{-1/2} \hbar^{1/2} c^{1/2}$, and when we evaluate it, we get $2.18 \times 10^{-8}kg$. Comparing this to the mass of a proton ($1.67\times 10^{-27}$) we can see that we do not need to know quantum gravity to study nuclear physics.
