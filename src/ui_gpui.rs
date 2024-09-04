use gpui::*;
use std::rc::Rc;
use crate::config::*;

#[derive(Clone)]
pub struct AppView {
    pub repos: Rc<Vec<Rc<Repo>>>,
    pub current: Option<String>,
}

impl Render for AppView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0x666666))
            .w_full()
            .h_full()
            // title
            .child(
                div()
                    .w_full()
                    .h(px(30.0))
                    .flex()
                    .child(format!(" "))
                    .child(format!("GBKSync"))
                    .child(format!(" "))
                    .text_color(rgb(0xefefef))
                    .items_center()
                    .justify_center(),
            )
            // body
            .child(
                div()
                    .flex()
                    .bg(rgb(0x2e7d32))
                    .w_full()
                    .h_full()
                    // .size(Length::Definite(Pixels(300.0).into()))
                    .justify_center()
                    .items_center()
                    .text_xl()
                    .text_color(rgb(0xffffff))
                    .child(
                        // Left menu part
                        div()
                            .child(
                                div()
                                    .on_mouse_down(MouseButton::Left, |_evt, _ctx| {
                                        println!("click");
                                    })
                                    .child(format!("GBKSync")),
                            )
                            .child(list(ListState::new(
                                5,
                                ListAlignment::Top,
                                px(10.0),
                                |index, ctx| {
                                    div()
                                        .bg(rgb(0x00ff00))
                                        .p(px(10.0))
                                        .child(format!("item"))
                                        .into_any_element()
                                },
                            )))
                            .w(px(300.0))
                            .h_full()
                            .bg(rgb(0xff0000))
                            .p(px(10.0)),
                    )
                    .child(
                        // Right part
                        div()
                            .child(format!("button"))
                            .w_full()
                            .h_full()
                            .bg(rgb(0x00ff00))
                            .p(px(10.0)),
                    ),
            )
    }
}

// fn main() {
//     App::new().run(|cx: &mut AppContext| {
//         let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
//         cx.open_window(
//             WindowOptions {
//                 window_bounds: Some(WindowBounds::Windowed(bounds)),
//                 ..Default::default()
//             },
//             |cx| {
//                 cx.new_view(|_cx| HelloWorld {
//                     text: "World".into(),
//                 })
//             },
//         )
//         .unwrap();
//     });
// }
