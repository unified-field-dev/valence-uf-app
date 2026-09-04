use orbital::primitives::DataTableColumnDef;

pub fn trait_table_columns() -> Vec<DataTableColumnDef> {
    vec![
        DataTableColumnDef::new("name", "Trait").with_sortable(false),
        DataTableColumnDef::new("version", "Version").with_sortable(false),
        DataTableColumnDef::new("description", "Description").with_sortable(false),
    ]
}
