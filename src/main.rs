use dioxus::prelude::*;
use dioxus::events::{MouseEvent, FormEvent, PointerData};

use rand::{random, thread_rng, Rng};

fn main() {
    dioxus::web::launch(app);
}


fn app(cx: Scope) -> Element {
    let first_load = use_state(&cx, || false);
    let configured = use_state(&cx, || false);
    let positions: &UseState<Vec<(i32, i32, i32)>> = use_state(&cx, || Vec::new());
    if **first_load && !**configured {
        let max_dims = dims("container");

        for _ in 0..900 {
            let x = thread_rng().gen_range(0..(max_dims.0 as i32));
            let y = thread_rng().gen_range(0..(max_dims.1 as i32));
            let size = thread_rng().gen_range(1..3);
            positions.make_mut().push((x, y, size));
        }

        configured.set(true);
    } else {
        first_load.set(true);
    }

    let mut status = "drifting";

    let score = use_state(&cx, || 0);

    let view_width = 400;
    let view_height = 200;

    let safe_zone_1_min = (800, 600);
    let safe_zone_1_max = (1200, 900);

    let left = use_state(&cx, || 400);
    let top = use_state(&cx, || 400);

    let view_start = (*left.current(), *top.current());
    let view_end = (left + view_width, top + view_height);


    let not_drift = **top > safe_zone_1_min.1 && **top < safe_zone_1_max.1 && **left > safe_zone_1_min.0 && **left < safe_zone_1_max.0;

    if not_drift {
        status = "stable zone";
    }

    let mut stars: Vec<Element> = Vec::new();
    for props in &**positions {
        let size = in_view_scope(*props, view_start, view_end, not_drift);

        if size != props.2 && size != (props.2 + 1) * 2 {
            score.set(**score + 1);
        }

        stars.push(cx.render(rsx!{
            Star {
                left: props.0,
                top: props.1,
                size: size,
            }
        }))
    }

    

    if !not_drift && rand::random() {
        let new_top = thread_rng().gen_range(-1..1);
        let new_left = thread_rng().gen_range(0..2);
        top.set(**top + new_top);
        left.set(**left + new_left);
    }

    let cur_score = **score;

    let adjustment = 40;

    cx.render(rsx!{
        div {
            id: "container",
            Sun {}
            TargetView{}
            ScopeView {
                left: view_start.0,
                top: view_start.1,
                width: view_width,
                height: view_height,
                div {
                    style: "position: relative; top: -4rem",
                    h4 {
                        "{status} / discovered {cur_score} possible exoplanets"
                    }
                }
                div {
                    style: "position: relative; left: -105px; top: -45px; width: 100px;",
                    Button {
                        name: "left",
                        onclick: move |_| left.set(**left - adjustment),
                    }
                    Button {
                        name: "right",
                        onclick: move |_| left.set(**left + adjustment),
                    }
                    Button {
                        name: "up",
                        onclick: move |_| top.set(**top - adjustment),
                    }
                    Button {
                        name: "down",
                        onclick: move |_| top.set(**top + adjustment),
                    }
                }
            }
            stars.iter()
        }
    })
}

fn dims(id: &str) -> (f64, f64) {
    let parent_rect = web_sys::window()
    .unwrap().document()
    .unwrap().get_element_by_id(id)
    .unwrap().get_bounding_client_rect();
    (parent_rect.width(), parent_rect.height())
}

fn in_view_scope(props: (i32, i32, i32), start: (i32, i32), end: (i32, i32), not_drifting: bool) -> i32 {
    if props.0 > start.0 && props.0 < end.0 && props.1 > start.1 && props.1 < end.1 {
        let full_size = (props.2 + 1) * 2;

        let mut odds = 3000;
        if not_drifting {
            odds = 300;
        }
        if thread_rng().gen_range(1..odds) == 1 {
            thread_rng().gen_range(full_size - 3..full_size)
        } else {
            full_size
        }
    } else {
        props.2
    }
}


#[allow(non_snake_case)]
fn Sun(cx: Scope) -> Element {
    cx.render(rsx!{
        div {
            class: "sun"
        }
    })
}

#[derive(PartialEq, Props)]
struct StarProps {
    left: i32,
    top: i32,
    size: i32,
}

#[allow(non_snake_case)]
fn Star(cx: Scope<StarProps>) -> Element {

    let color = use_state(&cx, || "var(--fg)");

    let initial_size = use_state(&cx, || cx.props.size);

    if cx.props.size != **initial_size && cx.props.size != (**initial_size + 1) * 2 {
        color.set("var(--highlight)");
    }
    let size = cx.props.size as f64 / 5.0;
    cx.render(rsx!{
        div {
            class: "star",
            style: "left: {cx.props.left}px; top: {cx.props.top}px; 
                width: {size}rem; height: {size}rem; background-color: {color}"
        }
    })
}

#[derive(Props)]
struct ScopeProps<'a> {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
    children: Element<'a>
}

#[allow(non_snake_case)]
fn ScopeView<'a>(cx: Scope<'a, ScopeProps<'a>>) -> Element {
    cx.render(rsx!{
        div {
            class: "scope-view",
            style: "width: {cx.props.width}px; height: {cx.props.height}px; 
                    left: {cx.props.left}px; top: {cx.props.top}px",
            &cx.props.children
        }
        
    })
}

#[allow(non_snake_case)]
fn TargetView(cx: Scope) -> Element {
    cx.render(rsx!{
        div {
            class: "scope-view target-view",
            style: "border: .1rem solid var(--highlight)"
        }
    })
}

#[derive(Props)] 
pub struct ButtonProps<'a> {
    name: &'a str,
    onclick: EventHandler<'a, MouseEvent>
}

#[allow(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    cx.render(rsx!{
        div {
            button {
                class: "button button-outline",
                style: "margin: 0rem  .2rem .2rem 0;",
                width: "100%",
                onclick: move |evt| cx.props.onclick.call(evt),
                "{cx.props.name}"
            }
        }
    })
}