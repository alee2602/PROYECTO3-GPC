use crate::color::Color;
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::Uniforms;
use nalgebra_glm::{mat4_to_mat3, Mat3, Vec3, Vec4};
use rand::Rng;

#[derive(PartialEq, Debug, Clone)]
pub enum ShaderType {
    GasGiant,
    ColdGasGiant,
    Solar,
    RockyPlanet,
    RockyPlanetVariant,
    AlienPlanet,
    GlacialTextured,
    Moon,
    Spaceship
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

    let transformed =
        uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position =
        Vec4::new(transformed.x / w, transformed.y / w, transformed.z / w, 1.0);

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3
        .transpose()
        .try_inverse()
        .unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal,
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, shader_type: &ShaderType) -> Color {
    match shader_type {
        ShaderType::GasGiant => gas_giant_shader(fragment, uniforms),
        ShaderType::ColdGasGiant => cold_gas_giant_shader(fragment, uniforms),
        ShaderType::Solar => solar_shader(fragment, uniforms),
        ShaderType::RockyPlanet => rocky_planet_shader(fragment, uniforms),
        ShaderType::RockyPlanetVariant => rocky_planet_variant_shader(fragment, uniforms),
        ShaderType::AlienPlanet => alien_planet_shader(fragment, uniforms),
        ShaderType::GlacialTextured => glacial_textured_shader(fragment, uniforms),
        ShaderType::Moon => moon_shader(fragment, uniforms),
        ShaderType::Spaceship => blue_shader(fragment, uniforms)
    }
}

pub fn blue_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let base_blue = Color::new(30, 30, 100); // Azul oscuro base
    let highlight_blue = Color::new(70, 130, 180); // Azul claro para iluminación
    let shadow_black = Color::new(0, 0, 0); // Negro para sombras

    // Gradiente basado en la altura
    let gradient_factor = (fragment.position.y / 10.0).clamp(0.0, 1.0);

    // Oscilación temporal para dinamismo
    let time_factor = ((uniforms.time as f32 * 0.01).sin() * 0.5 + 0.5).clamp(0.0, 1.0);

    // Brillo suave basado en la normal del fragmento
    let brightness = fragment.normal.y.abs().clamp(0.0, 1.0);

    // Mezcla de colores: gradiente, sombras, y oscilaciones dinámicas
    base_blue
        .lerp(&highlight_blue, gradient_factor) // Gradiente altura
        .lerp(&shadow_black, 1.0 - brightness) // Sombras según la normal
        .lerp(&Color::new(50, 50, 100), time_factor * 0.2) // Efecto dinámico
}

pub fn moon_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.001;

    let base_color = Color::new(180, 180, 180);  
    let crater_color = Color::new(100, 100, 100);
    let dust_color = Color::new(150, 150, 150);   

    let craters = uniforms.noise.get_noise_3d(
        position.x * 150.0,
        position.y * 150.0,
        position.z * 150.0,
    ).abs();

    let dust = uniforms.noise.get_noise_3d(
        position.x * 80.0 + time,
        position.y * 80.0,
        position.z * 80.0,
    );

    let surface_details = uniforms.noise.get_noise_3d(
        position.x * 200.0,
        position.y * 200.0,
        position.z * 200.0,
    ).abs();

    let mut final_color = base_color;

    if craters > 0.7 {
        final_color = final_color.lerp(&crater_color, (craters - 0.7) * 2.0);
    }

    final_color = final_color.lerp(&dust_color, dust.abs() * 0.2);

    if surface_details > 0.8 {
        final_color = final_color.lerp(&crater_color, (surface_details - 0.8) * 0.5);
    }

    let light_dir = Vec3::new(0.6, 0.8, 0.4).normalize();
    let normal = position.normalize();
    let lambertian = light_dir.dot(&normal).max(0.0);
    let shading_factor = 0.75 + 0.25 * lambertian;

    final_color = final_color * shading_factor;
    final_color * fragment.intensity
}


