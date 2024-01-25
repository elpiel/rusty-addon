use stremio_core::types::addon::Manifest;

pub trait ManifestExt {
    /// Check in the Manifest whether the `id`` is supported by checking the each `id_prefixes`.
    fn is_id_supported(&self, id: &str) -> bool;
}

impl ManifestExt for Manifest {
    fn is_id_supported(&self, id: &str) -> bool {
        match self.id_prefixes.as_ref() {
            Some(supported_prefixes) => supported_prefixes
                .iter()
                .any(|prefix| id.starts_with(prefix)),
            // if no supported prefixes have been specified then all prefixes are supported
            None => true,
        }
    }
}
