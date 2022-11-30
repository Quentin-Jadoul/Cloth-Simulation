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

struct MyApp {
    diffuse_bind_group: wgpu::BindGroup,
    camera_bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    particles: Vec<Particle>,
    particle_buffer: wgpu::Buffer,
    nb_indices: usize,
}

impl MyApp {
    fn new(context: &Context) -> Self {
        let texture = context.create_srgb_texture("happy-tree.png", include_bytes!("happy-tree.png"));
    
        let diffuse_bind_group = create_texture_bind_group(context, &texture);
    
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
            include_str!("shader_instances.wgsl"),
            &[Vertex::desc(), Particle::desc()],
            &[
                &context.texture_bind_group_layout,
                &context.camera_bind_group_layout,
            ],
            wgpu::PrimitiveTopology::TriangleList
        );
    
        let (vertices, indices) = icosphere(1);

        let nb_indices = indices.len();
    
        let vertex_buffer = context.create_buffer(vertices.as_slice(), wgpu::BufferUsages::VERTEX);
        let index_buffer = context.create_buffer(indices.as_slice(), wgpu::BufferUsages::INDEX);

        let particles = (0..NUM_PARTICLES_PER_ROW*NUM_PARTICLES_PER_ROW).map(|index| {
            let x = index % NUM_PARTICLES_PER_ROW;
            let z = index / NUM_PARTICLES_PER_ROW;
            let position = cgmath::Vector3 { x: x as f32, y: 0.0, z: z as f32 } - PARTICLE_DISPLACEMENT;
            let velocity = cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 };

            Particle {
                position: position.into(), velocity: velocity.into(),
            }
        }).collect::<Vec<_>>();

        //let particle_data = particles.iter().map(Particle).collect::<Vec<_>>();
        let particle_buffer = context.create_buffer(particles.as_slice(), wgpu::BufferUsages::VERTEX);
        
        Self {
            diffuse_bind_group,
            camera_bind_group,
            pipeline,
            vertex_buffer,
            index_buffer,
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

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.particle_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(self.nb_indices as u32), 0, 0..self.particles.len() as _);
        }

        frame.present();

        Ok(())
    }

    fn update(&mut self, context: &Context, delta_time: f32) {
        // for particle in self.particles.iter_mut() {
        //     let rotation = if particle.position.is_zero() {
        //         // this is needed so an object at (0, 0, 0) won't get scaled to zero
        //         // as Quaternions can effect scale if they're not created correctly
        //         cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
        //     } else {
        //         cgmath::Quaternion::from_axis_angle(particle.position.normalize(), cgmath::Deg(45.0*delta_time))
        //     };
            
        //     particle.velocity = velocity * particle.velocity;
        // }

        // let instance_data = self.instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        // context.update_buffer(&self.instance_buffer, instance_data.as_slice());
    }
}

fn main() {
    let window = Window::new();

    let context = window.get_context();

    let my_app = MyApp::new(context);

    window.run(my_app);
}