pub fn gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let base_colors = [
        Vec3::new(110.0 / 255.0, 0.0 / 255.0, 90.0 / 255.0),
        Vec3::new(160.0 / 255.0, 20.0 / 255.0, 60.0 / 255.0),
        Vec3::new(130.0 / 255.0, 10.0 / 255.0, 80.0 / 255.0),
        Vec3::new(180.0 / 255.0, 40.0 / 255.0, 90.0 / 255.0),
        Vec3::new(140.0 / 255.0, 10.0 / 255.0, 70.0 / 255.0),
    ];

    let time = uniforms.time as f32 * 0.001;
    let dynamic_y = fragment.vertex_position.y + time;

    let distortion_scale = 10.0;
    let distortion_value = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * distortion_scale,
        dynamic_y * distortion_scale,
    );

    let distorted_y = dynamic_y + distortion_value * 0.1 + fragment.vertex_position.x * 0.05;

    let band_frequency = 40.0;
    let band_sine = (distorted_y * band_frequency).sin();
    let band_variation = (fragment.vertex_position.y * 10.0).sin() * 0.3;
    let band_index_float = (band_sine + band_variation + 1.0) / 2.0 * (base_colors.len() as f32);
    let band_index = band_index_float as usize % base_colors.len();
    let mut rng = rand::thread_rng();
    let random_offset: f32 = rng.gen_range(-0.03..0.03);
    let base_band_color =
        base_colors[band_index] + Vec3::new(random_offset, random_offset, random_offset);

    // Aumentar la saturación de algunas bandas de forma aleatoria
    let saturation_boost: f32 = if rng.gen_bool(0.5) { 1.2 } else { 1.0 };
    let boosted_band_color = base_band_color * saturation_boost;

    // Se elige el siguiente color de banda para suavizar la transición
    let next_band_index = (band_index + 1) % base_colors.len();
    let next_band_color =
        base_colors[next_band_index] + Vec3::new(random_offset, random_offset, random_offset);

    // Interpolación suave entre colores adyacentes
    let interpolation_factor = band_index_float.fract();
    let interpolated_color = boosted_band_color.lerp(&next_band_color, interpolation_factor);

    // capas de ruido de alta frecuencia para dar más textura a las bandas
    let noise_scale_1 = 80.0;
    let noise_value_1 = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * noise_scale_1,
        fragment.vertex_position.y * noise_scale_1,
    );

    let noise_scale_2 = 40.0;
    let noise_value_2 = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * noise_scale_2,
        fragment.vertex_position.y * noise_scale_2,
    );

    let perturbed_color = interpolated_color * (0.95 + (noise_value_1 + noise_value_2) * 0.015);

    let internal_shadow = (distorted_y * band_frequency * 0.1).sin().abs() * 0.15;
    let shaded_color = perturbed_color * (1.0 - internal_shadow);

    let shadow_noise_scale = 50.0;
    let shadow_noise = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * shadow_noise_scale,
        fragment.vertex_position.y * shadow_noise_scale,
    );
    let shadow_variation = 1.0 - shadow_noise * 0.05;
    let final_shaded_color = shaded_color * shadow_variation;
    let spot_noise_scale = 25.0;
    let spot_noise = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * spot_noise_scale,
        fragment.vertex_position.y * spot_noise_scale,
    );

    let mut final_color;

    if spot_noise > 0.75 {
        let mix_factor = (spot_noise - 0.75) / 0.25;
        let storm_color = Vec3::new(0.95, 0.85, 0.65);
        final_color = final_shaded_color.lerp(&storm_color, mix_factor);
    } else {
        final_color = final_shaded_color;
    }

    let normal = fragment.vertex_position.normalize();

    let light_dir = Vec3::new(0.6, 0.8, 0.4).normalize();
    let lambertian = light_dir.dot(&normal).max(0.0);
    let shading_factor = 0.75 + 0.25 * lambertian;

    final_color = final_color * shading_factor;

    // dispersión atmosférica
    let gradient_shading = 1.0 - (fragment.vertex_position.y.abs() * 0.15);
    final_color = final_color * gradient_shading;

    // reflejos especulares para simular brillos en la atmósfera
    let view_dir = Vec3::new(0.0, 0.0, 1.0).normalize();
    let reflect_dir = (2.0 * normal.dot(&light_dir) * normal - light_dir).normalize();
    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(10.0);

    final_color = final_color + Vec3::new(1.0, 1.0, 1.0) * specular_intensity * 0.15;

    final_color = final_color * fragment.intensity;

    Color::new(
        (final_color.x * 255.0) as u8,
        (final_color.y * 255.0) as u8,
        (final_color.z * 255.0) as u8,
    )
}

