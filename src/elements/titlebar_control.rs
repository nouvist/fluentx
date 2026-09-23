use gpui::{
    App, Div, ElementId, IntoElement, RenderOnce, Stateful, Window, WindowControlArea, div,
    prelude::*, px, rgba, svg,
};

use crate::foundation::colors::AppColors;

#[derive(IntoElement)]
pub struct TitlebarControl;

pub fn titlebar_control() -> impl IntoElement {
    TitlebarControl
}

impl TitlebarControl {
    fn render_container(id: impl Into<ElementId>) -> Stateful<Div> {
        div().id(id).flex_1().flex().justify_center().items_center()
    }
}

impl RenderOnce for TitlebarControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.colors();
        div()
            .flex()
            .size_full()
            .max_w(px(144.))
            .child(
                Self::render_container("min")
                    .bg(c.control.subtle.primary)
                    .hover(|style| style.bg(c.control.subtle.secondary))
                    .active(|style| style.bg(c.control.secondary.tertiary))
                    .window_control_area(WindowControlArea::Min)
                    .child(
                        svg()
                            .path("fluentx/titlebar/minimize.svg")
                            .text_color(c.foreground.primary.primary)
                            .size(px(16.)),
                    ),
            )
            .child(
                Self::render_container("max")
                    .bg(c.control.subtle.primary)
                    .hover(|style| style.bg(c.control.subtle.secondary))
                    .active(|style| style.bg(c.control.secondary.tertiary))
                    .window_control_area(WindowControlArea::Max)
                    .child(
                        svg()
                            .path(match window.is_maximized() {
                                true => "fluentx/titlebar/restore.svg",
                                false => "fluentx/titlebar/maximize.svg",
                            })
                            .text_color(c.foreground.primary.primary)
                            .size(px(16.)),
                    ),
            )
            .child(
                Self::render_container("close")
                    .group("close")
                    .bg(c.control.subtle.primary)
                    .hover(|style| style.bg(rgba(0xc42b1cff)))
                    .active(|style| style.bg(rgba(0xc42b1ce6)))
                    .window_control_area(WindowControlArea::Close)
                    .child(
                        svg()
                            .path("fluentx/titlebar/close.svg")
                            .text_color(c.foreground.primary.primary)
                            .id("svg")
                            .when(c.brightness.is_light(), |it| {
                                it.group_hover("close", |style| {
                                    style.text_color(c.foreground.on_accent.primary)
                                })
                                .group_active("close", |style| {
                                    style.text_color(c.foreground.on_accent.primary)
                                })
                            })
                            .size(px(16.)),
                    ),
            )
    }
}
