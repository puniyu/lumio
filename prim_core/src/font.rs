use std::sync::LazyLock;

use skia_safe::textlayout::{FontCollection, TypefaceFontProvider};

mod store;

static FONT_STORE: LazyLock<store::FontStore> = LazyLock::new(store::FontStore::new);

pub struct FontManger {
    font_collection: FontCollection,
}

impl Default for FontManger {
    fn default() -> Self {
        Self::new()
    }
}

impl FontManger {
    pub fn new() -> Self {
        let mut font_collection = FontCollection::new();
        let mut font_provider = TypefaceFontProvider::new();
        for (name, typeface) in (&*FONT_STORE).into_iter() {
            font_provider.register_typeface(typeface.clone(), Some(name.as_str()));
        }
        font_collection.set_asset_font_manager(Some(font_provider.into()));

        Self { font_collection }
    }

    pub fn font_collection(&self) -> &FontCollection {
        &self.font_collection
    }
}
