use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowControlArea, WindowOptions, div,
    prelude::*, px, size,
};

use crate::foundation::{backdrop::Backdrop, colors::Colors};

pub mod foundation;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.global::<Colors>();
        div()
            .flex()
            .flex_col()
            .size_full()
            .justify_center()
            .items_center()
            .gap_3()
            .child(
                div()
                    .size_16()
                    .bg(c.accent.base)
                    .window_control_area(WindowControlArea::Drag),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        let options = WindowOptions {
            titlebar: None,
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };

        cx.open_window(options, |window, cx| {
            Colors::init(window, cx);
            #[cfg(windows)]
            Backdrop::new(window).map(|it| {
                it.extend();
                it.mica();
            });

            cx.new(|_| HelloWorld)
        })
        .unwrap();
    });
}
