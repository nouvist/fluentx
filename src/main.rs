use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowControlArea, WindowOptions, div,
    prelude::*, px, size,
};

use crate::{
    elements::{icon, titlebar_control},
    foundation::{
        assets::Assets,
        backdrop::Backdrop,
        colors::{AppColors, Colors},
        icons::{IconNameRegular, init_icon_filled, init_icon_regular},
    },
};

pub mod elements;
pub mod foundation;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.colors();
        div()
            .flex()
            .flex_col()
            .size_full()
            .text_color(c.foreground.primary.primary)
            .child(
                div()
                    .flex()
                    .w_full()
                    .h(px(48.))
                    .child(div().flex_1().window_control_area(WindowControlArea::Drag))
                    .child(titlebar_control()),
            )
            .child(div().size_12().bg(c.accent.base))
            .child(icon(IconNameRegular::PERSON_12))
            .child(icon(IconNameRegular::PERSON_16))
            .child(icon(IconNameRegular::PERSON_20))
            .child(icon(IconNameRegular::PERSON_24))
            .child(icon(IconNameRegular::PERSON_28))
            .child(icon(IconNameRegular::PERSON_32))
            .child(icon(IconNameRegular::PERSON_48))
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

        init_icon_regular(cx).unwrap();
        init_icon_filled(cx).unwrap();

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
