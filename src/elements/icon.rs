use gpui::{IntoElement, ParentElement, RenderOnce, SharedString, Styled, div, px};

use crate::foundation::icons::{IconNameFilled, IconNameRegular};

pub enum IconName {
    Regular(IconNameRegular),
    Filled(IconNameFilled),
}

impl From<IconNameRegular> for IconName {
    fn from(value: IconNameRegular) -> Self {
        IconName::Regular(value)
    }
}

impl From<IconNameFilled> for IconName {
    fn from(value: IconNameFilled) -> Self {
        IconName::Filled(value)
    }
}

impl From<IconName> for SharedString {
    fn from(value: IconName) -> Self {
        match value {
            IconName::Regular(it) => it.content().into(),
            IconName::Filled(it) => it.content().into(),
        }
    }
}

impl IconName {
    pub fn font(&self) -> &'static str {
        match self {
            IconName::Regular(_) => "FluentSystemIcons-Regular",
            IconName::Filled(_) => "FluentSystemIcons-Filled",
        }
    }

    pub fn content(&self) -> SharedString {
        match self {
            IconName::Regular(it) => it.content().into(),
            IconName::Filled(it) => it.content().into(),
        }
    }

    pub fn size(&self) -> f32 {
        match self {
            IconName::Regular(it) => it.size(),
            IconName::Filled(it) => it.size(),
        }
    }
}

#[derive(IntoElement)]
pub struct Icon {
    name: IconName,
}

pub fn icon(name: impl Into<IconName>) -> Icon {
    Icon { name: name.into() }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        let size = px(self.name.size());
        div()
            .font_family(self.name.font())
            .child(self.name.content())
            .size(size)
            .text_size(size)
            .text_center()
    }
}
