pub mod components;
pub mod server;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use components::app::App;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}
