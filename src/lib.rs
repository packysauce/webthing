use sauron::prelude::*;

pub mod tailwind;

pub mod daisy {
    use sauron::{
        node,
        prelude::{text, Value},
        MouseEvent, Node,
    };
    use serde::{Deserialize, Serialize};
    use strum::IntoEnumIterator;

    #[derive(Clone, Copy, Debug)]
    pub enum Msg {
        ChangeTheme(Theme),
    }

    #[derive(
        Clone, Copy, strum::Display, strum::AsRefStr, strum::EnumIter, Debug, Serialize, Deserialize,
    )]
    #[strum(serialize_all = "lowercase")]
    pub enum Theme {
        Light,
        Dark,
        Cupcake,
        Bumblebee,
        Emerald,
        Corporate,
        Synthwave,
        Retro,
        Cyberpunk,
        Valentine,
        Halloween,
        Garden,
        Forest,
        Aqua,
        Lofi,
        Pastel,
        Fantasy,
        Wireframe,
        Black,
        Luxury,
        Dracula,
    }

    impl From<Theme> for Value {
        fn from(t: Theme) -> Self {
            Self::String(t.to_string())
        }
    }

    pub struct ThemeSelector;

    pub type MouseClickHandler<Msg> = dyn Fn((&MouseEvent,)) -> Msg;

    impl Theme {
        pub fn dropdown<Msg>() -> Node<Msg>
        where
            Msg: From<self::Msg> + 'static,
        {
            node! {
                <div class="dropdown sm:dropdown-left dropdown-end overflow-visible">
                    <div tabindex=0 class="m-1 btn">"Theme"</div>
                    <div tabindex=0 class="p-2 artboard shadow max-h-60 overflow-scroll dropdown-content bg-base-100 rounded-md w-64">
                    {
                        for v in Self::iter() {
                            node! {
                                <div class="flex flex-col w-full text-base-content bg-base-100" data-theme={v}>
                                    <a on_click=move |_| {
                                        self::Msg::ChangeTheme(v).into()
                                    }>
                                        <h1 class="text-md">{text(v)}</h1>
                                    </a>
                                </div>
                            }
                        }
                    }
                    </div>
                </div>
            }
        }
    }
}

pub mod example {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;

    use super::daisy;
    use macros::{Assoc, Entity};
    use sauron::html::text;
    use sauron::node;
    use serde::Deserialize;
    use serde::Serialize;
    use wtf::PersistedState;
    use wtf::RawAssoc;

    #[derive(Debug, Serialize, Deserialize, Entity)]
    #[entity(id = 1)]
    pub struct Person {
        pub name: String,
        pub theme: daisy::Theme,
        pic_url: Option<String>,
    }

    #[derive(Assoc, Debug)]
    #[assoc(id = 10)]
    pub struct ProfilePic<S: PersistedState>(RawAssoc, S);

    impl Person {
        pub fn new(name: &str, theme: daisy::Theme) -> Self {
            let mut hasher = DefaultHasher::new();
            name.hash(&mut hasher);
            let pic_idx = (hasher.finish() % 70) + 1;
            let pic_url = if pic_idx == 65 ||  pic_idx == 45 {
                None
            } else {
                Some(format!("https://i.pravatar.cc/512?img={}", pic_idx))
            };
            Self {
                name: name.to_string(),
                theme,
                pic_url,
            }
        }

        pub fn card<MSG>(&self) -> sauron::Node<MSG> {
            let classes = if self.pic_url.is_some() {
                "avatar"
            } else {
                "avatar placeholder"
            };
            node! {
                <div class="flex max-w-sm" data-theme=self.theme>
                    <div class="flex-1 card shadow bordered bg-base-100 text-base-content">
                        <div class="card-body">
                            <div class=classes>
                            {
                                if let Some(ref url) = self.pic_url {
                                    node! {
                                    <div class="mb-8 rounded-full w-24 h-24">
                                        <img src=url></img>
                                    </div>
                                    }
                                } else {
                                    node! {
                                    <div class="mb-8 bg-neutral-focus text-neutral-content rounded-full w-24 h-24">
                                        <span class="text-xl">{text(&self.name.chars().next().unwrap().to_string())}</span>
                                    </div>
                                    }
                                }
                            }
                            </div>
                            <h2 class="card-title">{text(&self.name)}</h2>
                        </div>
                    </div>
                </div>
            }
        }
    }
}

use example::Person;

use crate::daisy::Theme;

#[derive(Debug, derive_more::From)]
pub enum Msg {
    None,
    ThemeMsg(daisy::Msg),
    SetName(String),
    NewPerson,
}

pub struct App {
    theme: daisy::Theme,
    name: String,
    people: Vec<Person>,
}

impl App {
    pub fn new() -> Self {
        App {
            theme: Theme::Dark,
            name: "".to_string(),
            people: vec![
                Person::new("Detective Popcorn", Theme::Cyberpunk),
                Person::new("Blaze", Theme::Lofi),
                Person::new("Lazer", Theme::Aqua),
                Person::new("Blazer", Theme::Synthwave),
            ],
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
            Msg::ThemeMsg(daisy::Msg::ChangeTheme(theme)) => {
                let doc = document();
                let r = doc.document_element().unwrap();
                r.set_attribute("data-theme", theme.as_ref()).unwrap();
                self.theme = theme
            }
            Msg::NewPerson => {
                let name = self.name.clone();
                self.name = "".to_string();
                self.people.push(Person::new(&name, Theme::Cyberpunk));
            }
            Msg::SetName(s) => self.name = s,
            Msg::None => {}
        }
        Cmd::none()
    }

    fn view(&self) -> sauron::Node<Msg> {
        sauron::node! {
          <main class="flex justify-center">
          {Theme::dropdown()}
          {
              for p in &self.people {
                  p.card()
              }
          }
          </main>
        }
    }
}

#[wasm_bindgen(start)]
pub fn wasm_main() {
    Program::mount_to_body(App::new());
}
