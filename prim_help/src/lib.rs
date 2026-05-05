mod layout;
mod measure;
mod options;

use layout::{Flex, FlexDirection, FlexItem, FlexWrap};
use measure::Measure;

use bytes::Bytes;
use prim_core::{Error, FontManger};
use skia_safe::{
	BlurStyle, Canvas, ClipOp, Color, Data, EncodedImageFormat, Image, MaskFilter, Paint,
	PaintStyle, RRect, Rect,
	canvas::SrcRectConstraint,
	surfaces,
	textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle},
};

pub use options::{Background, HelpGroup, HelpItem, Options, Theme};

pub struct Help {
	options: Options,
}

impl Help {
	const WIDTH: i32 = 600;
	const PADDING: f32 = 24.0;
	const CARD_GAP: f32 = 12.0;
	const CARD_PADDING: f32 = 12.0;
	const CARD_MIN_HEIGHT: f32 = 72.0;
	const ICON_SIZE: f32 = 20.0;
	const ICON_TEXT_GAP: f32 = 8.0;
	const COLS: usize = 3;

	const MAIN_TITLE_FONT_SIZE: f32 = 32.0;
	const TITLE_FONT_SIZE: f32 = 26.0;
	const NAME_FONT_SIZE: f32 = 14.0;
	const DESC_FONT_SIZE: f32 = 12.0;

	const FONT_FAMILY: &str = "DouyinSansBold";
	const SCALE_FACTOR: f32 = 2.0;
	const CARD_RADIUS: f32 = 12.0;
	const BLUR_SIGMA: f32 = 20.0;

	const DEFAULT_CARD_COLOR: Color = Color::from_argb(180, 255, 255, 255);
	const DEFAULT_TEXT_COLOR: Color = Color::from_argb(255, 50, 50, 60);
	const DEFAULT_DESC_COLOR: Color = Color::from_argb(200, 80, 80, 90);

	pub fn new(options: Options) -> Self {
		Self { options }
	}

	fn draw_background(&self, canvas: &Canvas, height: i32) -> Result<Option<Image>, Error> {
		let background = self.options.theme.background.clone().unwrap_or_default();
		match background {
			Background::Color(color) => {
				canvas.clear(color);
				Ok(None)
			}
			Background::Image(data) => {
				let data = Data::new_copy(&data);
				let image = Image::from_encoded(data).ok_or(Error::Decode)?;

				let img_w = image.width() as f32;
				let img_h = image.height() as f32;
				let dst_w = Self::WIDTH as f32;
				let dst_h = height as f32;

				let scale = (dst_w / img_w).max(dst_h / img_h);
				let src_w = dst_w / scale;
				let src_h = dst_h / scale;
				let src_x = (img_w - src_w) / 2.0;
				let src_y = (img_h - src_h) / 2.0;

				let src_rect = Rect::from_xywh(src_x, src_y, src_w, src_h);
				let dst_rect = Rect::from_wh(dst_w, dst_h);

				canvas.draw_image_rect(
					&image,
					Some((&src_rect, SrcRectConstraint::Fast)),
					dst_rect,
					&Paint::default(),
				);

				Ok(Some(image))
			}
		}
	}

	fn draw_card(&self, canvas: &Canvas, rect: Rect, bg_image: Option<&Image>, canvas_height: i32) {
		let rrect = RRect::new_rect_xy(rect, Self::CARD_RADIUS, Self::CARD_RADIUS);

		if let Some(image) = bg_image {
			canvas.save();
			canvas.clip_rrect(rrect, ClipOp::Intersect, true);

			let img_w = image.width() as f32;
			let img_h = image.height() as f32;
			let dst_w = Self::WIDTH as f32;
			let dst_h = canvas_height as f32;

			let scale = (dst_w / img_w).max(dst_h / img_h);
			let src_w = dst_w / scale;
			let src_h = dst_h / scale;
			let src_x = (img_w - src_w) / 2.0;
			let src_y = (img_h - src_h) / 2.0;

			let src_rect = Rect::from_xywh(src_x, src_y, src_w, src_h);
			let dst_rect = Rect::from_wh(dst_w, dst_h);

			let blur_filter = skia_safe::image_filters::blur(
				(Self::BLUR_SIGMA, Self::BLUR_SIGMA),
				None,
				None,
				None,
			);
			let mut blur_paint = Paint::default();
			blur_paint.set_image_filter(blur_filter);

			canvas.draw_image_rect(
				image,
				Some((&src_rect, SrcRectConstraint::Fast)),
				dst_rect,
				&blur_paint,
			);

			canvas.restore();
		}

		let mut paint = Paint::default();
		paint.set_color(Self::DEFAULT_CARD_COLOR);
		paint.set_style(PaintStyle::Fill);
		paint.set_anti_alias(true);

		let shadow_paint = {
			let mut p = Paint::default();
			p.set_color(Color::from_argb(30, 0, 0, 0));
			p.set_mask_filter(MaskFilter::blur(BlurStyle::Normal, 8.0, false));
			p
		};
		canvas.draw_rrect(rrect, &shadow_paint);
		canvas.draw_rrect(rrect, &paint);
	}

