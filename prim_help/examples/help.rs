use std::borrow::Cow;

use prim_help::{Background, Help, HelpItem, HelpGroup, Options, Theme};
use prim_core::Render;

fn main() {
    let options = Options {
        title: Some(Cow::Borrowed("帮助中心")),
        theme: Theme {
            background: Some(Background::Color(skia_safe::Color::from_argb(255, 245, 245, 250))),
        },
        groups: vec![
            HelpGroup {
                name: Cow::Borrowed("基础功能"),
                list: vec![
                    HelpItem {
                        name: Cow::Borrowed("订阅"),
                        desc: Cow::Borrowed("管理您的订阅和账单"),
                        icon: None,
                    },
                    HelpItem {
                        name: Cow::Borrowed("令牌"),
                        desc: Cow::Borrowed("管理访问令牌"),
                        icon: None,
                    },
                    HelpItem {
                        name: Cow::Borrowed("仓库"),
                        desc: Cow::Borrowed("管理代码仓库"),
                        icon: None,
                    },
                ],
            },
        ],
    };

    let help = Help::new(options);
    let data = help.render().unwrap();

    std::fs::write("help.png", data).unwrap();
    println!("Done: help.png");
}
