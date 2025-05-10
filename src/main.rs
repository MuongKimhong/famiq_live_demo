mod modal;
use modal::*;

use bevy::prelude::*;
use famiq::prelude::*;

#[derive(Component)]
pub struct RotatingCube;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FamiqPlugin::new_no_camera()))
        .add_systems(Startup, (setup_ui, setup_background))
        .add_systems(Update, (handle_btn_press, rotate_3d_camera))
        .run();
}

fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 8.0),
    ));

    let cube_xyz = vec![
        Vec3::new(2.0, 0.8, 2.0),
        Vec3::new(0.0, 0.8, 0.0),
        Vec3::new(2.0, 0.8, -2.0),
        Vec3::new(-2.0, 0.8, -2.0),
        Vec3::new(-2.0, 0.8, 2.0),
    ];

    for xyz in cube_xyz.into_iter() {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_translation(xyz),
            RotatingCube
        ));
    }
}

fn setup_ui(
    mut fa_query: FaQuery,
    mut famiq_res: ResMut<FamiqResource>,
) {
    fa_query.insert_str("something", "");
    fa_query.insert_str("select", "");
    fa_query.insert_num("counter", 0);
    fa_query.insert_bool("show_modal", false);

    #[cfg(target_arch = "wasm32")]
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).use_style_path("styles.json").inject();

    #[cfg(not(target_arch = "wasm32"))]
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).hot_reload().inject();

    fps!();

    let logo_container = container!(
        class: "sub-container",
        children: [
            image!(path: "bevy_logo.png", class: "mr-5", width: "240px", height: "130px"),
            image!(path: "famiq_logo.png", class: "ml-5", width: "220px", height: "130px")
        ]
    );
    let input_container = container!(
        class: "sub-container",
        children: [
            text_input!(placeholder: "Enter something", class: "input", model: "something"),
            text!(text: "$[something]")
        ]
    );
    let select_container = container!(
        class: "sub-container",
        children: [
            selection!(
                placeholder: "Choose one",
                class: "input",
                model: "select",
                choices: ["meh", "nah", "nope"]
            ),
            text!(text: "$[select]")
        ]
    );
    let counter_buttons = container!(
        class: "sub-container",
        children: [
            button!(text: "Increase", id: "#increase", class: "success-dark mr-2"),
            button!(text: "Decrease", id: "#decrease", class: "warning-dark ml-2")
        ]
    );
    container!(
        id: "#container",
        class: "mx-auto my-auto py-10",
        children: [
            logo_container,
            text!(text: "Welcome to Bevy + Famiq\n What you see are not HTML!", class: "h3 mt-2"),

            circular!(color: "cyan", size: 50.0, class: "mt-2"),

            select_container,
            input_container,

            text!(text: "Counter: $[counter]", class: "h3 mt-2"),
            counter_buttons,

            button!(text: "show modal", class: "large mt-3", id: "#show-modal"),
            text!(text: "modal state $[show_modal]")
        ]
    );
    setup_modal();
}

fn handle_btn_press(
    mut events: EventReader<FaMouseEvent>,
    mut fa_query: FaQuery
) {
    for e in events.read() {
        if let Some(btn_id) = e.button_press() {
            match btn_id.as_str() {
                "#increase" => {
                    let counter = fa_query.get_data_mut("counter").unwrap().as_num_mut();
                    *counter += 1;
                },
                "#decrease" => {
                    let counter = fa_query.get_data_mut("counter").unwrap().as_num_mut();
                    *counter -= 1;
                },
                "#show-modal" => fa_query.mutate_bool("show_modal", true),
                "#close-modal" => fa_query.mutate_bool("show_modal", false),
                _ => {}
            }
        }
    }
}

fn rotate_3d_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<RotatingCube>>
) {
    let speed = std::f32::consts::PI; // 45 degree

    for mut transform in &mut query {
        if transform.translation.x == 0.0 {
            transform.rotate_y(speed * time.delta_secs());
        }
        else if transform.translation.x == -2.0 {
            transform.rotate_z(speed * time.delta_secs());
        }
        else {
            transform.rotate_x(speed * time.delta_secs());
        }
    }
}
