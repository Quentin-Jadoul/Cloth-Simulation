use wgpu_bootstrap::{
    window::Window,
    frame::Frame,
    cgmath::{ self, prelude::* },
    application::Application,
    texture::create_texture_bind_group,
    context::Context,
    camera::Camera,
    default::{ Vertex, Particle },
    wgpu,
    geometry::icosphere,
};

const NUM_PARTICLES_PER_ROW: u32 = 3;
const PARTICLE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(NUM_PARTICLES_PER_ROW as f32 * 0.5, 0.0, NUM_PARTICLES_PER_ROW as f32 * 0.5);

const CUBE_SIZE: f32 = 5.0;
//create a cube vertices and indices
const VERTICES: &[Vertex] = &[
    Vertex { position: [-CUBE_SIZE, -CUBE_SIZE, -CUBE_SIZE], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [-CUBE_SIZE, CUBE_SIZE, -CUBE_SIZE], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [CUBE_SIZE, CUBE_SIZE, -CUBE_SIZE], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [CUBE_SIZE, -CUBE_SIZE, -CUBE_SIZE], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [-CUBE_SIZE, -CUBE_SIZE, CUBE_SIZE], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [-CUBE_SIZE, CUBE_SIZE, CUBE_SIZE], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [CUBE_SIZE, CUBE_SIZE, CUBE_SIZE], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [CUBE_SIZE, -CUBE_SIZE, CUBE_SIZE], normal: [0.0, 0.0, 0.0], tangent: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], },
    
];

const INDICES: &[u16] = &[
    0,1,
    1,2,
    2,3,
    3,0,
    0,4,
    1,5,
    2,6,
    3,7,
    4,5,
    5,6,
    6,7,
    7,4,
];
struct MyApp {
    
    camera_bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    line_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    cube_vertex_buffer: wgpu::Buffer,
    cube_index_buffer: wgpu::Buffer,
    particles: Vec<Particle>,
    particle_buffer: wgpu::Buffer,
    nb_indices: usize,
}

impl MyApp {
    fn new(context: &Context) -> Self {
    
        let camera = Camera {
            eye: (0.0, 10.0, 15.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: context.get_aspect_ratio(),
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let (_camera_buffer, camera_bind_group) = camera.create_camera_bind_group(context);
    
        let pipeline = context.create_render_pipeline(
            "Render Pipeline",
            include_str!("red.wgsl"),
            &[Vertex::desc(), Particle::desc()],
            &[
                &context.camera_bind_group_layout,
            ],
            wgpu::PrimitiveTopology::TriangleList
        );

        let line_pipeline = context.create_render_pipeline(
            "Render Pipeline",
            include_str!("blue.wgsl"),
            &[Vertex::desc()],
            &[
                &context.camera_bind_group_layout,
            ],
            wgpu::PrimitiveTopology::LineList
        );
    
        let (vertices, indices) = icosphere(1);

        let nb_indices = indices.len();
    
        let vertex_buffer = context.create_buffer(vertices.as_slice(), wgpu::BufferUsages::VERTEX);
        let index_buffer = context.create_buffer(indices.as_slice(), wgpu::BufferUsages::INDEX);

        let cube_vertex_buffer = context.create_buffer(VERTICES, wgpu::BufferUsages::VERTEX);
        let cube_index_buffer = context.create_buffer(INDICES, wgpu::BufferUsages::INDEX);

        let particles = (0..NUM_PARTICLES_PER_ROW*NUM_PARTICLES_PER_ROW).map(|index| {
            let x = index % NUM_PARTICLES_PER_ROW;
            let z = index / NUM_PARTICLES_PER_ROW;
            let position = cgmath::Vector3 { x: (x as f32) * 3 as f32, y: 0.0, z: (z as f32) * 3 as f32 } - PARTICLE_DISPLACEMENT * 3 as f32;
            let velocity = cgmath::Vector3 { x: 1.0, y: 1.0, z: 1.0 };

            Particle {
                position: position.into(), velocity: velocity.into(),
            }
        }).collect::<Vec<_>>();

        //let particle_data = particles.iter().map(Particle).collect::<Vec<_>>();
        let particle_buffer = context.create_buffer(particles.as_slice(), wgpu::BufferUsages::VERTEX);
        
        Self {
            camera_bind_group,
            pipeline,
            line_pipeline,
            vertex_buffer,
            index_buffer,
            cube_vertex_buffer,
            cube_index_buffer,
            particles,
            particle_buffer,
            nb_indices
        }
    }
}

impl Application for MyApp {
    fn render(&self, context: &Context) -> Result<(), wgpu::SurfaceError> {
        let mut frame = Frame::new(context)?;

        {
            let mut render_pass = frame.begin_render_pass(wgpu::Color {r: 0.1, g: 0.2, b: 0.3, a: 1.0});

            //particle render
            render_pass.set_pipeline(&self.pipeline);
            
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.particle_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(self.nb_indices as u32), 0, 0..self.particles.len() as _);

            //cube render
            render_pass.set_pipeline(&self.line_pipeline);

            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.cube_vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.cube_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(INDICES.len() as u32), 0, 0..1);
        }

        frame.present();

        Ok(())
    }

    fn update(&mut self, context: &Context, delta_time: f32) {
        for particle in self.particles.iter_mut() {
            //update the position of the particle
            particle.position[0] += particle.velocity[0] * delta_time;
            particle.position[1] += particle.velocity[1] * delta_time;
            particle.position[2] += particle.velocity[2] * delta_time;
        }

        let particle_data = self.particles.clone();
        context.update_buffer(&self.particle_buffer, particle_data.as_slice());
    }
}

fn main() {
    let window = Window::new();

    let context = window.get_context();

    let my_app = MyApp::new(context);

    window.run(my_app);
}
