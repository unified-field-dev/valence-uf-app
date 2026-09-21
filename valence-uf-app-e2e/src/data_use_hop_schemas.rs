//! Minimal schemas so Referenced Reads E2E can open hop peer/initiator pages.
//!
//! Peer attribution uses SchemaRegistry connections (valence-app bake leaves
//! `connection_edges` empty). Keep these connections aligned with
//! `data_use_catalog_fixtures.rs` method names and e2e `build.rs` edges.

#![allow(dead_code)]

use valence::{valence_schema, Database, DatabaseFromEngine, MEM_ENGINE_ID};

const HOP_DB: DatabaseFromEngine = Database::from_engine("default", MEM_ENGINE_ID);

valence_schema! {
    Todo {
        repository: "https://github.com/unified-field-dev/valence-uf-app",
        table: "todo",
        version: "0.1.0",
        description: "E2E hop initiator for Referenced Reads",
        database: HOP_DB,
        fields: [
            id: { r#type: FieldType::String, primary_key: true, required: true },
            owner: { r#type: FieldType::Record("user"), required: false },
        ],
        connections: [
            owner: {
                table: "user",
                cardinality: HasOne,
                required: false,
                on_delete: SetNull,
            },
            tags: {
                table: "tag",
                cardinality: ManyToMany,
                edge_table: "todo_tag",
                on_delete: Cascade,
            },
        ],
    }
}

valence_schema! {
    Project {
        repository: "https://github.com/unified-field-dev/valence-uf-app",
        table: "project",
        version: "0.1.0",
        description: "E2E HasMany hop initiator",
        database: HOP_DB,
        fields: [
            id: { r#type: FieldType::String, primary_key: true, required: true },
            owner: { r#type: FieldType::Record("user"), required: false },
        ],
        connections: [
            owner: {
                table: "user",
                cardinality: HasOne,
                required: false,
                on_delete: SetNull,
            },
            tasks: {
                table: "task",
                cardinality: HasMany,
                reverse_field: "project",
                on_delete: Cascade,
            },
        ],
    }
}

valence_schema! {
    Task {
        repository: "https://github.com/unified-field-dev/valence-uf-app",
        table: "task",
        version: "0.1.0",
        description: "E2E HasMany hop peer",
        database: HOP_DB,
        fields: [
            id: { r#type: FieldType::String, primary_key: true, required: true },
            project: { r#type: FieldType::Record("project"), required: false },
        ],
        connections: [
            project: {
                table: "project",
                cardinality: HasOne,
                required: false,
                on_delete: SetNull,
            },
        ],
    }
}

valence_schema! {
    Tag {
        repository: "https://github.com/unified-field-dev/valence-uf-app",
        table: "tag",
        version: "0.1.0",
        description: "E2E M2M hop peer",
        database: HOP_DB,
        fields: [
            id: { r#type: FieldType::String, primary_key: true, required: true },
        ],
    }
}