pub fn cold_gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let base_colors = [
        Vec3::new(100.0 / 255.0, 150.0 / 255.0, 180.0 / 255.0),
        Vec3::new(120.0 / 255.0, 180.0 / 255.0, 200.0 / 255.0),
        Vec3::new(90.0 / 255.0, 140.0 / 255.0, 170.0 / 255.0),
        Vec3::new(130.0 / 255.0, 190.0 / 255.0, 210.0 / 255.0),
        Vec3::new(80.0 / 255.0, 120.0 / 255.0, 160.0 / 255.0),
    ];

    let time = uniforms.time as f32 * 0.001;
    let dynamic_y = fragment.vertex_position.y + time;

    let distortion_scale = 10.0;
    let distortion_value = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * distortion_scale,
        dynamic_y * distortion_scale,
    );

    let wind_tilt = fragment.vertex_position.x * 0.02;
    let distorted_y =
        dynamic_y + wind_tilt + distortion_value * 0.1 + fragment.vertex_position.x * 0.05;

    let band_frequency = 40.0;
    let band_sine = (distorted_y * band_frequency).sin();
    let band_variation = (fragment.vertex_position.y * 10.0).sin() * 0.3;
    let band_index_float = (band_sine + band_variation + 1.0) / 2.0 * (base_colors.len() as f32);
    let band_index = band_index_float as usize % base_colors.len();
    let mut rng = rand::thread_rng();
    let random_offset: f32 = rng.gen_range(-0.03..0.03);
    let base_band_color =
        base_colors[band_index] + Vec3::new(random_offset, random_offset, random_offset);

    let saturation_boost: f32 = if rng.gen_bool(0.5) { 1.2 } else { 1.0 };
    let boosted_band_color = base_band_color * saturation_boost;

    let next_band_index = (band_index + 1) % base_colors.len();
    let next_band_color =
        base_colors[next_band_index] + Vec3::new(random_offset, random_offset, random_offset);

    let interpolation_factor = band_index_float.fract();
    let interpolated_color = boosted_band_color.lerp(&next_band_color, interpolation_factor);

    let noise_scale_1 = 80.0;
    let noise_value_1 = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * noise_scale_1,
        fragment.vertex_position.y * noise_scale_1,
    );

    let noise_scale_2 = 40.0;
    let noise_value_2 = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * noise_scale_2,
        fragment.vertex_position.y * noise_scale_2,
    );

    let perturbed_color = interpolated_color * (0.95 + (noise_value_1 + noise_value_2) * 0.015);

    let internal_shadow = (distorted_y * band_frequency * 0.1).sin().abs() * 0.15;
    let shaded_color = perturbed_color * (1.0 - internal_shadow);

    let shadow_noise_scale = 50.0;
    let shadow_noise = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * shadow_noise_scale,
        fragment.vertex_position.y * shadow_noise_scale,
    );
    let shadow_variation = 1.0 - shadow_noise * 0.05;
    let final_shaded_color = shaded_color * shadow_variation;

    let spot_noise_scale = 15.0;
    let spot_noise = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * spot_noise_scale,
        fragment.vertex_position.y * spot_noise_scale,
    );

    let mut final_color;

    if spot_noise > 0.7 {
        let mix_factor = (spot_noise - 0.7) / 0.3;
        let storm_color = Vec3::new(0.75, 0.85, 0.95);
        final_color = final_shaded_color.lerp(&storm_color, mix_factor);
    } else {
        final_color = final_shaded_color;
    }

    let normal = fragment.vertex_position.normalize();

    let light_dir = Vec3::new(0.6, 0.8, 0.4).normalize();
    let lambertian = light_dir.dot(&normal).max(0.0);
    let shading_factor = 0.75 + 0.25 * lambertian;
    final_color = final_color * shading_factor;

    let gradient_shading = 1.0 - (fragment.vertex_position.y.abs() * 0.15);
    final_color = final_color * gradient_shading;

    let view_dir = Vec3::new(0.0, 0.0, 1.0).normalize();
    let reflect_dir = (2.0 * normal.dot(&light_dir) * normal - light_dir).normalize();
    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(10.0);
    final_color = final_color + Vec3::new(1.0, 1.0, 1.0) * specular_intensity * 0.15;

    final_color = final_color * fragment.intensity;

    Color::new(
        (final_color.x * 255.0) as u8,
        (final_color.y * 255.0) as u8,
        (final_color.z * 255.0) as u8,
    )
}

