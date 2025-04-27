mod modal;
use modal::*;

use bevy::prelude::*;
use famiq::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FamiqPlugin))
        .add_systems(Startup, setup_ui)
        .add_systems(Update, handle_btn_press)
        .run();
}

fn setup_ui(mut fa_query: FaQuery, mut famiq_res: ResMut<FamiqResource>) {
    fa_query.insert_str("something", "");
    fa_query.insert_str("select", "");
    fa_query.insert_num("counter", 0);
    fa_query.insert_bool("show_modal", false);

    #[cfg(target_arch = "wasm32")]
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).use_style_path("styles.json").inject();

    #[cfg(not(target_arch = "wasm32"))]
    FamiqBuilder::new(&mut fa_query, &mut famiq_res).hot_reload().inject();

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
