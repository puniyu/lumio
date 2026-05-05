use bytes::Bytes;
use skia_safe::Color;
use std::borrow::Cow;

/// 主题配置
#[derive(Debug, Clone, Default)]
pub struct Theme {
    pub background: Option<Background>
}

/// 背景类型
#[derive(Debug, Clone)]
pub enum Background {
    /// 图片背景
    Image(Bytes),
    /// 纯色背景
    Color(Color),
}

impl Default for Background {
    fn default() -> Self {
        Self::Color(Color::from_argb(255, 245, 245, 250))
    }
}

/// 单个帮助项
#[derive(Debug)]
pub struct HelpItem {
    pub name: Cow<'static, str>,
    pub desc: Cow<'static, str>,
    pub icon: Option<Bytes>,
}

/// 帮助分组
#[derive(Debug)]
pub struct HelpGroup {
    pub name: Cow<'static, str>,
    pub list: Vec<HelpItem>,
}

/// 渲染选项配置
#[derive(Debug)]
pub struct Options {
    pub title: Option<Cow<'static, str>>,
    pub theme: Theme,
    pub groups: Vec<HelpGroup>,
}