pub fn solar_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let bright_color = Color::new(255, 240, 70);
    let mid_color = Color::new(255, 100, 0);
    let dark_color = Color::new(70, 10, 0);

    let position = Vec3::new(
        fragment.vertex_position.x,
        fragment.vertex_position.y,
        fragment.depth,
    );

    let base_frequency = 0.04 + position.x * 0.01;
    let pulsate_amplitude = 0.6 + position.y * 0.02;
    let t = uniforms.time as f32 * 0.02;

    let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

    let zoom = 1500.0;

    // Obtener ruido en 3D para generar las manchas solares
    let noise_value1 = uniforms.noise.get_noise_3d(
        position.x * zoom,
        position.y * zoom,
        (position.z + pulsate) * zoom,
    );
    let noise_value2 = uniforms.noise.get_noise_3d(
        (position.x + 300.0) * zoom,
        (position.y + 300.0) * zoom,
        (position.z + 300.0 + pulsate) * zoom,
    );

    let noise_value = (noise_value1 + noise_value2) * 0.5;

    let fine_noise = uniforms.noise.get_noise_3d(
        position.x * 500.0,
        position.y * 500.0,
        (position.z + pulsate) * 500.0,
    );

    let adjusted_noise = (noise_value + fine_noise * 0.6) * 1.8 - 0.4;

    let high_freq_noise = uniforms.noise.get_noise_3d(
        position.x * 2000.0,
        position.y * 2000.0,
        (position.z + pulsate) * 2000.0,
    ) * 0.03;

    let bands_pattern1 = (position.y * 6.0 + noise_value * 25.0 + t * 0.15).sin() * 0.2;
    let bands_pattern2 = (position.y * 10.0 + noise_value * 50.0 + t * 0.08).sin() * 0.1;

    let combined_bands = bands_pattern1 + bands_pattern2 + high_freq_noise;

    let color = if adjusted_noise + combined_bands > 0.4 {
        mid_color.lerp(&bright_color, adjusted_noise + combined_bands - 0.4)
    } else {
        dark_color.lerp(&mid_color, (adjusted_noise + combined_bands) * 2.5)
    };

    let pulse_effect = 1.0 + 0.15 * ((t * 1.5 + position.x * 0.05).sin());
    let final_color = color * pulse_effect;

    final_color * fragment.intensity
}

