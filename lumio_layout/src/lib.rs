use skia_safe::Rect;

/// Flex 布局方向
#[derive(Clone, Copy, Default)]
pub enum FlexDirection {
    #[default]
    Row,
    Column,
}

/// Flex 换行模式
#[derive(Clone, Copy, Default, PartialEq)]
pub enum FlexWrap {
    #[default]
    Wrap,
    #[allow(dead_code)]
    NoWrap,
}

/// Flex 布局
#[derive(Clone, Default)]
pub struct Flex {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
    pub gap: f32,
    pub width: f32,
    pub height: f32,
    pub items: Vec<FlexItem>,
}

/// Flex 布局项
#[derive(Clone)]
pub enum FlexItem {
    /// 固定尺寸
    Fixed { width: f32, height: f32 },
    /// 可伸缩

    Flexed { flex: f32, min_height: f32 },
    /// 嵌套 Flex 容器
    Container(Flex),
}

impl FlexItem {
    pub fn fixed(width: f32, height: f32) -> Self {
        Self::Fixed { width, height }
    }

    pub fn flex(flex: f32, min_height: f32) -> Self {
        Self::Flexed { flex, min_height }
    }

    pub fn container(flex: Flex) -> Self {
        Self::Container(flex)
    }
}

impl Flex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn wrap(mut self, wrap: FlexWrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn push(mut self, item: FlexItem) -> Self {
        self.items.push(item);
        self
    }

    /// 计算布局
    pub fn compute(&self) -> Vec<Rect> {
        match self.direction {
            FlexDirection::Row => self.compute_row(),
            FlexDirection::Column => self.compute_column(),
        }
    }

    fn compute_row(&self) -> Vec<Rect> {
        let mut rects = Vec::new();
        let mut current_x = 0.0f32;
        let mut current_y = 0.0f32;
        let mut row_max_h = 0.0f32;

        for item in &self.items {
            let (w, h) = match item {
                FlexItem::Fixed { width, height } => (*width, *height),
                FlexItem::Flexed { flex, min_height } => (self.width / flex, *min_height),
                FlexItem::Container(flex) => {
                    let child_rects = flex.compute();
                    let child_w = child_rects.iter().map(|r| r.x() + r.width()).fold(0.0f32, |a, b| a.max(b));
                    let child_h = child_rects.iter().map(|r| r.y() + r.height()).fold(0.0f32, |a, b| a.max(b));
                    (child_w, child_h)
                }
            };

            if self.wrap == FlexWrap::Wrap && current_x + w > self.width && current_x > 0.0 {
                current_x = 0.0;
                current_y += row_max_h + self.gap;
                row_max_h = 0.0;
            }

            rects.push(Rect::from_xywh(current_x, current_y, w, h));
            current_x += w + self.gap;
            row_max_h = row_max_h.max(h);
        }
        rects
    }

    fn compute_column(&self) -> Vec<Rect> {
        let mut rects = Vec::new();
        let mut current_y = 0.0f32;

        for item in &self.items {
            let (w, h) = match item {
                FlexItem::Fixed { width, height } => (*width, *height),
                FlexItem::Flexed { flex, min_height } => {
                    let h = (self.height / flex).max(*min_height);
                    (self.width, h)
                }
                FlexItem::Container(flex) => {
                    let child_rects = flex.compute();
                    let child_w = child_rects.iter().map(|r| r.x() + r.width()).fold(0.0f32, |a, b| a.max(b));
                    let child_h = child_rects.iter().map(|r| r.y() + r.height()).fold(0.0f32, |a, b| a.max(b));
                    (child_w, child_h)
                }
            };

            rects.push(Rect::from_xywh(0.0, current_y, w, h));
            current_y += h + self.gap;
        }
        rects
    }

    /// 计算总高度
    pub fn total_height(&self) -> f32 {
        let rects = self.compute();
        rects.iter().map(|r| r.y() + r.height()).fold(0.0f32, |a, b| a.max(b))
    }
}
