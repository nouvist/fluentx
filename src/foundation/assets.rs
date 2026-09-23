use gpui::{AssetSource, SharedString};

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        Ok(match path {
            "fluentx/titlebar/close.svg" => {
                Some(include_bytes!("./assets/titlebar/close.svg").into())
            }
            "fluentx/titlebar/maximize.svg" => {
                Some(include_bytes!("./assets/titlebar/maximize.svg").into())
            }
            "fluentx/titlebar/restore.svg" => {
                Some(include_bytes!("./assets/titlebar/restore.svg").into())
            }
            "fluentx/titlebar/minimize.svg" => {
                Some(include_bytes!("./assets/titlebar/minimize.svg").into())
            }
            _ => None,
        })
    }

    fn list(&self, _path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(vec![])
    }
}
