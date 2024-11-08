use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{look_at, perspective, Mat4,Vec3, Vec4};
use std::f32::consts::PI;
use std::time::Duration;

mod camera;
mod color;
mod fragment;
mod framebuffer;
mod line;
mod obj;
mod shaders;
mod triangle;
mod vertex;

use camera::Camera;
use color::Color;
use fastnoise_lite::FastNoiseLite;
use framebuffer::Framebuffer;
use obj::Obj;
use shaders::{fragment_shader, vertex_shader, ShaderType};
use vertex::Vertex;

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite,
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation_angle: f32) -> Mat4 {
    Mat4::new_translation(&translation) * 
    Mat4::from_axis_angle(&Vec3::y_axis(), rotation_angle) * 
    Mat4::new_scaling(scale)
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 60.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    perspective(fov, aspect_ratio, 0.1, 1000.0)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0,
        0.0,
        0.0,
        width / 2.0,
        0.0,
        -height / 2.0,
        0.0,
        height / 2.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    shader_type: &ShaderType,
) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle::triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = fragment_shader(&fragment, uniforms, shader_type);
            framebuffer.set_current_color(shaded_color.to_hex());
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn line_with_thickness(
    framebuffer: &mut Framebuffer,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    thickness: f32,
) {
    let dx = (x2 as f32) - (x1 as f32);
    let dy = (y2 as f32) - (y1 as f32);
    let distance = (dx * dx + dy * dy).sqrt();

    if distance == 0.0 {
        return;
    }

    // Normalizar el vector de dirección
    let dx = dx / distance;
    let dy = dy / distance;

    framebuffer.line(x1, y1, x2, y2);

    // Para líneas muy delgadas, solo se dibuja la línea principal
    if thickness <= 1.0 {
        return;
    }

    // Dibujar líneas adicionales para el grosor
    for offset in 1..=(thickness as i32) {
        let offset = offset as f32 * 0.5;

        let perpx = -dy * offset;
        let perpy = dx * offset;

        let x1_offset = (x1 as f32 + perpx) as usize;
        let y1_offset = (y1 as f32 + perpy) as usize;
        let x2_offset = (x2 as f32 + perpx) as usize;
        let y2_offset = (y2 as f32 + perpy) as usize;

        if x1_offset < framebuffer.width
            && y1_offset < framebuffer.height
            && x2_offset < framebuffer.width
            && y2_offset < framebuffer.height
        {
            framebuffer.line(x1_offset, y1_offset, x2_offset, y2_offset);
        }

        let x1_offset = (x1 as f32 - perpx) as usize;
        let y1_offset = (y1 as f32 - perpy) as usize;
        let x2_offset = (x2 as f32 - perpx) as usize;
        let y2_offset = (y2 as f32 - perpy) as usize;

        if x1_offset < framebuffer.width
            && y1_offset < framebuffer.height
            && x2_offset < framebuffer.width
            && y2_offset < framebuffer.height
        {
            framebuffer.line(x1_offset, y1_offset, x2_offset, y2_offset);
        }
    }
}

fn render_orbit_lines(
    framebuffer: &mut Framebuffer,
    orbit_radius: f32,
    color: Color,
    segments: usize,
    uniforms: &Uniforms,
) {
    framebuffer.set_current_color(color.to_hex());

    for i in 0..segments {
        let angle1 = 2.0 * PI * (i as f32) / (segments as f32);
        let angle2 = 2.0 * PI * ((i + 1) as f32) / (segments as f32);

        // Posiciones en el espacio 3D
        let world_pos1 = Vec4::new(
            orbit_radius * angle1.cos(),
            0.0,
            orbit_radius * angle1.sin(),
            1.0,
        );
        let world_pos2 = Vec4::new(
            orbit_radius * angle2.cos(),
            0.0,
            orbit_radius * angle2.sin(),
            1.0,
        );

        let clip_pos1 = uniforms.projection_matrix * uniforms.view_matrix * world_pos1;
        let clip_pos2 = uniforms.projection_matrix * uniforms.view_matrix * world_pos2;

        let ndc_pos1 = Vec3::new(
            clip_pos1.x / clip_pos1.w,
            clip_pos1.y / clip_pos1.w,
            clip_pos1.z / clip_pos1.w,
        );
        let ndc_pos2 = Vec3::new(
            clip_pos2.x / clip_pos2.w,
            clip_pos2.y / clip_pos2.w,
            clip_pos2.z / clip_pos2.w,
        );

        // Transformar a coordenadas de pantalla
        let screen_pos1 =
            uniforms.viewport_matrix * Vec4::new(ndc_pos1.x, ndc_pos1.y, ndc_pos1.z, 1.0);
        let screen_pos2 =
            uniforms.viewport_matrix * Vec4::new(ndc_pos2.x, ndc_pos2.y, ndc_pos2.z, 1.0);

        let screen_x1 = screen_pos1.x as usize;
        let screen_y1 = screen_pos1.y as usize;
        let screen_x2 = screen_pos2.x as usize;
        let screen_y2 = screen_pos2.y as usize;

        if screen_x1 < framebuffer.width
            && screen_y1 < framebuffer.height
            && screen_x2 < framebuffer.width
            && screen_y2 < framebuffer.height
        {
            line_with_thickness(framebuffer, screen_x1, screen_y1, screen_x2, screen_y2, 0.001);
        }
    }
}

