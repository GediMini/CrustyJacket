use bevy::prelude::*;
use bevy_mod_picking::*;

fn convert_position_zup_to_yup(position: Vec3) -> Vec3 {
    Vec3::new(position.x, position.z, -position.y)
}

// fn convert_rotation_zup_to_yup(rotation: Quat) -> Quat {
//     // Assuming the rotation is in Euler angles (pitch, yaw, roll)
//     let (pitch, yaw, roll) = rotation.to_euler();
//     Quat::from_euler(EulerRot::YXZ, -roll, pitch, -yaw)
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        // .add_startup_system(setup_ui.system())
        .add_startup_system(setup)
        .add_system(pan_camera)
        .add_system(rotate_camera)
        .add_system(print_picked_mesh)
        // .add_system(update_ui_text.system())
        .run();
}

fn create_cylinder(
    commands: &mut Commands,
    start: Vec3,
    end: Vec3,
    radius: f32,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
) {
    // Calculate the height and center point of the cylinder
    let height = (end - start).length();
    let center = (start + end) / 2.0;

    // Calculate the rotation quaternion to align the cylinder with the desired axis
    let direction = (end - start).normalize();
    let rotation = Quat::from_rotation_arc(Vec3::Y, direction);

    // Create the cylinder in Bevy with the calculated height, center point, and rotation
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform {
                translation: center,
                rotation,
                scale: Vec3::new(radius, height / 2.0, radius),
            },
            ..Default::default()
        },
        PickableBundle::default(),
    ));
}

fn pan_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut movement = Vec3::ZERO;

    let pan_speed = 2.0;
    let delta = time.delta_seconds() * pan_speed;

    if keyboard_input.pressed(KeyCode::W) {
        movement -= Vec3::Z * delta;
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement += Vec3::Z * delta;
    }
    if keyboard_input.pressed(KeyCode::A) {
        movement -= Vec3::X * delta;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement += Vec3::X * delta;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        movement -= Vec3::Y * delta;
    }
    if keyboard_input.pressed(KeyCode::E) {
        movement += Vec3::Y * delta;
    }

    if movement != Vec3::ZERO {
        for mut transform in query.iter_mut() {
            // Convert the movement vector to global space by multiplying it with the camera's rotation
            let global_movement = transform.rotation * movement;
            transform.translation += global_movement;
        }
    }
}

// fn pan_camera(
//     time: Res<Time>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<&mut Transform, With<Camera>>,
// ) {
//     let mut movement = Vec3::ZERO;

//     let pan_speed = 2.0;
//     let delta = time.delta_seconds() * pan_speed;

//     if keyboard_input.pressed(KeyCode::W) {
//         movement -= Vec3::Z * delta;
//     }
//     if keyboard_input.pressed(KeyCode::S) {
//         movement += Vec3::Z * delta;
//     }
//     if keyboard_input.pressed(KeyCode::A) {
//         movement -= Vec3::X * delta;
//     }
//     if keyboard_input.pressed(KeyCode::D) {
//         movement += Vec3::X * delta;
//     }
//     if keyboard_input.pressed(KeyCode::Q) {
//         movement += Vec3::Y * delta;
//     }
//     if keyboard_input.pressed(KeyCode::E) {
//         movement -= Vec3::Y * delta;
//     }

//     if movement != Vec3::ZERO {
//         for mut transform in query.iter_mut() {
//             transform.translation += movement;
//         }
//     }
// }

