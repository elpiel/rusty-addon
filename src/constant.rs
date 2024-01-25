use once_cell::sync::Lazy;
use semver::Version;

use stremio_core::types::addon::{
    ExtraProp, Manifest, ManifestCatalog, ManifestExtra, ManifestResource, OptionsLimit,
};

pub const MANIFEST: Lazy<Manifest> = Lazy::new(|| Manifest {
    id: "org.stremio.rusty-addon".into(),
    name: "Rusty addon".into(),
    version: Version::new(0, 1, 0),
    resources: vec![
        ManifestResource::Short("catalog".into()),
        ManifestResource::Short("stream".into()),
    ],
    types: vec!["movie".into(), "series".into()],
    catalogs: vec![
        ManifestCatalog {
            r#type: "others".into(),
            id: "bbbcatalog".into(),
            name: Some("Rust test".into()),
            extra: ManifestExtra::default(),
        },
        ManifestCatalog {
            r#type: "series".into(),
            id: "last-videos".into(),
            name: Some("lastVideos".into()),
            extra: ManifestExtra::Full {
                props: vec![ExtraProp {
                    name: "lastVideosIds".to_string(),
                    is_required: true,
                    options: vec![],
                    options_limit: OptionsLimit(100),
                }],
            },
        },
    ],
    id_prefixes: Some(vec!["tt".into(), "kitsu".into()]),
    description: Some("Rust addon test".into()),
    contact_email: Default::default(),
    logo: Default::default(),
    background: Default::default(),
    addon_catalogs: Default::default(),
    behavior_hints: Default::default(),
});