fn main() {
    let window_width = 1000;
    let window_height = 800;
    let framebuffer_width = 1000;
    let framebuffer_height = 800;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Sistema Solar",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0x000000);

    let obj_sphere = Obj::load("assets/models/sphere.obj").expect("Failed to load sphere.obj");
    let vertex_arrays_sphere = obj_sphere.get_vertex_array();

    let obj_moon = Obj::load("assets/models/moon.obj").expect("Failed to load moon.obj");
    let vertex_arrays_moon = obj_moon.get_vertex_array();

    let mut camera = Camera::new(
        Vec3::new(0.0, 100.0, 100.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix =
        create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

    let orbital_radii = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0];
    let orbital_speeds = vec![0.005,0.005, 0.005, 0.005, 0.005, 0.005];
    let shaders = vec![
        ShaderType::RockyPlanet,
        ShaderType::RockyPlanetVariant,
        ShaderType::GasGiant,
        ShaderType::ColdGasGiant,
        ShaderType::AlienPlanet,
        ShaderType::GlacialTextured,
    ];
     // Variables para controlar la cámara
    let camera_speed = 1.0;
    let rotation_speed = 0.05;
    let zoom_speed = 2.0;

    let mut time = 0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        let mut movement = Vec3::new(0.0, 0.0, 0.0);
        if window.is_key_down(Key::W) {
            movement.z -= camera_speed;
        }
        if window.is_key_down(Key::S) {
            movement.z += camera_speed;
        }
        if window.is_key_down(Key::A) {
            movement.x -= camera_speed;
        }
        if window.is_key_down(Key::D) {
            movement.x += camera_speed;
        }
        if movement.magnitude() > 0.0 {
            camera.move_center(movement);
        }

        // Rotación de la cámara
        if window.is_key_down(Key::Left) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, rotation_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, -rotation_speed);
        }

        // Zoom
        if window.is_key_down(Key::Q) {
            camera.zoom(-zoom_speed);
        }
        if window.is_key_down(Key::E) {
            camera.zoom(zoom_speed);
        }

        let view_matrix = look_at(&camera.eye, &camera.center, &camera.up);

        time += 1;
        framebuffer.clear();

        // Base uniforms para las órbitas
        let base_uniforms = Uniforms {
            model_matrix: Mat4::identity(),
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: fastnoise_lite::FastNoiseLite::new(),
        };

        let sun_rotation_speed = 0.0001; 
        let sun_rotation = time as f32 * sun_rotation_speed;

        // Renderizado del sol 
        let sun_uniforms = Uniforms {
            model_matrix: create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 1.0, sun_rotation),
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: fastnoise_lite::FastNoiseLite::new(),
        };
        render(
            &mut framebuffer,
            &sun_uniforms,
            &vertex_arrays_sphere,
            &ShaderType::Solar,
        );
        
        // Renderizado de las órbitas y planetas
        for (i, &radio) in orbital_radii.iter().enumerate() {
            render_orbit_lines(
                &mut framebuffer,
                radio,
                Color::new(128, 128, 128),
                150,
                &base_uniforms,
            );
        
            let orbital_speed = orbital_speeds[i];
            let planet_x = radio * (time as f32 * orbital_speed).cos();
            let planet_z = radio * (time as f32 * orbital_speed).sin();
            let planet_position = Vec3::new(planet_x, 0.0, planet_z);

            let speeds_rotation = vec![0.02, 0.015, 0.025, 0.018, 0.022, 0.016]; 
            let planet_rotation = time as f32 * speeds_rotation[i];
        
            let planet_uniforms = Uniforms {
                model_matrix: create_model_matrix(planet_position, 1.0, planet_rotation),
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                noise: fastnoise_lite::FastNoiseLite::new(),
            };
        
            render(
                &mut framebuffer,
                &planet_uniforms,
                &vertex_arrays_sphere,
                &shaders[i],
            );

            if i == 0 {
                let orbit_radius_moon = 2.0;
                let orbit_speed_moon = 0.01;
                let moon_x =
                    planet_position.x + orbit_radius_moon * (time as f32 * orbit_speed_moon).cos();
                let moon_z =
                    planet_position.z + orbit_radius_moon * (time as f32 * orbit_speed_moon).sin();
                let moon_position = Vec3::new(moon_x, 0.0, moon_z);

                let moon_rotation_speed = 0.005;
                let moon_rotation = time as f32 * moon_rotation_speed;

                let moon_uniforms = Uniforms {
                    model_matrix: create_model_matrix(moon_position, 0.3, moon_rotation),
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time,
                    noise: fastnoise_lite::FastNoiseLite::new(),
                };

                render(
                    &mut framebuffer,
                    &moon_uniforms,
                    &vertex_arrays_moon,
                    &ShaderType::Moon,
                );
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}
