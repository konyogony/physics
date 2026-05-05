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

For every new module of my program, such as particles, or electrostatics in this case, we create a new `Manager` and a new `Pipeline`. The `Manager` struct is responsible for creating, holding and managing
various states this new system requires. This includes buffers, bind groups and any other flags, such as output during 'ping-ponging'. Following the example we define this new `ElectricManager` as follows:

```rs
pub struct ElectricManager {
    pub charges: Vec<Charge>,
    pub electric_storage_buffers: ElectricStorageBuffers,
    pub electric_bind_groups: ElectricBindGroups,
    pub size: PhysicalSize<u32>,
    pub buffer_size: u64,
    pub next_charge: f32,
}
```

The charges vector (or a dynamic array as you may call it) will hold the position and charge of our charged particles. The buffers will hold the 3 buffers responsible for charges, potential field and the electric field.
Using buffers allows us to index into the correct slot and change data dynamically, such adding or removing charges from our system. Bind groups are just layouts that represent what data we will be passing into the shaders,
they remain static. The `size` referers to the `PhysicalSize` that our screen takes up, meaning the width and height in pixels, this is important to create `buffer_size`, which taken from the name limits the size of each buffer,
making sure we use only whats needed. Finally, `next_charge`, although the naming convention _is_ weird here, just indicates the charge on the next particle to be created. Later, this will used to alternate between positive and negative charges.

Now, how do we use this data to actually create our electric potential. Firstly, a buffer is created, and the vector of initial charges is initialised into it. This buffer gets passed onto our first comptue shader,
`electric_potential_cs`. This compute shader will be ran for every pixel on our screen and will directly use the formula discussed earlier to calculate the potential at any point. Therefore, a for loop through each particle is created,
which calculates the distance from each pixel to that charge, then sums it all up together and multiplies by a constant defined in the real life as $\frac{1}{4\pi\epsilon_0}$. However, it is important to note that the value
for $\epsilon_0$ is chosen not based on realism, but to fit the simulation look.

```rs
// Runs in 16x16 groups
#[spirv(compute(threads(16, 16), entry_point_name = "electric_potential_cs"))]
pub fn electric_potential_cs(
    // Get global index
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    // Buffer of charges
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] charges: &[Charge],
    // Buffer we will write to
    #[spirv(descriptor_set = 1, binding = 1, storage_buffer)] electric_potential: &mut [f32],
) {
    // Use our equation mentioned previously to convert to index.
    let x = global_invocation_id.x as usize;
    let y = global_invocation_id.y as usize;
    let index = x + y * constants.width as usize;

    // Check if we are outside the screen
    if x >= constants.width as usize || y >= constants.height as usize {
        return;
    }

    // Create a total variable
    let current_coords = Vec2::new(x as f32, y as f32);
    let mut potential = 0.0;

    let k = 1.0 / (4.0 * PI * constants.epsilon_naught);
    // Summation of all charges
    for charge in 0..constants.num_charges {
        // Extract charge
        let charge = charges[charge as usize];
        let charge_pos = charge.position;
        let charge_coords = Vec2::new(charge_pos[0], charge_pos[1]);

        let q = charge.charge;
        let r = (current_coords - charge_coords).length();
        // Usually potential is q / r, however for simulation purposes so that test charges dont
        // explode, we will use q / sqrt(r^2 + epsilon^2).
        // Note, this EPSILON_SQ is NOT correlated to epsilon_0, but is a smoothening factor.
        potential += q / (r + EPSILON_SQ).sqrt();
    }

    let final_potential = potential * k;

    // Write back to correct index.
    electric_potential[index] = final_potential;
}
```

## Electric Field

Now that we have acquired the electric potential buffer, we can use it directly in our second compute shader to generate the electric field iteself, as specified by $\vec{E} = -\vec{\nabla}\phi$.
But wait a minute, how do we even do partial derivatives inside of computer science? Well, since we cannot acquire the analytical solution before hand, we have to rely on sampling, more specifically the central difference method.

The central difference method can approximate the derivate of any funciton by taking multiple samples around a point, or in our case a pixel. The equation for this approximation will look as follows, where $h$ is a tiny step, e.g. 1 pixel.

$$
f'(x) \approx \frac{f(x+h) - f(x-h)}{2h}
$$

Now, if this operation is repeated in each direction, meaning horizontal and vertical, we will acquire the approximation for our $\partial \phi / \partial x$ and $\partial \phi / \partial y$.

```rs
#[spirv(compute(threads(16, 16), entry_point_name = "electric_field_cs"))]
pub fn electric_field_cs(
    // Inputs
) {
    // Extract index
    let x = global_invocation_id.x as i32;
    let y = global_invocation_id.y as i32;
    let index = x + y * constants.width as i32;

    // Check if outside bounds
    if x >= constants.width as i32 || y >= constants.height as i32 {
        return;
    }

    let max_index = constants.width as i32 * constants.height as i32;

    // Acquire index in all directions, while making sure its inside our range
    let up_index = (index + H * constants.width as i32).min(max_index - 1);
    let down_index = (index - H * constants.width as i32).max(0);
    let right_index = (index + H).min(max_index - 1);
    let left_index = (index - H).max(0);

    // Take sample from each index
    let up_sample = electric_potential[up_index as usize];
    let down_sample = electric_potential[down_index as usize];
    let left_sample = electric_potential[left_index as usize];
    let right_sample = electric_potential[right_index as usize];

    // Use central difference method.
    let d_dx = (right_sample - left_sample) / (2.0 * H as f32);
    let d_dy = (up_sample - down_sample) / (2.0 * H as f32);

    // Create the field
    let field = Field {
        field: [-d_dx, -d_dy],
        // Padding is required to make sure each struct is aligned to 16 bytes (4xf32 or 4xu32)
        _pad: [0.0; 2],
    };

    electric_field[index as usize] = field
}
```

When we multiply `H * constants.width`, we are basically forcing to wrap `H` times around, which progresses us downwards.
