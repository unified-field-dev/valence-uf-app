use std::sync::OnceLock;

use turf::inline_style_sheet_values;

/// CSS class names reused across connection, fields, and trait cards.
#[derive(Clone)]
pub struct CodeStyleClasses {
    pub sheet: String,
    pub code: String,
    pub subtle: String,
    pub list_item: String,
    pub link_wrap: String,
    pub link_button: String,
    pub meta_row: String,
}

static STYLES: OnceLock<CodeStyleClasses> = OnceLock::new();

pub fn code_style_classes() -> &'static CodeStyleClasses {
    STYLES.get_or_init(|| {
        let (style_sheet, class_names) = inline_style_sheet_values! {
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

            .ListItem {
                gap: 8px;
                align-items: center;
                flex-wrap: wrap;
                min-width: 0;
            }

            .LinkWrap {
                min-width: 0;
                max-width: min(100%, 280px);
                overflow: hidden;
            }

            .LinkButton {
                max-width: 100%;
                overflow: hidden;
                text-overflow: ellipsis;
                white-space: nowrap;
                display: inline-block;
            }

            .MetaRow {
                padding-inline-start: var(--orb-space-inline-xl);
                gap: 8px;
                align-items: center;
                flex-wrap: wrap;
            }
        };

        CodeStyleClasses {
            sheet: style_sheet.to_string(),
            code: class_names.code.to_string(),
            subtle: class_names.subtle.to_string(),
            list_item: class_names.list_item.to_string(),
            link_wrap: class_names.link_wrap.to_string(),
            link_button: class_names.link_button.to_string(),
            meta_row: class_names.meta_row.to_string(),
        }
    })
}
