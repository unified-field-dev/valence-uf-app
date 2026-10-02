//! Data-use probe product.
//!
//! Sits outside the valence-uf-app workspace (`[workspace] exclude`) and is linked
//! by the e2e server as a path dependency, the way a deployment links a product
//! crate from another repository. Its declared uses must reach the e2e catalog.

use valence::{valence_schema, Database, DatabaseFromEngine, MEM_ENGINE_ID};

const PROBE_DB: DatabaseFromEngine = Database::from_engine("default", MEM_ENGINE_ID);

/// Table every probe use targets.
pub const PROBE_TABLE: &str = "data_use_probe_widget";

valence_schema! {
    DataUseProbeWidget {
        repository: "https://github.com/unified-field-dev/valence-uf-app",
        table: "data_use_probe_widget",
        version: "0.1.0",
        description: "Data-use probe schema for deployment catalog E2E",
        database: PROBE_DB,
        fields: [
            id: { r#type: FieldType::String, primary_key: true, required: true },
            label: { r#type: FieldType::String, required: false },
        ],
    }
}

/// Registered table that nothing declares a use on, for the empty Data uses state.
pub const UNUSED_TABLE: &str = "data_use_probe_unused";

valence_schema! {
    DataUseProbeUnused {
        repository: "https://github.com/unified-field-dev/valence-uf-app",
        table: "data_use_probe_unused",
        version: "0.1.0",
        description: "Data-use probe schema with no declared uses",
        database: PROBE_DB,
        fields: [
            id: { r#type: FieldType::String, primary_key: true, required: true },
        ],
    }
}

// The probe has no generated Model, so these calls are never compiled. The data-use
// scan reads source text, so they still land in the catalog.
#[cfg(any())]
mod declared_uses {
    async fn read_widget(valence: &valence::Valence) {
        let _ = DataUseProbeWidget::get(
            "probe-1",
            valence,
            valence::use_!(r#"**Test:** Probe **product read** on the widget so the e2e suite proves a crate outside the workspace reaches the catalog."#),
        )
        .await;
    }

    async fn create_widget(valence: &valence::Valence, row: DataUseProbeWidget) {
        let _ = DataUseProbeWidget::create(
            row,
            valence,
            valence::use_!(r#"**Test:** Probe **product create** on the widget so the e2e suite proves linked product writes reach the catalog."#),
        )
        .await;
    }
}
