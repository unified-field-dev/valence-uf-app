//! Data-use probe worker binary.
//!
//! Stands in for a Chronon, Boson, or Photon runtime that runs in its own process.
//! The e2e server never links it; it lists the worker under
//! `[target.'cfg(any())'.dependencies]` so its declared uses join the catalog.

#[allow(clippy::missing_const_for_fn, reason = "main cannot be const")]
fn main() {}

// Never compiled; the data-use scan reads it from source.
#[cfg(any())]
mod declared_uses {
    async fn tick(valence: &valence::Valence, row: DataUseProbeWidget) {
        let _ = DataUseProbeWidget::update(
            "probe-1",
            row,
            valence,
            valence::use_!(r#"**Test:** Probe **worker update** on the widget so the e2e suite proves an out-of-process binary reaches the catalog."#),
        )
        .await;
    }
}
