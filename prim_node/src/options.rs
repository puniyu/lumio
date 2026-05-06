use napi::bindgen_prelude::Buffer;
use napi_derive::napi;


/// 主题配置
#[napi(object)]
pub struct Theme {
    pub background: Option<Background>
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[napi(object)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// 背景类型
#[napi]
pub enum Background {
    /// 图片背景
    Image(Buffer),
    /// 纯色背景
    Color(Rgb),
}

impl From<prim::help::Background> for Background  {
    fn from(value: prim::help::Background) -> Self {
        match value {
            prim::help::Background::Image(data) => Self::Image(data.to_vec().into()),
            prim::help::Background::Color(color) => Self::Color(Rgb {
                r: color.r(),
                g: color.g(),
                b: color.b(),
            }),
        }
    }
}

impl From<Background> for prim::help::Background  {
    fn from(value: Background) -> Self {
        match value {
            Background::Image(data) => Self::Image(data.to_vec().into()),
            Background::Color(color) => Self::Color(skia_safe::Color::from_rgb( color.r, color.g, color.b)),
        }
    }
}

impl From<prim::help::Theme> for Theme {
    fn from(value: prim::help::Theme) -> Self {
        Self {
            background: value.background.map(|b| b.into()),
        }
    }
}

impl From<Theme> for prim::help::Theme  {
    fn from(value: Theme) -> Self {
        prim::help::Theme {
            background: value.background.map(|b| b.into()),
        }
    }
}

/// 单个帮助项
#[napi(object)]
pub struct HelpItem {
    pub name: String,
    pub desc: String,
    pub icon: Option<Buffer>,
}

impl From<prim::help::HelpItem> for HelpItem {
    fn from(value: prim::help::HelpItem) -> Self {
        Self {
            name: value.name.into_owned(),
            desc: value.desc.into_owned(),
            icon: value.icon.map(|b| b.to_vec().into()),
        }
    }
}

impl From<HelpItem> for prim::help::HelpItem {
    fn from(value: HelpItem) -> Self {
        Self {
            name: value.name.into(),
            desc: value.desc.into(),
            icon: value.icon.map(|b| b.to_vec().into()),
        }
    }
}

/// 帮助分组
#[napi(object)]
pub struct HelpGroup {
    pub name: String,
    pub list: Vec<HelpItem>,
}

impl From<prim::help::HelpGroup> for HelpGroup {
    fn from(value: prim::help::HelpGroup) -> Self {
        Self {
            name: value.name.into_owned(),
            list: value.list.into_iter().map(|i| i.into()).collect(),
        }
    }
}

impl From<HelpGroup> for prim::help::HelpGroup {
    fn from(value: HelpGroup) -> Self {
        Self {
            name: value.name.into(),
            list: value.list.into_iter().map(|i| i.into()).collect(),
        }
    }
}

/// 渲染选项配置
#[napi(object)]
pub struct Options {
    pub title: Option<String>,
    pub theme: Theme,
    pub groups: Vec<HelpGroup>,
}

impl From<prim::help::Options> for Options {
    fn from(value: prim::help::Options) -> Self {
        Self {
            title: value.title.map(|t| t.into_owned()),
            theme: value.theme.into(),
            groups: value.groups.into_iter().map(|g| g.into()).collect(),
        }
    }
}

impl From<Options> for prim::help::Options {
    fn from(value: Options) -> Self {
        Self {
            title: value.title.map(|t| t.into()),
            theme: value.theme.into(),
            groups: value.groups.into_iter().map(|g| g.into()).collect(),
        }
    }
}