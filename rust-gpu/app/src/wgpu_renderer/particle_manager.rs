use shaders::particle::Particle;

pub struct ParticleManager {
    pub particles: Vec<Particle>,
}

impl ParticleManager {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn create_particle(&mut self, position: [f32; 2]) {
        let particle = Particle {
            position,
            velocity: [0.0; 2],
            color: [1.0, 1.0, 1.0],
            _pad: 0.0,
        };
        println!("{:?}", particle);
        self.particles.push(particle);
    }

    pub fn remove_all_particles(&mut self) {
        self.particles.clear();
    }
}