	fn draw_icon(
		&self,
		canvas: &Canvas,
		icon_data: &[u8],
		rect: Rect,
	) -> Result<(), Error> {
		let mut dom = skia_safe::svg::Dom::from_bytes(icon_data, skia_safe::FontMgr::default())
			.map_err(|_| Error::Decode)?;

		let root = dom.root();
		let svg_size = root.intrinsic_size();
		let scale_x = rect.width() / svg_size.width;
		let scale_y = rect.height() / svg_size.height;
		let scale = scale_x.min(scale_y);

		canvas.save();
		canvas.scale((scale, scale));
		let offset_x = rect.x() / scale;
		let offset_y = rect.y() / scale;
		canvas.translate((offset_x, offset_y));
		dom.set_container_size(svg_size);
		dom.render(canvas);
		canvas.restore();
		Ok(())
	}

	fn draw_text(
		&self,
		canvas: &Canvas,
		text: &str,
		params: &TextParams,
		font_collection: &FontCollection,
	) -> (usize, f32) {
		let mut text_style = TextStyle::new();
		text_style.set_font_size(params.font_size);
		text_style.set_color(params.color);
		text_style.set_font_families(&[params.font_family]);

		let mut paragraph_style = ParagraphStyle::new();
		paragraph_style.set_text_style(&text_style);
		paragraph_style.set_text_align(params.align);

		let mut builder = ParagraphBuilder::new(&paragraph_style, font_collection);
		builder.add_text(text);

		let mut paragraph = builder.build();
		paragraph.layout(params.rect.width());
		paragraph.paint(canvas, (params.rect.x(), params.rect.y()));
		(paragraph.line_number(), paragraph.height())
	}

