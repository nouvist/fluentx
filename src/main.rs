use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowControlArea, WindowOptions, div,
    prelude::*, px, size,
};

use crate::design::colors::Colors;

pub mod design {
    pub mod colors;
}

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.global::<Colors>();
        div()
            .flex()
            .flex_col()
            .gap_3()
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .bg(c.background.solid.primary)
            .text_color(c.foreground.primary.primary)
            .window_control_area(WindowControlArea::Drag)
            .child("Halo dunia!")
            .child(div().size_16().bg(c.accent.base))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: None,
                ..Default::default()
            },
            |window, cx| {
                Colors::init(window, cx);
                cx.new(|_| HelloWorld)
            },
        )
        .unwrap();
    });
}