pub fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let bright_color = Color::new(230, 120, 70);
    let mid_color = Color::new(140, 70, 40);
    let dark_color = Color::new(30, 10, 5);

    let position = Vec3::new(
        fragment.vertex_position.x,
        fragment.vertex_position.y,
        fragment.depth,
    );

    let zoom = 1200.0;

    // Obtener ruido para la superficie rocosa
    let noise_value1 =
        uniforms
            .noise
            .get_noise_3d(position.x * zoom, position.y * zoom, position.z * zoom);

    let noise_value2 = uniforms.noise.get_noise_3d(
        (position.x + 400.0) * zoom,
        (position.y + 400.0) * zoom,
        (position.z + 400.0) * zoom,
    );

    let noise_value = (noise_value1 + noise_value2) * 0.5;

    let crater_frequency = 1.5;
    let crater_amplitude = 2.0;
    let crater_value = (position.x * crater_frequency + position.y * crater_frequency).sin()
        * (position.x * crater_frequency - position.y * crater_frequency).cos()
        * crater_amplitude;

    let mut combined_value = (noise_value + crater_value).clamp(0.0, 1.0);

    let fine_noise = uniforms.noise.get_noise_3d(
        position.x * 1600.0,
        position.y * 1600.0,
        position.z * 1600.0,
    ) * 0.3;

    combined_value = (combined_value + fine_noise).clamp(0.0, 1.0);

    let fracture_noise = uniforms.noise.get_noise_3d(
        position.x * 2000.0,
        position.y * 2000.0,
        position.z * 2000.0,
    ) * 0.15;
    combined_value = (combined_value + fracture_noise).clamp(0.0, 1.0);

    let color = if combined_value > 0.5 {
        mid_color.lerp(&bright_color, (combined_value - 0.5) * 1.5)
    } else {
        dark_color.lerp(&mid_color, combined_value * 2.0)
    };

    let light_factor = (position.y * 0.5 + uniforms.time as f32 * 0.0015).sin() * 0.1 + 1.0;
    let directional_light = (position.x * 0.3 + uniforms.time as f32 * 0.002).cos() * 0.05 + 1.0;
    let final_light_factor = light_factor * directional_light;
    let mut final_color = color * final_light_factor;

    let pulsate_frequency = 0.06;
    let pulsate_amplitude = 0.1;
    let pulsate =
        (uniforms.time as f32 * pulsate_frequency + position.x * 0.02 + position.y * 0.02).sin()
            * pulsate_amplitude;
    final_color = final_color * (1.0 + pulsate);

    let shadow_texture_noise = uniforms.noise.get_noise_3d(
        position.x * 2500.0,
        position.y * 2500.0,
        position.z * 2500.0,
    ) * 0.3;
    final_color = final_color * (1.0 + shadow_texture_noise);

    let highlight_texture_noise = uniforms.noise.get_noise_3d(
        position.x * 3000.0,
        position.y * 3000.0,
        position.z * 3000.0,
    ) * 0.25;
    final_color = final_color * (1.0 + highlight_texture_noise);

    let depth_variation = uniforms.noise.get_noise_3d(
        position.x * 3500.0,
        position.y * 3500.0,
        position.z * 3500.0,
    ) * 0.1;
    final_color = final_color * (1.0 + depth_variation);

    final_color * fragment.intensity
}

