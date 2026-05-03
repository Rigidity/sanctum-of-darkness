use std::marker::PhantomData;

use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use serde::de::DeserializeOwned;

#[derive(Default, TypePath)]
pub struct Json5AssetLoader<T>(PhantomData<T>);

impl<T> AssetLoader for Json5AssetLoader<T>
where
    T: DeserializeOwned + Asset + Json5Extension,
{
    type Asset = T;
    type Settings = ();
    type Error = BevyError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset> {
        let mut bytes = String::new();
        reader.read_to_string(&mut bytes).await?;
        Ok(json5::from_str(&bytes)?)
    }

    fn extensions(&self) -> &[&str] {
        &[T::EXTENSION]
    }
}

pub trait Json5Extension {
    const EXTENSION: &'static str;
}
