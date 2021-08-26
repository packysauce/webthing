use sauron::{Application, Cmd, Program, html::text, prelude::wasm_bindgen};

pub mod tailwind;

#[derive(Debug)]
pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    count: i32,
}

impl App {
    pub fn new() -> Self {
        App { count: 0 }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl Application<Msg> for App {
    fn update(&mut self, msg: Msg) -> sauron::Cmd<Self, Msg>
    where
        Self: Sized + 'static,
    {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
        Cmd::none()
    }

    fn view(&self) -> sauron::Node<Msg> {
        sauron::node! {
            <main>
                <h1>"im supposed to be unstyled because tailwind"</h1>
                <input type="button"
                    value = "plus"
                    key = "inc"
                    on_click=|_| {
                        Msg::Increment
                    }
                />
                <div class="count">{text(self.count)}</div>
                <input type="button"
                    value = "-"
                    key = "dec"
                    on_click = |_| {
                        Msg::Decrement
                    }
                />
            </main>
        }
    }
}

#[wasm_bindgen(start)]
pub fn wasm_main() {
    Program::mount_to_body(App::new());
}