	fn render(&self) -> Result<Vec<u8>, Error> {
		let font_collection = FontManger::new().font_collection().clone();
		let total_gap = Self::CARD_GAP * (Self::COLS as f32 - 1.0);
		let card_width = (Self::WIDTH as f32 - Self::PADDING * 2.0 - total_gap) / Self::COLS as f32;

		let measure = Measure::new(font_collection.clone(), Self::FONT_FAMILY);
		let main_title_h = self
			.options
			.title
			.as_ref()
			.map(|t| measure.text(t, Self::MAIN_TITLE_FONT_SIZE, Self::WIDTH as f32).height + Self::PADDING)
			.unwrap_or(0.0);

		let mut root = Flex::new()
			.direction(FlexDirection::Column)
			.gap(Self::PADDING)
			.size(Self::WIDTH as f32, f32::MAX);

		// 顶部边距
		root = root.push(FlexItem::fixed(Self::WIDTH as f32, Self::PADDING));

		if self.options.title.is_some() {
			root = root.push(FlexItem::fixed(Self::WIDTH as f32, main_title_h));
		}

		for group in &self.options.groups {
			let group_container_width = (card_width + Self::CARD_GAP) * Self::COLS as f32 - Self::CARD_GAP;

			let mut group_flex = Flex::new()
				.direction(FlexDirection::Row)
				.wrap(FlexWrap::Wrap)
				.gap(Self::CARD_GAP)
				.size(group_container_width, f32::MAX);

			for item in &group.list {
				let card_size = measure.card(
					item,
					card_width,
					Self::CARD_PADDING,
					Self::ICON_SIZE,
					Self::ICON_TEXT_GAP,
					Self::NAME_FONT_SIZE,
					Self::DESC_FONT_SIZE,
					Self::CARD_MIN_HEIGHT,
				);
				group_flex = group_flex.push(FlexItem::fixed(card_size.width, card_size.height));
			}

			let group_height = group_flex.total_height();
			root = root.push(FlexItem::fixed(
				Self::WIDTH as f32,
				Self::TITLE_FONT_SIZE + Self::PADDING + group_height,
			));
		}

		// 底部边距
		root = root.push(FlexItem::fixed(Self::WIDTH as f32, Self::PADDING));

		let rects = root.compute();
		let height = rects.iter().map(|r| r.y() + r.height()).fold(0.0f32, |a, b| a.max(b)) as i32;

		let scaled_width = (Self::WIDTH as f32 * Self::SCALE_FACTOR) as i32;
		let scaled_height = (height as f32 * Self::SCALE_FACTOR) as i32;
		let mut surface =
			surfaces::raster_n32_premul((scaled_width, scaled_height)).ok_or(Error::Encode)?;

		let canvas = surface.canvas();
		canvas.scale((Self::SCALE_FACTOR, Self::SCALE_FACTOR));

		let bg_image = self.draw_background(canvas, height)?;

		if let Some(title) = &self.options.title {
			self.draw_text(
				canvas,
				title,
				&TextParams {
					rect: Rect::from_xywh(0.0, Self::PADDING, Self::WIDTH as f32, main_title_h),
					font_size: Self::MAIN_TITLE_FONT_SIZE,
					color: Self::DEFAULT_TEXT_COLOR,
					font_family: Self::FONT_FAMILY,
					align: TextAlign::Center,
				},
				&font_collection,
			);
		}

		let mut item_idx = if self.options.title.is_some() { 2 } else { 1 };

		for help_group in &self.options.groups {
			let group_rect = rects[item_idx];
			let group_top = group_rect.y();

			self.draw_text(
				canvas,
				&help_group.name,
				&TextParams {
					rect: Rect::from_xywh(
						Self::PADDING,
						group_top,
						Self::WIDTH as f32 - Self::PADDING * 2.0,
						Self::TITLE_FONT_SIZE,
					),
					font_size: Self::TITLE_FONT_SIZE,
					color: Self::DEFAULT_TEXT_COLOR,
					font_family: Self::FONT_FAMILY,
					align: TextAlign::Left,
				},
				&font_collection,
			);

			let group_container_width = (card_width + Self::CARD_GAP) * Self::COLS as f32 - Self::CARD_GAP;
			let mut group_flex = Flex::new()
				.direction(FlexDirection::Row)
				.wrap(FlexWrap::Wrap)
				.gap(Self::CARD_GAP)
				.size(group_container_width, f32::MAX);

			for item in &help_group.list {
				let card_size = measure.card(
					item,
					card_width,
					Self::CARD_PADDING,
					Self::ICON_SIZE,
					Self::ICON_TEXT_GAP,
					Self::NAME_FONT_SIZE,
					Self::DESC_FONT_SIZE,
					Self::CARD_MIN_HEIGHT,
				);
				group_flex = group_flex.push(FlexItem::fixed(card_size.width, card_size.height));
			}

			let card_rects = group_flex.compute();
			let cards_top = group_top + Self::TITLE_FONT_SIZE + Self::PADDING;

			for (i, item) in help_group.list.iter().enumerate() {
				let card_base = card_rects[i];
				let card_x = Self::PADDING + card_base.x();
				let card_y = cards_top + card_base.y();
				let card_rect = Rect::from_xywh(card_x, card_y, card_base.width(), card_base.height());

				self.draw_card(canvas, card_rect, bg_image.as_ref(), height);

				let has_icon = item.icon.as_ref().is_some_and(|v| !v.is_empty());
				let content_y = card_y + Self::CARD_PADDING;

				let name_x = if has_icon {
					let icon_rect = Rect::from_xywh(
						card_x + Self::CARD_PADDING,
						content_y,
						Self::ICON_SIZE,
						Self::ICON_SIZE,
					);
					self.draw_icon(canvas, item.icon.as_deref().unwrap(), icon_rect)?;
					card_x + Self::CARD_PADDING + Self::ICON_SIZE + Self::ICON_TEXT_GAP
				} else {
					card_x + Self::CARD_PADDING
				};

				let name_w = card_width - Self::CARD_PADDING * 2.0;
				let name_result = measure.text(&item.name, Self::NAME_FONT_SIZE, name_w);
				let name_rect_h = name_result.line_count as f32 * Self::NAME_FONT_SIZE;

				let (_, name_h) = self.draw_text(
					canvas,
					&item.name,
					&TextParams {
						rect: Rect::from_xywh(name_x, content_y, name_w, name_rect_h),
						font_size: Self::NAME_FONT_SIZE,
						color: Self::DEFAULT_TEXT_COLOR,
						font_family: Self::FONT_FAMILY,
						align: TextAlign::Left,
					},
					&font_collection,
				);

				let desc_max_h = card_base.height() - Self::CARD_PADDING - name_h - 8.0;
				self.draw_text(
					canvas,
					&item.desc,
					&TextParams {
						rect: Rect::from_xywh(
							card_x + Self::CARD_PADDING,
							content_y + name_h + 8.0,
							card_width - Self::CARD_PADDING * 2.0,
							desc_max_h,
						),
						font_size: Self::DESC_FONT_SIZE,
						color: Self::DEFAULT_DESC_COLOR,
						font_family: Self::FONT_FAMILY,
						align: TextAlign::Left,
					},
					&font_collection,
				);
			}

			item_idx += 1;
		}

		let image = surface.image_snapshot();
		let data = image.encode(None, EncodedImageFormat::PNG, None).ok_or(Error::Encode)?;

		Ok(data.as_bytes().to_vec())
	}
}

impl prim_core::Render for Help {
	#[inline]
	fn render(&self) -> Result<Bytes, Error> {
		let data = self.render()?;
		Ok(Bytes::from(data))
	}
}

pub(crate) struct TextParams<'a> {
	pub rect: Rect,
	pub font_size: f32,
	pub color: Color,
	pub font_family: &'a str,
	pub align: TextAlign,
}