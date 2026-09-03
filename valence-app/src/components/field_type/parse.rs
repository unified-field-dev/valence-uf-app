//! Pure parsing for Valence `field_type` metadata strings.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedFieldType {
    Plain(String),
    InlineEnum { name: String, variants: Vec<String> },
    ExternalEnum { name: String, path: String },
}

pub fn parse_field_type(field_type: &str, context_name: &str, field_name: &str) -> ParsedFieldType {
    if let Some(rest) = field_type.strip_prefix("enum:") {
        let variants: Vec<String> = rest
            .split(',')
            .filter(|v| !v.is_empty())
            .map(str::to_string)
            .collect();
        let name = inline_enum_name(context_name, field_name);
        return ParsedFieldType::InlineEnum { name, variants };
    }

    if let Some(path) = field_type.strip_prefix("ext_enum:") {
        let name = path.rsplit("::").next().unwrap_or(path).to_string();
        return ParsedFieldType::ExternalEnum {
            name,
            path: path.to_string(),
        };
    }

    ParsedFieldType::Plain(field_type.to_string())
}

fn inline_enum_name(context_name: &str, field_name: &str) -> String {
    format!(
        "{}{}",
        to_pascal_case(context_name),
        to_pascal_case(field_name)
    )
}

pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_inline_enum_variants() {
        let parsed = parse_field_type("enum:PENDING,IN_PROGRESS,COMPLETED", "test_status", "phase");
        assert_eq!(
            parsed,
            ParsedFieldType::InlineEnum {
                name: "TestStatusPhase".to_string(),
                variants: vec![
                    "PENDING".to_string(),
                    "IN_PROGRESS".to_string(),
                    "COMPLETED".to_string(),
                ],
            }
        );
    }

    #[test]
    fn parses_account_plan_enum_name() {
        let parsed = parse_field_type(
            "enum:free,starter,professional,enterprise",
            "account",
            "plan",
        );
        assert!(matches!(
            parsed,
            ParsedFieldType::InlineEnum {
                name,
                ..
            } if name == "AccountPlan"
        ));
    }

    #[test]
    fn parses_external_enum() {
        let parsed = parse_field_type("ext_enum:crate::ColorEnum", "test_ext_enum", "color");
        assert_eq!(
            parsed,
            ParsedFieldType::ExternalEnum {
                name: "ColorEnum".to_string(),
                path: "crate::ColorEnum".to_string(),
            }
        );
    }

    #[test]
    fn plain_type_passthrough() {
        let parsed = parse_field_type("record<user>", "account", "owner");
        assert_eq!(parsed, ParsedFieldType::Plain("record<user>".to_string()));
    }

    #[test]
    fn to_pascal_case_splits_snake() {
        assert_eq!(to_pascal_case("test_status"), "TestStatus");
        assert_eq!(to_pascal_case("plan"), "Plan");
    }
}
