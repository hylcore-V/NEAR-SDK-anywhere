pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window().document().expect("no global `document` exists")
}

pub fn history() -> web_sys::History {
    window().history().expect("no global `history` exists")
}

pub fn local_storage() -> web_sys::Storage {
    window()
        .local_storage()
        .unwrap()
        .expect("no `localStorage` exists")
}

pub fn location() -> web_sys::Location {
    window().location()
}

pub fn current_url() -> String {
    location().to_string().into()
}

pub fn replace_url(url: &str) {
    let title = document().title();
    history()
        .replace_state_with_url(&js_sys::Map::new(), &title, Some(url))
        .unwrap();
}
