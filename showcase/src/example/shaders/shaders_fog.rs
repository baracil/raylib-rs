/*******************************************************************************************
*
*   raylib [shaders] example - fog
*
*   NOTE: This example requires raylib OpenGL 3.3 or ES2 versions for shaders support,
*         OpenGL 1.1 does not support shaders, recompile raylib to OpenGL 3.3 version.
*
*   NOTE: Shaders used in this example are #version 330 (OpenGL 3.3).
*
*   This example has been created using raylib 2.5 (www.raylib.com)
*   raylib is licensed under an unmodified zlib/libpng license (View raylib.h for details)
*
*   Example contributed by Chris Camacho (@codifies) and reviewed by Ramon Santamaria (@raysan5)
*
*   Chris Camacho (@codifies -  http://bedroomcoders.co.uk/) notes:
*
*   This is based on the PBR lighting example, but greatly simplified to aid learning...
*   actually there is very little of the PBR example left!
*   When I first looked at the bewildering complexity of the PBR example I feared
*   I would never understand how I could do simple lighting with raylib however its
*   a testement to the authors of raylib (including rlights.h) that the example
*   came together fairly quickly.
*
*   Copyright (c) 2019 Chris Camacho (@codifies) and Ramon Santamaria (@raysan5)
*
********************************************************************************************/

use raylib::prelude::*;

use raylib::rlights;

#[cfg(not(target_arch = "wasm32"))]
const GLSL_VERSION: i32 = 330;
#[cfg(target_arch = "wasm32")]
const GLSL_VERSION: i32 = 100;

pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread) -> crate::SampleOut {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    rl.set_window_size(screen_width, screen_height);
    rl.set_window_title(thread, "raylib [shaders] example - fog");

    // Define the camera to look into our 3d world
    let mut camera = Camera3D::perspective(
        rvec3(2.0, 2.0, 6.0), // position
        rvec3(0.0, 0.5, 0.0), // target
        rvec3(0.0, 1.0, 0.0), // up
        45.0,
    ); // fov, type

    // Load models and texture
    let mut modelA = unsafe {
        rl.load_model_from_mesh(
            thread,
            Mesh::gen_mesh_torus(thread, 0.4, 1.0, 16, 32).make_weak(),
        )
        .unwrap()
    };
    let mut modelB = unsafe {
        rl.load_model_from_mesh(
            thread,
            Mesh::gen_mesh_cube(thread, 1.0, 1.0, 1.0).make_weak(),
        )
        .unwrap()
    };
    let mut modelC = unsafe {
        rl.load_model_from_mesh(
            thread,
            Mesh::gen_mesh_sphere(thread, 0.5, 32, 32).make_weak(),
        )
        .unwrap()
    };
    let mut texture = rl
        .load_texture(thread, "original/shaders/resources/texel_checker.png")
        .unwrap();

    // Assign texture to default model material
    modelA.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapType::MAP_ALBEDO as usize]
        .texture = *texture.as_ref();
    modelB.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapType::MAP_ALBEDO as usize]
        .texture = *texture.as_ref();
    modelC.materials_mut()[0].maps_mut()[raylib::consts::MaterialMapType::MAP_ALBEDO as usize]
        .texture = *texture.as_ref();

    // Load shader and set up some uniforms
    let mut shader = rl
        .load_shader(
            thread,
            Some(&format!(
                "original/shaders/resources/shaders/glsl{}/base_lighting.vs",
                GLSL_VERSION
            )),
            Some(&format!(
                "original/shaders/resources/shaders/glsl{}/fog.fs",
                GLSL_VERSION
            )),
        )
        .unwrap();
    shader.locs_mut()[raylib::consts::ShaderLocationIndex::LOC_MATRIX_MODEL as usize] =
        shader.get_shader_location("matModel");
    shader.locs_mut()[raylib::consts::ShaderLocationIndex::LOC_VECTOR_VIEW as usize] =
        shader.get_shader_location("viewPos");

    // Ambient light level
    let ambientLoc = shader.get_shader_location("ambient");
    shader.set_shader_value(ambientLoc, Vector4::new(0.2, 0.2, 0.2, 1.0));

    let mut fogDensity = 0.15;
    let fogDensityLoc = shader.get_shader_location("fogDensity");
    shader.set_shader_value(fogDensityLoc, fogDensity);

    // NOTE: All models share the same shader
    modelA.materials_mut()[0].shader = *shader.as_ref();
    modelB.materials_mut()[0].shader = *shader.as_ref();
    modelC.materials_mut()[0].shader = *shader.as_ref();

    // Using just 1 point lights
    rlights::create_light(
        rlights::LightType::LIGHT_POINT,
        rvec3(0, 2, 6),
        Vector3::zero(),
        Color::WHITE,
        &shader,
    );

    rl.set_camera_mode(&camera, raylib::consts::CameraMode::CAMERA_ORBITAL); // Set an orbital camera mode

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second
                           //--------------------------------------------------------------------------------------

    // Main game loop
    return Box::new(
        move |rl: &mut RaylibHandle, thread: &RaylibThread| -> () // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        rl.update_camera(&mut camera); // Update camera

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_UP)
        {
            fogDensity += 0.001;
            if fogDensity > 1.0
                {fogDensity = 1.0;}
        }

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_DOWN)
        {
            fogDensity -= 0.001;
            if fogDensity < 0.0
                {fogDensity = 0.0;}
        }

        shader.set_shader_value( fogDensityLoc, fogDensity);

        // Rotate the torus
        modelA.set_transform(&(*modelA.transform() * Matrix::rotate_x(-0.025)));
        modelA.set_transform(&(*modelA.transform() * Matrix::rotate_z(0.012)));

        // Update the light shader with the camera view position
        let loc = shader.locs_mut()[raylib::consts::ShaderLocationIndex::LOC_VECTOR_VIEW as usize];
        shader.set_shader_value( loc, camera.position);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::GRAY);
{
        let mut d = d.begin_mode3D(&camera);

        // Draw the three models
        d.draw_model(&modelA, Vector3::zero(), 1.0, Color::WHITE);
        d.draw_model(&modelB, rvec3(-2.6, 0,  0), 1.0, Color::WHITE);
        d.draw_model(&modelC, rvec3(2.6, 0,  0), 1.0, Color::WHITE);

        for  i in (-20..20).step_by(2){

            d.draw_model(&modelA, rvec3(i, 0,  2), 1.0, Color::WHITE);
        }

    }

        d.draw_text(&format!("Use KEY_UP/KEY_DOWN to change fog density [{:.2}]", fogDensity), 10, 10, 20, Color::RAYWHITE);

        //----------------------------------------------------------------------------------
    },
    );

    // // De-Initialization
    // //--------------------------------------------------------------------------------------
    // UnloadModel(modelA);    // Unload the model A
    // UnloadModel(modelB);    // Unload the model B
    // UnloadModel(modelC);    // Unload the model C
    // UnloadTexture(texture); // Unload the texture
    // UnloadShader(shader);   // Unload shader

    // CloseWindow(); // Close window and OpenGL context
    // //--------------------------------------------------------------------------------------

    // return 0;
}