pub fn rocky_planet_variant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let bright_color = Color::new(237, 201, 175);  
    let mid_color = Color::new(193, 154, 107);  
    let dark_color = Color::new(139, 108, 66);  


    let position = Vec3::new(
        fragment.vertex_position.x,
        fragment.vertex_position.y,
        fragment.depth,
    );

    let zoom = 1000.0;

    // Obtener ruido para la superficie rocosa
    let noise_value1 =
        uniforms
            .noise
            .get_noise_3d(position.x * zoom, position.y * zoom, position.z * zoom);

    let noise_value2 = uniforms.noise.get_noise_3d(
        (position.x + 400.0) * zoom,
        (position.y + 400.0) * zoom,
        (position.z + 400.0) * zoom,
    );

    let noise_value = (noise_value1 + noise_value2) * 0.5;

    let crater_frequency = 1.5;
    let crater_amplitude = 2.0;
    let crater_value = (position.x * crater_frequency + position.y * crater_frequency).sin()
        * (position.x * crater_frequency - position.y * crater_frequency).cos()
        * crater_amplitude;

    let mut combined_value = (noise_value + crater_value).clamp(0.0, 1.0);

    let fine_noise = uniforms.noise.get_noise_3d(
        position.x * 1600.0,
        position.y * 1600.0,
        position.z * 1600.0,
    ) * 0.3;

    combined_value = (combined_value + fine_noise).clamp(0.0, 1.0);

    let fracture_noise = uniforms.noise.get_noise_3d(
        position.x * 2000.0,
        position.y * 2000.0,
        position.z * 2000.0,
    ) * 0.15;
    combined_value = (combined_value + fracture_noise).clamp(0.0, 1.0);

    let color = if combined_value > 0.5 {
        mid_color.lerp(&bright_color, (combined_value - 0.5) * 1.5)
    } else {
        dark_color.lerp(&mid_color, combined_value * 2.0)
    };

    let light_factor = (position.y * 0.5 + uniforms.time as f32 * 0.0015).sin() * 0.1 + 1.0;
    let directional_light = (position.x * 0.3 + uniforms.time as f32 * 0.002).cos() * 0.05 + 1.0;
    let final_light_factor = light_factor * directional_light;
    let mut final_color = color * final_light_factor;

    let pulsate_frequency = 0.04;
    let pulsate_amplitude = 0.08;
    let pulsate =
        (uniforms.time as f32 * pulsate_frequency + position.x * 0.02 + position.y * 0.02).sin()
            * pulsate_amplitude;
    final_color = final_color * (1.0 + pulsate);

    let shadow_texture_noise = uniforms.noise.get_noise_3d(
        position.x * 2500.0,
        position.y * 2500.0,
        position.z * 2500.0,
    ) * 0.3;
    final_color = final_color * (1.0 + shadow_texture_noise);

    let highlight_texture_noise = uniforms.noise.get_noise_3d(
        position.x * 3000.0,
        position.y * 3000.0,
        position.z * 3000.0,
    ) * 0.25;
    final_color = final_color * (1.0 + highlight_texture_noise);

    let depth_variation = uniforms.noise.get_noise_3d(
        position.x * 3500.0,
        position.y * 3500.0,
        position.z * 3500.0,
    ) * 0.1;
    final_color = final_color * (1.0 + depth_variation);

    final_color * fragment.intensity
}

