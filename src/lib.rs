use sauron::html::{text, ul};
use sauron::web_sys::Event;
use sauron::{node, InputEvent, Node};
use sauron::{prelude::wasm_bindgen, Application, Cmd, Program};

pub mod tailwind;

pub mod example {
    use macros::Entity;
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Debug, Serialize, Deserialize, Entity)]
    #[entity(id = 1)]
    pub struct Person {
        pub name: String,
        //title: String,
    }

    impl Person {
        pub fn new(name: String) -> Self {
            Self { name }
        }
    }
}

use example::Person;

#[derive(Debug)]
pub enum Msg {
    None,
    SetName(String),
    NewPerson,
}

pub struct App {
    name: String,
    people: Vec<Person>,
}

impl App {
    pub fn new() -> Self {
        App {
            name: "".to_string(),
            people: vec![],
        }
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
            Msg::NewPerson => {
                let name = self.name.clone();
                self.name = "".to_string();
                self.people.push(Person::new(name));
            }
            Msg::SetName(s) => self.name = s,
            Msg::None => {}
        }
        Cmd::none()
    }

    fn view(&self) -> sauron::Node<Msg> {
        let items = self
            .people
            .iter()
            .map(|p| node!(<li>{text(&p.name)}</li>))
            .collect::<Vec<Node<Msg>>>();
        sauron::node! {
          <main>
            <div class="flex bg-gray-100">
                <form on_submit=|e: Event| {
                    e.prevent_default();
                    Msg::NewPerson
                }>
                <input
                    autocomplete="off"
                    name="name"
                    class="border rounded-sm p-1 m-1 focus:outline-none focus:ring-2"
                    type="text"
                    placeholder="Name"
                    on_input=|v: InputEvent| {
                        Msg::SetName(v.value)
                    }>
                </input>
                <button class="focus:outline-none p-1 rounded-md ring-1 bg-blue-100 pulse focus:ring-2 hover:ring-2" type="submit">"Submit"</button>
                </form>
            </div>
            {ul(vec![], items)}
          </main>
        }
    }
}

#[wasm_bindgen(start)]
pub fn wasm_main() {
    Program::mount_to_body(App::new());
}
