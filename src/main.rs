#[macro_use]
extern crate glium;

fn main() {
    use glium::{glutin, Surface};

    use std::fs;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = &fs::read_to_string("src/shader.vs").unwrap();
    let fragment_shader_src = &fs::read_to_string("src/shader.fs").unwrap();

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t = 0.0;
    let mut _t : f32 = 0.0;
    let mut dir = 1.0;

    event_loop.run(move |event, _, control_flow| {

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        t += 0.02;
        if t > 0.5 {
            t = -0.5;
            dir = dir * -1.0;
        }
        _t = t * dir;
        
        let uniforms = uniform! {
            matrix: [
                [_t.cos(), _t.tan(), 0.0, 0.0],
                [_t.tan(), _t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ _t, _t, 0.0, 1.0f32],
            ]
        };

        // let uniforms = uniform! {
            // matrix: [
                // [1.0, 0.0, 0.0, 0.0],
                // [0.0, 1.0, 0.0, 0.0],
                // [0.0, 0.0, 1.0, 0.0],
                // [0.0, 0.0, 0.0, 1.0f32],
            // ]
        // };

        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [ -0.5,  0.5] };
        let vertex3 = Vertex { position: [ 0.5, -0.5] };
        let vertex4 = Vertex { position: [ 0.5, -0.5] };
        let vertex5 = Vertex { position: [ 0.5,  0.5] };
        let vertex6 = Vertex { position: [-0.5, 0.5] };
        let shape = vec![vertex1, vertex2, vertex3,vertex4,vertex5,vertex6];
    
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();
    });
}