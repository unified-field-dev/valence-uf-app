use std::sync::OnceLock;

use turf::inline_style_sheet_values;

/// Shared bordered-table surface styles for Valence schema and entity pages.
pub struct BorderedTableStyles {
    pub sheet: String,
    pub table_wrap: String,
    pub table_scroller: String,
    pub compact_table: String,
    pub code: String,
    pub subtle: String,
    pub redacted: String,
}

static STYLES: OnceLock<BorderedTableStyles> = OnceLock::new();

pub fn bordered_table_styles() -> &'static BorderedTableStyles {
    STYLES.get_or_init(|| {
        let (sheet, names) = inline_style_sheet_values! {
            .TableWrap {
                border: 1px solid var(--orb-color-border-muted);
                border-radius: 8px;
                overflow: hidden;
            }

            .TableScroller {
                overflow-x: auto;
            }

            .CompactTable {
                width: 100%;
                table-layout: fixed;
            }

            .Code {
                font-family: var(--orb-type-family-mono);
                font-size: var(--orb-type-size-sm);
                background-color: var(--orb-color-surface-subtle);
                padding: 2px 6px;
                border-radius: 4px;
            }

            .Subtle {
                color: var(--orb-color-text-tertiary);
            }

            .Redacted {
                color: var(--orb-color-text-tertiary);
                font-style: italic;
                letter-spacing: 2px;
            }
        };

        BorderedTableStyles {
            sheet: sheet.to_string(),
            table_wrap: names.table_wrap.to_string(),
            table_scroller: names.table_scroller.to_string(),
            compact_table: names.compact_table.to_string(),
            code: names.code.to_string(),
            subtle: names.subtle.to_string(),
            redacted: names.redacted.to_string(),
        }
    })
}
