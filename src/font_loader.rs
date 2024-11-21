use std::error::Error;
use std::fmt::Display;

use anyhow::Result;
use bevy::asset::{
    Asset, AssetLoader, AsyncReadExt, LoadContext,
    io::Reader,
};
use bevy::utils::ConditionalSendFuture;
use bevy::reflect::TypePath;

#[derive(Debug)]
pub struct FontLoaderError;

impl Error for FontLoaderError {}

impl Display for FontLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("FontLoaderError")
    }
}

#[derive(Default)]
pub struct FontLoader;

impl AssetLoader for FontLoader {
    /// Bevy doesn't allow multiple [`AssetLoader`]s for the same extension anymore.
    /// So now we're returning our own type so Bevy can determine which loader to
    /// use without binding an extension to our loader.
    type Asset = TextMeshFont;
    type Settings = ();
    type Error = FontLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader
                .read_to_end(&mut bytes)
                .await
                .expect("unable to read font");

            // ttf fontloading
            let font = TextMeshFont {
                ttf_font: ttf2mesh::TTFFile::from_buffer_vec(bytes.clone())
                    .expect("unable to decode asset"),
            };

            Ok(font)
        })
    }

    fn extensions(&self) -> &[&str] {
        &[]
    }
}

#[derive(TypePath, Asset)]
// #[uuid = "5415ac03-d009-471e-89ab-dc0d4e31a8c4"]
pub struct TextMeshFont {
    pub(crate) ttf_font: ttf2mesh::TTFFile,
}

impl std::fmt::Debug for TextMeshFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TextMeshFont<>")
    }
}

unsafe impl Sync for TextMeshFont {} // FIXME - verify the soundness
unsafe impl Send for TextMeshFont {} // FIXME - verify the soundness