pub fn alien_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let ocean_color = Color::new(25, 25, 112);
    let flora_color = Color::new(110, 62, 136);
    let alien_color = Color::new(13, 246, 243);

    let position = Vec3::new(
        fragment.vertex_position.x,
        fragment.vertex_position.y,
        fragment.depth,
    );
    let zoom = 450.0;

    let time_factor = uniforms.time as f32 * 0.15;

    let noise_value1 = uniforms.noise.get_noise_3d(
        position.x * zoom + time_factor,
        position.y * zoom + time_factor,
        position.z * zoom + time_factor,
    );

    let noise_value2 = uniforms.noise.get_noise_3d(
        (position.x + 300.0) * zoom + time_factor,
        (position.y + 300.0) * zoom + time_factor,
        (position.z + 300.0) * zoom + time_factor,
    );

    let noise_value = (noise_value1 + noise_value2) * 0.5;

    let drift_noise = uniforms.noise.get_noise_3d(
        position.x * 0.05 + time_factor,
        position.y * 0.05 + time_factor,
        position.z * 0.05 + time_factor,
    );

    let combined_value = (noise_value + drift_noise * 0.4).clamp(0.0, 1.0);

    let base_color = if combined_value > 0.75 {
        alien_color
    } else if combined_value > 0.4 {
        flora_color
    } else {
        ocean_color
    };

    let texture_zoom1 = 700.0;
    let texture_noise1 = uniforms.noise.get_noise_3d(
        position.x * texture_zoom1,
        position.y * texture_zoom1,
        position.z * texture_zoom1,
    ) * 0.3;

    let texture_zoom2 = 1000.0;
    let texture_noise2 = uniforms.noise.get_noise_3d(
        position.x * texture_zoom2,
        position.y * texture_zoom2,
        position.z * texture_zoom2,
    ) * 0.25;

    let texture_zoom3 = 1500.0;
    let texture_noise3 = uniforms.noise.get_noise_3d(
        position.x * texture_zoom3,
        position.y * texture_zoom3,
        position.z * texture_zoom3,
    ) * 0.2;

    let texture_zoom4 = 2000.0;
    let texture_noise4 = uniforms.noise.get_noise_3d(
        position.x * texture_zoom4,
        position.y * texture_zoom4,
        position.z * texture_zoom4,
    ) * 0.15;

    let background_noise1 = uniforms.noise.get_noise_3d(
        position.x * 2500.0,
        position.y * 2500.0,
        position.z * 2500.0,
    ) * 0.15;

    let background_noise2 = uniforms.noise.get_noise_3d(
        position.x * 3500.0,
        position.y * 3500.0,
        position.z * 3500.0,
    ) * 0.1;

    let texture_combined = (texture_noise1
        + texture_noise2
        + texture_noise3
        + texture_noise4
        + background_noise1
        + background_noise2)
        .clamp(0.0, 1.0);

    let texturized_color = base_color * (1.0 + texture_combined);

    let limited_texturized_color = texturized_color.limit_min(50);

    let light_factor = (position.y * 0.5 + uniforms.time as f32 * 0.001).sin() * 0.2 + 1.0;
    let directional_light = (position.x * 0.4 + uniforms.time as f32 * 0.0015).cos() * 0.2 + 1.0;
    let final_light_factor = light_factor * directional_light;

    let illuminated_color = limited_texturized_color * final_light_factor;

    let final_color = illuminated_color.limit_min(50);

    final_color * fragment.intensity
}

pub fn glacial_textured_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let ice_blue = Color::new(173, 216, 230);  

    let position = Vec3::new(
        fragment.vertex_position.x,
        fragment.vertex_position.y,
        fragment.depth,
    );

    let zoom = 100.0;

    let time_factor = uniforms.time as f32 * 0.1;

    let base_noise = uniforms.noise.get_noise_3d(
        position.x * zoom,
        position.y * zoom,
        position.z * zoom,
    ) * 0.6;

    let detail_noise1 = uniforms.noise.get_noise_3d(
        position.x * 700.0,
        position.y * 700.0,
        position.z * 700.0,
    ) * 0.5;

    let detail_noise2 = uniforms.noise.get_noise_3d(
        position.x * 1200.0 + time_factor,
        position.y * 1200.0 + time_factor,
        position.z * 1200.0 + time_factor,
    ) * 0.4;

    let fine_detail_noise = uniforms.noise.get_noise_3d(
        position.x * 2500.0,
        position.y * 2500.0,
        position.z * 2500.0,
    ) * 0.3;

    let combined_texture = (base_noise + detail_noise1 + detail_noise2 + fine_detail_noise).clamp(0.0, 1.0);

    let texturized_color = ice_blue * (1.0 + combined_texture);

    let flicker_effect = (position.x * 0.05 + uniforms.time as f32 * 0.005).sin() * 0.1 + 0.9;
    let flicker_light = (position.y * 0.03 + uniforms.time as f32 * 0.007).cos() * 0.1 + 0.95;
    let final_flicker_factor = flicker_effect * flicker_light;

    let illuminated_color = texturized_color * final_flicker_factor;

    let final_color = illuminated_color.limit_min(60);

    final_color * fragment.intensity
}