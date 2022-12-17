struct Particle {
    position_x: f32,
    position_y: f32,
    position_z: f32,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
}

struct ComputationData {
    delta_time: f32,
    nb_particles: u32,
}

@group(0) @binding(0) var<storage, read_write> particlesData: array<Particle>;
@group(1) @binding(0) var<uniform> data: ComputationData;

@compute @workgroup_size(64, 1, 1) 
fn main(@builtin(global_invocation_id) param: vec3<u32>) {
    if (param.x >= u32(data.nb_particles)) {
          return;
    }

    var particle = particlesData[param.x];

    particlesData[param.x].position_x += particle.velocity_x * data.delta_time;
    particlesData[param.x].position_y += particle.velocity_y * data.delta_time;
    particlesData[param.x].position_z += particle.velocity_z * data.delta_time;

    particlesData[param.x].velocity_y += -9.81 * data.delta_time;
}