use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

mod opengl;
use opengl::buffers::{AttributeType, FrameBuffer, RenderBuffer, VertexBuffer, VertexBufferLayout};
use opengl::shaders::{Program, Shader};
use opengl::textures::{Texture, TextureColorFormat};
use opengl::uniforms::{Uniform, UniformType};
use opengl::Glwrapper;

mod camera;
use camera::PerspectiveCamera;

mod object;

// https://github.com/LordBenjamin/sharp-and-rusty
// https://github.com/gobanos/test-glutin-opengl/blob/master/src/main.rs

fn main() {
    // load model

    let obj = object::obj::load_new(
        "D:\\Davide\\Programmazione\\Javascript\\3D Engine - webgl\\OBJs\\suzane.obj",
        true,
        true,
    );

    // init window

    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Ratio 0.1.0");

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );

    let glwr = Glwrapper::new(&windowed_context.context());
    let gl = &glwr.gl;

    // gl stuff

    let vb = VertexBuffer::new(obj.get_vertices(), gl);
    vb.bind(gl);

    let vbl = VertexBufferLayout::new(
        vec![
            (String::from("a_Position"), AttributeType::Float3),
            (String::from("a_TexCoords"), AttributeType::Float2),
            (String::from("a_Normal"), AttributeType::Float3),
        ],
        gl,
    );
    vbl.bind(gl);

    glwr.print_errors();

    let shader = Shader::new(
        include_str!("shader_source/BSDF.vertex"),
        include_str!("shader_source/BSDF.fragment"),
        gl,
    );
    let program = Program::new(&shader, gl);
    program.bind(gl);
    // shader.delete(gl);

    let mut camera = PerspectiveCamera::new([0.0, 0.0, -5.0]);
    let (vw, vh): (f32, f32) = windowed_context.window().inner_size().into();
    camera.set_aspect_ratio(vw / vh);
    let mut vp_matrix = Uniform::new(
        "vp_matrix",
        UniformType::Mat4x4(camera.matrix()),
        &program,
        gl,
    );

    let mut texture = Texture::load_new(
        "D:/Davide/Programmazione/Rust/ratio/src/image_source/diffuse.png",
        0,
        gl,
    );
    texture.bind(gl);
    let mut image = Uniform::new(
        "image",
        UniformType::Texture(texture.get_id()),
        &program,
        gl,
    );

    // ----- UNTIL HERE WORKS BELOW IS THE HELL

    // render to texture

    let vb_screen = VertexBuffer::new(
        &vec![
            -1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 0.0,
        ],
        gl,
    );
    // vb_screen.bind(gl);
    let vbl_screen = VertexBufferLayout::new(
        vec![
            (String::from("a_Position"), AttributeType::Float2),
            (String::from("a_TexCoords"), AttributeType::Float2),
        ],
        gl,
    );
    // vbl_screen.bind(gl);

    let post_shader = Shader::new(
        include_str!("shader_source/post.vertex"),
        include_str!("shader_source/post.fragment"),
        gl,
    );
    let post_program = Program::new(&post_shader, gl);
    post_program.bind(gl);
    // post_shader.delete(gl);

    let (vw, vh) = (vw as usize, vh as usize);
    let fb = FrameBuffer::new(0, vw, vh, gl);
    fb.bind(gl);

    let rb = RenderBuffer::new(vw, vh, gl);
    rb.bind(gl);

    glwr.print_errors();

    // prepare for render pass
    vb.bind(&glwr.gl);
    vbl.bind(&glwr.gl);
    program.bind(&glwr.gl);
    fb.bind(&glwr.gl);
    texture.bind(&glwr.gl);

    // event loop

    el.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                // RESIZE
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    let (width, height) = physical_size.into();

                    /* UPDATE CAMERA */
                    camera.set_aspect_ratio((width as f32) / (height as f32));
                    vp_matrix.set(UniformType::Mat4x4(camera.matrix()), &program, &glwr.gl);

                    /* UPDATE VIEPORT */
                    glwr.resize(width, height);

                    /* RESIZE THE FRAME BUFFER TEXTURE AND THE RENDER BUFFER */
                    fb.resize_texture(width as usize, height as usize, &glwr.gl);
                    texture.bind(&glwr.gl); // back to the right texture
                    rb.resize(width as usize, height as usize, &glwr.gl);
                }
                WindowEvent::MouseWheel {
                    device_id: _,
                    delta,
                    phase: _,
                    modifiers: _,
                } => match delta {
                    glutin::event::MouseScrollDelta::LineDelta(_dx, dy) => {
                        camera.shift_position([0.0, 0.0, dy / 5.0]);
                        vp_matrix.set(UniformType::Mat4x4(camera.matrix()), &program, &glwr.gl);
                        windowed_context.window().request_redraw();
                    }
                    glutin::event::MouseScrollDelta::PixelDelta(p) => {
                        let glutin::dpi::LogicalPosition { x: _x, y } = p;
                        camera.shift_position([0.0, 0.0, (y as f32) / 200.0]);
                        vp_matrix.set(UniformType::Mat4x4(camera.matrix()), &program, &glwr.gl);
                        windowed_context.window().request_redraw();
                    }
                },
                WindowEvent::KeyboardInput {
                    device_id: _,
                    input,
                    is_synthetic: _,
                } => {
                    if let Some(key_code) = input.virtual_keycode {
                        match key_code {
                            glutin::event::VirtualKeyCode::A => {
                                camera.shift_position([-0.1, 0.0, 0.0]);
                                vp_matrix.set(
                                    UniformType::Mat4x4(camera.matrix()),
                                    &program,
                                    &glwr.gl,
                                );
                                windowed_context.window().request_redraw();
                            }
                            glutin::event::VirtualKeyCode::D => {
                                camera.shift_position([0.1, 0.0, 0.0]);
                                vp_matrix.set(
                                    UniformType::Mat4x4(camera.matrix()),
                                    &program,
                                    &glwr.gl,
                                );
                                windowed_context.window().request_redraw();
                            }
                            _ => (),
                        }
                    }
                    //
                }
                // FILE DROPPED
                WindowEvent::DroppedFile(path_buffer) => {
                    let path = path_buffer.as_path().to_str();
                    match path {
                        Some(file) => {
                            if file.ends_with(".png")
                                | file.ends_with(".jpg")
                                | file.ends_with(".jpeg")
                                | file.ends_with(".bmp")
                                | file.ends_with(".tiff")
                            {
                                texture.delete(&glwr.gl);
                                texture = Texture::load_new(file, texture.get_id(), &glwr.gl);
                                image.set(
                                    UniformType::Texture(texture.get_id()),
                                    &program,
                                    &glwr.gl,
                                );
                                windowed_context.window().request_redraw();
                            } else if file.ends_with(".obj") {
                                println!("This is an object");
                            } else {
                                println!("WARN: this file is not supported.");
                            }
                        }
                        None => {
                            println!("Error reading path");
                        }
                    }
                }
                // CLOSE
                WindowEvent::CloseRequested => {
                    &vbl.delete(&glwr.gl);
                    &vb.delete(&glwr.gl);
                    &program.delete(&glwr.gl);
                    *control_flow = ControlFlow::Exit;
                }
                // UNHANDLED
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // render pass
                glwr.draw_frame([0.05, 0.05, 0.05, 1.0]);
                glwr.clear_depth_buffer();
                glwr.depth_test(true);
                glwr.draw_triangles(obj.get_vertices_count());

                // post-processing pass
                vb_screen.bind(&glwr.gl);
                vbl_screen.bind(&glwr.gl);
                post_program.bind(&glwr.gl);
                fb.bind_texture(&glwr.gl);
                glwr.bind_drawing_buffer();

                glwr.depth_test(false);
                glwr.draw_triangles(6);

                // to screen
                windowed_context.swap_buffers().unwrap();

                // prepare for render pass
                vb.bind(&glwr.gl);
                vbl.bind(&glwr.gl);
                program.bind(&glwr.gl);
                fb.bind(&glwr.gl);
                texture.bind(&glwr.gl);
            }
            _ => (),
        }
    });
}
