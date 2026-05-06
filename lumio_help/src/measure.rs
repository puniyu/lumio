use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};

use crate::HelpItem;

/// 文本测量结果
#[derive(Clone, Default)]
pub struct MeasureResult {
    pub width: f32,
    pub height: f32,
    pub line_count: usize,
}

/// 测量上下文
pub struct Measure {
    font_collection: FontCollection,
    font_family: String,
}

impl Measure {
    pub fn new(font_collection: FontCollection, font_family: impl Into<String>) -> Self {
        Self {
            font_collection,
            font_family: font_family.into(),
        }
    }

    /// 测量文本
    pub fn text(&self, text: &str, font_size: f32, max_width: f32) -> MeasureResult {
        let mut text_style = TextStyle::new();
        text_style.set_font_size(font_size);
        text_style.set_font_families(std::slice::from_ref(&self.font_family));

        let mut paragraph_style = ParagraphStyle::new();
        paragraph_style.set_text_style(&text_style);

        let mut builder = ParagraphBuilder::new(&paragraph_style, &self.font_collection);
        builder.add_text(text);

        let mut paragraph = builder.build();
        paragraph.layout(max_width);

        MeasureResult {
            width: paragraph.max_intrinsic_width().min(max_width),
            height: paragraph.height(),
            line_count: paragraph.line_number(),
        }
    }

    /// 测量卡片尺寸
    #[allow(clippy::too_many_arguments)]
    pub fn card(
        &self,
        item: &HelpItem,
        card_width: f32,
        card_padding: f32,
        icon_size: f32,
        icon_text_gap: f32,
        name_font_size: f32,
        desc_font_size: f32,
        card_min_height: f32,
    ) -> MeasureResult {
        let has_icon = item.icon.as_ref().is_some_and(|v| !v.is_empty());
        let icon_offset = if has_icon { icon_size + icon_text_gap } else { 0.0 };

        let name_w = card_width - card_padding * 2.0 - icon_offset;
        let name = self.text(&item.name, name_font_size, name_w);
        let desc = self.text(&item.desc, desc_font_size, card_width - card_padding * 2.0);

        let height = (card_padding * 2.0 + name.height + 8.0 + desc.height).max(card_min_height);

        MeasureResult {
            width: card_width,
            height,
            line_count: name.line_count + desc.line_count,
        }
    }
}