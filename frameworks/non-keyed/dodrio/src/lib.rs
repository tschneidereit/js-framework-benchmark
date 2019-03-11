// Note: this code is based in part on the wasm-bindgen version of the benchmark:
// https://github.com/krausest/js-framework-benchmark/tree/bb9e5d5ace205f695cf30062b9362cba13db88e0/frameworks/keyed/wasm-bindgen
use dodrio::bumpalo::{self, Bump};
use dodrio::{Cached, Render, RootRender, VdomWeak};
use js_sys::Math;
use log::*;
use wasm_bindgen::prelude::*;

const ADJECTIVES_LEN: usize = 25;
const ADJECTIVES_LEN_F64: f64 = ADJECTIVES_LEN as f64;
const ADJECTIVES: [&str; ADJECTIVES_LEN] = [
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
];

const COLOURS_LEN: usize = 11;
const COLOURS_LEN_F64: f64 = COLOURS_LEN as f64;
const COLOURS: [&str; COLOURS_LEN] = [
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

const NOUNS_LEN: usize = 13;
const NOUNS_LEN_F64: f64 = NOUNS_LEN as f64;
const NOUNS: [&str; NOUNS_LEN] = [
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

fn random(max: f64) -> usize {
    ((Math::random() * 1000.0) % max) as usize
}

fn jumbotron_button<'a, 'bump, F>(
    bump: &'bump Bump,
    id: &'a str,
    title: &'a str,
    callback: F,
) -> dodrio::Node<'bump>
where
    'a: 'bump,
    F: 'static + Fn(&mut dyn RootRender, VdomWeak, web_sys::Event),
{
    use dodrio::builder::{button, div, text};
    div(bump)
        .attr("class", "col-sm-6 smallpad")
        .children([button(bump)
            .attr("type", "button")
            .attr("class", "btn btn-primary btn-block")
            .attr("id", id)
            .on("click", callback)
            .children([text(title)])
            .finish()])
        .finish()
}

struct Jumbotron {
}

impl Render for Jumbotron {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> dodrio::Node<'bump>
    where
        'a: 'bump,
    {
        use dodrio::builder::*;

        div(bump)
            .attr("class", "jumbotron")
            .children([div(bump)
                .attr("class", "row")
                .children([
                    div(bump)
                        .attr("class", "col-md-6")
                        .children([h1(bump).children([text("Dodrio")]).finish()])
                        .finish(),
                    div(bump)
                        .attr("class", "col-md-6")
                        .children([div(bump)
                            .attr("class", "row")
                            .children([
                                jumbotron_button(
                                    bump,
                                    "run",
                                    "Create 1,000 rows",
                                    |root, vdom, _event| {
                                        root.unwrap_mut::<Main>().run(vdom);
                                    },
                                ),
                                jumbotron_button(
                                    bump,
                                    "runlots",
                                    "Create 10,000 rows",
                                    |root, vdom, _event| {
                                        root.unwrap_mut::<Main>().run_lots(vdom);
                                    },
                                ),
                                jumbotron_button(
                                    bump,
                                    "add",
                                    "Append 1,000 rows",
                                    |root, vdom, _event| {
                                        root.unwrap_mut::<Main>().add(vdom);
                                    },
                                ),
                                jumbotron_button(
                                    bump,
                                    "update",
                                    "Update every 10th row",
                                    |root, vdom, _event| {
                                        root.unwrap_mut::<Main>().update(vdom);
                                    },
                                ),
                                jumbotron_button(bump, "clear", "Clear", |root, vdom, _event| {
                                    root.unwrap_mut::<Main>().clear(vdom);
                                }),
                                jumbotron_button(
                                    bump,
                                    "swaprows",
                                    "Swap Rows",
                                    |root, vdom, _event| {
                                        root.unwrap_mut::<Main>().swap_rows(vdom);
                                    },
                                ),
                            ])
                            .finish()])
                        .finish(),
                ])
                .finish()])
            .finish()
    }
}

struct Row {
    id: usize,
    label: String,
    selected: bool,
}

impl Row {
    fn new(id: usize, label: String) -> Self {
        Row { id, label, selected: false }
    }

    fn update(&mut self) {
        self.label.push_str(" !!!");
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

impl Render for Row {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> dodrio::Node<'bump>
    where
        'a: 'bump,
    {
        use dodrio::builder::*;

        let id = self.id;
        let id_str = bumpalo::format!(in bump, "{}", self.id);

        tr(bump)
            .attr("class", if self.selected { "danger" } else { "" })
            .children([
                td(bump)
                    .attr("class", "col-md-1")
                    .children([text(id_str.into_bump_str())])
                    .finish(),
                td(bump)
                    .attr("class", "col-md-4")
                    .children([a(bump)
                        .on("click", move |root, vdom, _event| {
                            root.unwrap_mut::<Main>().select(id, vdom);
                        })
                        .children([text(&self.label)])
                        .finish()])
                    .finish(),
                td(bump)
                    .attr("class", "col-md-1")
                    .children([a(bump)
                        .on("click", move |root, vdom, _event| {
                            root.unwrap_mut::<Main>().remove(id, vdom);
                        })
                        .children([glyphicon(bump)])
                        .finish()])
                    .finish(),
                td(bump).attr("class", "col-md-6").finish(),
            ])
            .finish()
    }
}

fn glyphicon<'bump>(bump: &'bump Bump) -> dodrio::Node<'bump> {
    use dodrio::builder::span;
    span(bump)
        .attr("class", "glyphicon glyphicon-remove")
        .attr("aria-hidden", "true")
        .finish()
}

struct Main {
    jumbotron: Cached<Jumbotron>,
    // data: Vec<Cached<Row>>,
    data: Vec<Row>,
    next_id: usize,
}

impl Main {
    fn new() -> Main {
        Main {
            jumbotron: Cached::new(Jumbotron {}),
            data: vec![],
            next_id: 1,
        }
    }

    fn run(&mut self, vdom: VdomWeak) {
        self.data.truncate(0);
        for _ in 0..1000 {
            self.add_row();
        }
        vdom.schedule_render();
    }

    fn run_lots(&mut self, vdom: VdomWeak) {
        self.data.truncate(0);
        for _ in 0..10000 {
            self.add_row();
        }
        vdom.schedule_render();
    }

    fn add(&mut self, vdom: VdomWeak) {
        for _ in 0..1000 {
            self.add_row();
        }
        vdom.schedule_render();
    }

    fn update(&mut self, vdom: VdomWeak) {
        for row in self.data.iter_mut().step_by(10) {
            row.update();
            // Cached::invalidate(row);
        }
        vdom.schedule_render();
    }

    fn clear(&mut self, vdom: VdomWeak) {
        self.data.truncate(0);
        vdom.schedule_render();
    }

    fn swap_rows(&mut self, vdom: VdomWeak) {
        if self.data.len() > 998 {
            self.data.swap(1, 998);
        }
        vdom.schedule_render();
    }

    fn add_row(&mut self) {
        let adjective = ADJECTIVES[random(ADJECTIVES_LEN_F64)];
        let colour = COLOURS[random(COLOURS_LEN_F64)];
        let noun = NOUNS[random(NOUNS_LEN_F64)];
        let capacity = adjective.len() + colour.len() + noun.len() + 2;
        let mut label = String::with_capacity(capacity);
        label.push_str(adjective);
        label.push(' ');
        label.push_str(colour);
        label.push(' ');
        label.push_str(noun);

        // self.data.push(Cached::new(Row::new(self.next_id, label)));
        self.data.push(Row::new(self.next_id, label));
        self.next_id += 1;
    }

    fn select(&mut self, id: usize, vdom: VdomWeak) {
        self.data.iter_mut().find(|r| r.selected).map(|r| {
            r.set_selected(false);
            // Cached::invalidate(r);
        });
        self.data.iter_mut().find(|r| r.id == id).map(|r| {
            r.set_selected(true);
            // Cached::invalidate(r);
        });
        vdom.schedule_render();
    }

    fn remove(&mut self, id: usize, vdom: VdomWeak) {
        let position = self.data.iter().position(|r| r.id == id).unwrap();
        self.data.remove(position);
        vdom.schedule_render();
    }
}

impl Render for Main {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> dodrio::Node<'bump>
    where
        'a: 'bump,
    {
        use dodrio::{builder::*, bumpalo::collections::Vec};

        let mut rows = Vec::with_capacity_in(self.data.len(), bump);
        rows.extend(self.data.iter().map(|r| r.render(bump)));
        div(bump)
            .attr("id", "main")
            .children([div(bump)
                .attr("class", "container")
                .children([
                    self.jumbotron.render(bump),
                    table(bump)
                        .attr("class", "table table-hover table-striped test-data")
                        .children([tbody(bump).children(rows).finish()])
                        .finish(),
                    span(bump)
                        .attr("class", "preloadicon glyphicon glyphicon-remove")
                        .attr("aria-hidden", "true")
                        .finish(),
                ])
                .finish()])
            .finish()
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    // Initialize debug logging for if/when things go wrong.
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).expect("should initialize logging OK");

    // Get the document's `<body>`.
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Construct a new Jumbotron component.
    let main = Main::new();

    // Mount our Main component to the `<body>`.
    let vdom = dodrio::Vdom::new(&body, main);

    // Run the virtual DOM and its listeners forever.
    vdom.forget();
}
