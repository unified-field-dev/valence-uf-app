//! Data-use probe binary that is a workspace member but never declared by the e2e
//! server. Its purpose must stay out of the catalog: only components the host names
//! count.

#[allow(clippy::missing_const_for_fn, reason = "main cannot be const")]
fn main() {}

// Never compiled; present so the scan would find it if it wrongly read this crate.
#[cfg(any())]
mod declared_uses {
    async fn read(valence: &valence::Valence) {
        let _ = DataUseProbeWidget::get(
            "probe-1",
            valence,
            valence::use_!(r#"**Test:** Probe **unwired read** PROBE_UNWIRED_MUST_NOT_APPEAR so the e2e suite proves undeclared binaries stay out of the catalog."#),
        )
        .await;
    }
}
