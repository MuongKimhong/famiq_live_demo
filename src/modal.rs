use super::*;

pub fn setup_modal(){
    modal!(
        model: "show_modal",
        children: [
            container!(
                id: "#modal-container",
                class: "mx-auto my-auto",
                children: [
                    text!(text: "Hello from modal", class: "h1"),
                    button!(text: "Close", class: "primary-dark", id: "#close-modal")
                ]
            )
        ]
    );
}
