use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowControlArea, WindowOptions, div,
    prelude::*, px, size,
};

use crate::{
    elements::titlebar_control,
    foundation::{
        assets::Assets,
        backdrop::Backdrop,
        colors::{AppColors, Colors},
    },
};

pub mod elements;
pub mod foundation;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(
                div()
                    .flex()
                    .w_full()
                    .h(px(48.))
                    .child(div().flex_1().window_control_area(WindowControlArea::Drag))
                    .child(titlebar_control()),
            )
            .child(div().size_12().bg(cx.colors().accent.base))
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
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