fn rotate_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let rotation_speed = 45.0; // degrees per second
    let delta = time.delta_seconds() * rotation_speed;
    let shift_pressed =
        keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift);

    let mut rotation_x = Quat::IDENTITY;
    let mut rotation_y = Quat::IDENTITY;
    let mut rotation_z = Quat::IDENTITY;

    if keyboard_input.pressed(KeyCode::X) {
        rotation_x = if shift_pressed {
            Quat::from_rotation_x(-delta.to_radians())
        } else {
            Quat::from_rotation_x(delta.to_radians())
        };
    }
    if keyboard_input.pressed(KeyCode::Y) {
        rotation_y = if shift_pressed {
            Quat::from_rotation_y(-delta.to_radians())
        } else {
            Quat::from_rotation_y(delta.to_radians())
        };
    }
    if keyboard_input.pressed(KeyCode::Z) {
        rotation_z = if shift_pressed {
            Quat::from_rotation_z(-delta.to_radians())
        } else {
            Quat::from_rotation_z(delta.to_radians())
        };
    }

    let rotation = rotation_x * rotation_y * rotation_z;

    if rotation != Quat::IDENTITY {
        for mut transform in query.iter_mut() {
            transform.rotation = rotation * transform.rotation;
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(15.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // // cylinder
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cylinder {
    //         radius: 0.5,
    //         height: 2.0,
    //         segments: 10,
    //         resolution: 32,
    //     })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 1.0, 0.0),
    //     ..default()
    // });

    // Cylinders
    // List of cylinder positions in ROSAP z-up coordinate system
    let cylinder_positions = vec![
        (Vec3::new(-3.0, 0.0, 0.0), Vec3::new(-2.0, 0.0, 4.0)),
        (Vec3::new(-2.0, 0.0, 4.0), Vec3::new(-1.0, 0.0, 9.0)),
        (Vec3::new(3.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 4.0)),
        (Vec3::new(2.0, 0.0, 4.0), Vec3::new(1.0, 0.0, 9.0)),
        (Vec3::new(-1.0, 0.0, 9.0), Vec3::new(1.0, 0.0, 9.0)),
        (Vec3::new(-2.0, 0.0, 4.0), Vec3::new(2.0, 0.0, 4.0)),
        (Vec3::new(-2.0, 0.0, 4.0), Vec3::new(2.0, 0.0, 4.0)),
        (Vec3::new(-3.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 2.0)),
        (Vec3::new(0.0, 0.0, 2.0), Vec3::new(2.0, 0.0, 4.0)),
        (Vec3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 2.0)),
        (Vec3::new(0.0, 0.0, 2.0), Vec3::new(-2.0, 0.0, 4.0)),
        // ... add more positions as needed
    ];

    // Generate a cylinder mesh and material to reuse for all cylinders
    let cylinder_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: 0.5,
        height: 2.0,
        segments: 10,
        resolution: 32,
    }));
    let cylinder_material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    for &(start, end) in &cylinder_positions {
        let start_coor = convert_position_zup_to_yup(start);
        let end_coor = convert_position_zup_to_yup(end);
        create_cylinder(
            &mut commands,
            start_coor,
            end_coor,
            0.3,
            &cylinder_mesh,
            &cylinder_material,
        );
        // commands.spawn(PbrBundle {
        //     mesh: cylinder_mesh.clone(),
        //     material: cylinder_material.clone(),
        //     transform: Transform::from_xyz(position.x, position.y, position.z),
        //     ..Default::default()
        // });
    }

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 20.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PickingCameraBundle::default(),
    ));
}

fn print_picked_mesh(mut events: EventReader<PickingEvent>) {
    // fn print_picked_mesh(mut events: EventReader<PickingEvent>, mut query: Query<&mut PickingMessage>) {
    // let mut message = query.single_mut().unwrap();

    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => info!("Gee Willikers, it's a click! {:?}", e),
            // PickingEvent::Selection(e) => {
            //     message.text = format!("A selection event happened: {:?}", e);
            // }
            // PickingEvent::Hover(e) => {
            //     message.text = format!("Egads! A hover event!? {:?}", e);
            // }
            // PickingEvent::Clicked(e) => {
            //     message.text = format!("Gee Willikers, it's a click! {:?}", e);
            // }
        }
    }
}

// struct PickingMessage {
//     text: String,
// }

// fn setup_ui(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     // Load the font
//     let font = asset_server.load("fonts/FiraSans-Bold.ttf");

//     // Spawn the UI camera
//     commands.spawn_bundle(UiCameraBundle::default());

//     // Spawn the text widget
//     commands
//         .spawn_bundle(TextBundle {
//             style: Style {
//                 align_self: AlignSelf::FlexEnd,
//                 ..Default::default()
//             },
//             text: Text {
//                 sections: vec![TextSection {
//                     value: "".to_string(),
//                     style: TextStyle {
//                         font: font.clone(),
//                         font_size: 24.0,
//                         color: Color::WHITE,
//                     },
//                 }],
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .insert(PickingMessage {
//             text: "".to_string(),
//         }); // Add the PickingMessage component
// }

// fn update_ui_text(mut query: Query<(&PickingMessage, &mut Text)>) {
//     if let Ok((message, mut ui_text)) = query.single_mut() {
//         ui_text.sections[0].value = message.text.clone();
//     }
// }
