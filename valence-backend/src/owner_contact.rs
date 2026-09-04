//! Owner-card contact resolution under viewer-scoped privacy.

/// Build `(name, email, handle)` from a privacy-filtered user record.
///
/// When `email` is absent from the filtered record (viewer cannot read it),
/// email is redacted as `"—"` instead of echoing `owner_id`.
#[must_use]
pub fn owner_contact_from_user_record(
    user: Option<&serde_json::Value>,
    owner_id: &str,
) -> (String, String, String) {
    let handle = owner_id.to_string();
    match user.and_then(|u| u.get("email")).and_then(|v| v.as_str()) {
        Some(email) if !email.is_empty() => (email.to_string(), email.to_string(), handle),
        _ => (owner_id.to_string(), "—".to_string(), handle),
    }
}

#[cfg(test)]
mod tests {
    use super::owner_contact_from_user_record;
    use serde_json::json;

    #[test]
    fn visible_email_used_happy_path() {
        let user = json!({ "email": "a@example.com" });
        let (name, email, handle) = owner_contact_from_user_record(Some(&user), "user1");
        assert_eq!(name, "a@example.com");
        assert_eq!(email, "a@example.com");
        assert_eq!(handle, "user1");
    }

    #[test]
    fn missing_email_redacted_sad() {
        let user = json!({ "id": "user1" });
        let (name, email, handle) = owner_contact_from_user_record(Some(&user), "user1");
        assert_eq!(name, "user1");
        assert_eq!(email, "—");
        assert_eq!(handle, "user1");
    }

    #[test]
    fn absent_user_record_redacted_sad() {
        let (name, email, handle) = owner_contact_from_user_record(None, "user1");
        assert_eq!(name, "user1");
        assert_eq!(email, "—");
        assert_eq!(handle, "user1");
    }
}
