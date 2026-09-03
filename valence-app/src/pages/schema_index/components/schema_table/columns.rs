use orbital::primitives::DataTableColumnDef;

pub fn schema_table_columns() -> Vec<DataTableColumnDef> {
    vec![
        DataTableColumnDef::new("name", "Schema").with_sortable(false),
        DataTableColumnDef::new("databases", "Database").with_sortable(false),
        DataTableColumnDef::new("version", "Version").with_sortable(false),
        DataTableColumnDef::new("description", "Description").with_sortable(false),
    ]
}
