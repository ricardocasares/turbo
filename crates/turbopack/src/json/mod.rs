use crate::{
    asset::{Asset, AssetVc},
    reference::AssetReferencesSetVc,
};
use turbo_tasks_fs::{FileContentVc, FileSystemPathVc};

#[turbo_tasks::value(Asset)]
#[derive(PartialEq, Eq)]
pub struct ModuleAsset {
    pub source: AssetVc,
}

#[turbo_tasks::value_impl]
impl ModuleAssetVc {
    pub fn new(source: AssetVc) -> Self {
        Self::slot(ModuleAsset {
            source: source.clone(),
        })
    }
}

#[turbo_tasks::value_impl]
impl Asset for ModuleAsset {
    fn path(&self) -> FileSystemPathVc {
        self.source.clone().path()
    }
    fn content(&self) -> FileContentVc {
        self.source.clone().content()
    }
    async fn references(&self) -> AssetReferencesSetVc {
        AssetReferencesSetVc::empty()
    }
}