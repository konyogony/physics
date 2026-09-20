#![allow(clippy::too_many_arguments)]
use crate::wgpu_renderer::bind_groups::GlobalBindGroupLayout;
use crate::wgpu_renderer::bind_groups::electric::ElectricBindGroups;
use crate::wgpu_renderer::bind_groups::electric::ElectricBindGroupsLayout;
use crate::wgpu_renderer::bind_groups::electric::ElectricStorageBuffers;
use shaders_shared::BEM_WIRE_RADIUS;
use shaders_shared::Charge;
use shaders_shared::EPSILON;
use shaders_shared::MAX_CHARGES;
use shaders_shared::MAX_PLATES;
use shaders_shared::PX_PER_UNIT;
use shaders_shared::Plate;
use shaders_shared::SEGMENTS_PER_PLATE;
use shaders_shared::Segment;
use wgpu::Device;
use wgpu::Queue;
use winit::dpi::PhysicalSize;

pub struct ElectricManager {
    // plate
    pub plates: Vec<Plate>,
    pub segments: Vec<Segment>,
    pub plates_buffer_size: u64,
    pub segments_buffer_size: u64,
    pub segments_dirty: bool,
    // charge
    pub charges: Vec<Charge>,
    pub charges_buffer_size: u64,
    pub max_steps: usize,
    pub next_charge: f32,
    pub num_particles_per_charge: u32,
    // Shared
    pub electric_storage_buffers: ElectricStorageBuffers,
    pub electric_bind_groups: ElectricBindGroups,
    pub size: PhysicalSize<u32>,
    pub tracing_capacity_size: usize,
}

impl ElectricManager {
    pub fn new(
        device: &Device,
        queue: &Queue,
        global_bind_group_layout: &GlobalBindGroupLayout,
        size: PhysicalSize<u32>,
        initial_charges: Vec<Charge>,
        initial_plates: Vec<Plate>,
        max_steps: usize,
        num_particles_per_charge: u32,
    ) -> Self {
        let charges_buffer_size = (std::mem::size_of::<Charge>() * MAX_CHARGES as usize) as u64;
        let plates_buffer_size = (std::mem::size_of::<Plate>() * MAX_PLATES as usize) as u64;
        let segments_buffer_size =
            (std::mem::size_of::<Segment>() * (MAX_PLATES * SEGMENTS_PER_PLATE) as usize) as u64;

        let segments = Self::generate_segments(initial_plates.clone(), SEGMENTS_PER_PLATE as u64);

        let (electric_storage_buffers, tracing_capacity_size) =
            global_bind_group_layout.electric.create_electric_buffers(
                device,
                size,
                queue,
                charges_buffer_size,
                plates_buffer_size,
                segments_buffer_size,
                initial_charges.clone(),
                initial_plates.clone(),
                segments.clone(),
                max_steps,
                num_particles_per_charge,
            );
        let electric_bind_groups = global_bind_group_layout
            .electric
            .create_electric_bind_groups(device, &electric_storage_buffers);

        Self {
            plates: initial_plates,
            segments,
            segments_buffer_size,
            segments_dirty: true,
            plates_buffer_size,
            charges: initial_charges,
            charges_buffer_size,
            max_steps,
            next_charge: 1.0,
            num_particles_per_charge,
            electric_bind_groups,
            electric_storage_buffers,
            size,
            tracing_capacity_size,
        }
    }

    // resize can also act as a recreate function
    pub fn resize(
        &mut self,
        device: &Device,
        queue: &Queue,
        new_size: PhysicalSize<u32>,
        global_bind_group_layout: &GlobalBindGroupLayout,
        max_steps: usize,
        num_particles_per_charge: u32,
    ) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        let old_size = self.size;
        let old_center = egui::Vec2::new(old_size.width as f32 / 2.0, old_size.height as f32 / 2.0);
        let new_center = egui::Vec2::new(new_size.width as f32 / 2.0, new_size.height as f32 / 2.0);

        let delta = new_center - old_center;
        for charge in self.charges.iter_mut() {
            charge.position = (egui::Vec2::from(charge.position) + delta).into();
        }
        for plate in self.plates.iter_mut() {
            for v in plate.edges.chunks_exact_mut(2) {
                v[0] += delta.x;
                v[1] += delta.y;
            }
        }

        // plates got their pos updated, so segments gotta be re-gened
        // and the computed charge value will have to be re-computed too
        self.segments = Self::generate_segments(self.plates.clone(), SEGMENTS_PER_PLATE as u64);

        let (new_buffers, new_capacity) =
            global_bind_group_layout.electric.create_electric_buffers(
                device,
                new_size,
                queue,
                self.charges_buffer_size,
                self.plates_buffer_size,
                self.segments_buffer_size,
                self.charges.clone(),
                self.plates.clone(),
                self.segments.clone(),
                max_steps,
                num_particles_per_charge,
            );
        self.electric_storage_buffers = new_buffers;
        self.electric_bind_groups = global_bind_group_layout
            .electric
            .create_electric_bind_groups(device, &self.electric_storage_buffers);
        self.tracing_capacity_size = new_capacity;
        self.size = new_size;
        self.max_steps = max_steps;
        self.num_particles_per_charge = num_particles_per_charge;
        self.segments_dirty = true;
    }

    pub fn ensure_tracing_capacity(
        &mut self,
        device: &Device,
        bind_group_layout: &ElectricBindGroupsLayout,
    ) -> bool {
        let active_particles =
            (self.charges.len() * self.num_particles_per_charge as usize) + self.segments.len();
        let needed_elements = (active_particles * self.max_steps).max(1);

        if needed_elements <= self.tracing_capacity_size {
            return false;
        }

        let trace_size = std::mem::size_of::<shaders_shared::TracePoint>();
        let max_binding_size = device.limits().max_storage_buffer_binding_size as usize;
        let max_allowable_elements = max_binding_size / trace_size;

        if needed_elements > max_allowable_elements {
            eprintln!(
                "Warning: Tracing buffer request ({needed_elements} elements, ~{:.2} MB) exceeds limit ({max_allowable_elements} elements, 128 MB). Clamping to max.",
                (needed_elements * trace_size) as f64 / 1_048_576.0
            );
        }
        let new_capacity = (needed_elements + needed_elements / 5).min(max_allowable_elements);
        let new_buffer_size = (new_capacity * trace_size) as u64;

        self.electric_storage_buffers.tracing = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("TracingBuffer"),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            size: new_buffer_size,
            mapped_at_creation: false,
        });

        self.electric_bind_groups =
            bind_group_layout.create_electric_bind_groups(device, &self.electric_storage_buffers);

        self.tracing_capacity_size = new_capacity;
        true
    }

    pub fn remove_charge(
        &mut self,
        queue: &Queue,
        position: [f32; 2],
        max_distance: Option<f32>,
    ) -> Option<Charge> {
        if self.charges.is_empty() {
            return None;
        }

        // go through all charges, enumerate to get index and charge.
        // then calculate distance between the charge and position of cursor
        // then get closest one
        let (closest_idx, min_dist_sq) = self
            .charges
            .iter()
            .enumerate()
            .map(|(idx, charge)| {
                let dx = charge.position[0] - position[0];
                let dy = charge.position[1] - position[1];
                let dist_sq = dx * dx + dy * dy;
                (idx, dist_sq)
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))?;

        // make sure its actually close tho
        if let Some(max_dist) = max_distance
            && min_dist_sq > max_dist * max_dist
        {
            return None;
        }

        let removed = self.charges.swap_remove(closest_idx);
        if !self.charges.is_empty() {
            queue.write_buffer(
                &self.electric_storage_buffers.charges,
                0,
                bytemuck::cast_slice(&self.charges),
            );
        }

        self.segments_dirty = true;
        Some(removed)
    }

    pub fn add_charge(&mut self, queue: &Queue, position: [f32; 2]) {
        if self.charges.len() >= MAX_CHARGES as usize {
            return;
        }

        let charge = Charge {
            position,
            charge: self.next_charge,
            _pad: 0.0,
        };

        let offset = (self.charges.len() * std::mem::size_of::<Charge>()) as u64;
        let data = bytemuck::bytes_of(&charge);

        // We can INSERT specific pieces of data into the buffer.
        queue.write_buffer(&self.electric_storage_buffers.charges, offset, data);
        self.charges.push(charge);
        self.segments_dirty = true;
    }

    pub fn set_next_charge(&mut self, next_value: f32) {
        self.next_charge = next_value
    }

    pub fn toggle_charge(&mut self) {
        self.next_charge = -self.next_charge;
    }

    pub fn remove_all_charges(&mut self) {
        self.charges = Vec::new();
        self.segments_dirty = true;
    }

    // Plates & Segments

    pub fn generate_segments(plates: Vec<Plate>, segment_per_plate: u64) -> Vec<Segment> {
        let mut all_segments: Vec<Segment> = Vec::new();

        for plate in plates.clone() {
            // Extract all the edges
            let edges = vec![
                glam::Vec2::new(plate.edges[0], plate.edges[1]),
                glam::Vec2::new(plate.edges[2], plate.edges[3]),
                glam::Vec2::new(plate.edges[4], plate.edges[5]),
                glam::Vec2::new(plate.edges[6], plate.edges[7]),
            ];

            // Separate into vectors and lengths
            let mut edges_lengths: Vec<f32> = Vec::new();
            let vec_a = edges[1] - edges[0];
            edges_lengths.push(vec_a.length());

            let vec_b = edges[2] - edges[1];
            edges_lengths.push(vec_b.length());

            let vec_c = edges[3] - edges[2];
            edges_lengths.push(vec_c.length());

            let vec_d = edges[0] - edges[3];
            edges_lengths.push(vec_d.length());

            // get perimeter and the ideal length for each segemnt
            let perimeter: f32 = edges_lengths.iter().sum();

            if perimeter <= 0.0 {
                continue;
            }

            let ideal = perimeter / segment_per_plate as f32;

            let mut segment_lengths: Vec<f32> = Vec::new();
            let mut segment_floors: Vec<f32> = Vec::new();
            let mut segment_remainders: Vec<f32> = Vec::new();

            // Compute the actual length + the rounded part
            // Then the remainder for each
            let segment_a = edges_lengths[0] / ideal;
            segment_lengths.push(segment_a);
            segment_floors.push(segment_a.floor().max(1.0));
            segment_remainders.push(segment_a.fract());

            let segment_b = edges_lengths[1] / ideal;
            segment_lengths.push(segment_b);
            segment_floors.push(segment_b.floor().max(1.0));
            segment_remainders.push(segment_b.fract());

            let segment_c = edges_lengths[2] / ideal;
            segment_lengths.push(segment_c);
            segment_floors.push(segment_c.floor().max(1.0));
            segment_remainders.push(segment_c.fract());

            let segment_d = edges_lengths[3] / ideal;
            segment_lengths.push(segment_d);
            segment_floors.push(segment_d.floor().max(1.0));
            segment_remainders.push(segment_d.fract());

            // Get highest remainder
            let sum_floors: f32 = segment_floors.iter().sum();
            let left_segments = (segment_per_plate as f32 - sum_floors).max(0.0) as usize;

            for _ in 0..left_segments {
                if let Some((highest_index, _)) = segment_remainders
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.total_cmp(b))
                {
                    segment_floors[highest_index] += 1.0;
                    segment_remainders[highest_index] = -1.0;
                }
            }
            let winding = (edges[1] - edges[0]).perp_dot(edges[2] - edges[0]).signum();

            let edge_pairs = [
                (edges[0], edges[1]),
                (edges[1], edges[2]),
                (edges[2], edges[3]),
                (edges[3], edges[0]),
            ];

            for (idx, &(start, end)) in edge_pairs.iter().enumerate() {
                let num_segments = segment_floors[idx] as usize;

                let edge_vec = end - start;
                let tangent = edge_vec / edge_vec.length().max(EPSILON);
                let normal = glam::Vec2::new(tangent.y, -tangent.x) * winding;

                for j in 0..num_segments {
                    let t0 = j as f32 / num_segments as f32;
                    let t1 = (j + 1) as f32 / num_segments as f32;

                    let p0 = start + (end - start) * t0;
                    let p1 = start + (end - start) * t1;

                    all_segments.push(Segment {
                        midpoint: [(p0.x + p1.x) / 2.0, (p0.y + p1.y) / 2.0],
                        normal: [normal.x, normal.y],
                        length: (p1 - p0).length(),
                        target: plate.potential,
                        solved_charge: 0.0,
                        _pad: 0.0,
                    });
                }
            }
        }

        all_segments
    }

    pub fn solve_for_charges(&mut self, queue: &Queue) {
        let n = self.segments.len();

        if n == 0 || !self.segments_dirty {
            return;
        }

        let mut a = nalgebra::DMatrix::<f32>::zeros(n, n);
        let mut b = nalgebra::DVector::<f32>::zeros(n);

        let s = PX_PER_UNIT;
        let soft2 = s * s;

        for i in 0..n {
            let segment = self.segments[i];
            let midpoint = egui::Vec2::from(segment.midpoint) / s;

            let external: f32 = self
                .charges
                .iter()
                .map(|c| {
                    c.charge
                        / ((midpoint - egui::Vec2::from(c.position) / s).length_sq() + soft2).sqrt()
                })
                .sum();

            b[i] = segment.target - external;

            for j in 0..n {
                a[(i, j)] = if i == j {
                    let l = self.segments[i].length / s;
                    (2.0 / l) * (l / (2.0 * BEM_WIRE_RADIUS)).asinh()
                } else {
                    let d2 =
                        (midpoint - egui::Vec2::from(self.segments[j].midpoint) / s).length_sq();
                    1.0 / (d2 + soft2).sqrt()
                };
            }
        }

        let decomp = a.lu();
        if let Some(q) = decomp.solve(&b) {
            let solved_q = q.as_slice();
            for i in 0..n {
                self.segments[i].solved_charge = solved_q[i];
            }

            queue.write_buffer(
                &self.electric_storage_buffers.segments,
                0,
                bytemuck::cast_slice(&self.segments),
            );
        } else {
            eprintln!("BEM linear system was singular or ill-conditioned");
        }

        self.segments_dirty = false;
    }
}
