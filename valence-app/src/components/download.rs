/// Browser download helpers (WASM only).
#[cfg(target_arch = "wasm32")]
pub fn download_text_file(filename: &str, content: &str, mime: &str) {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    let array = js_sys::Uint8Array::from(content.as_bytes());
    let parts = js_sys::Array::new();
    parts.push(&array);

    let mut props = BlobPropertyBag::new();
    props.set_type(mime);

    let Ok(blob) = Blob::new_with_u8_array_sequence_and_options(&parts, &props) else {
        return;
    };
    let Ok(url) = Url::create_object_url_with_blob(&blob) else {
        return;
    };

    if let Ok(anchor) = document.create_element("a") {
        if let Ok(anchor) = anchor.dyn_into::<HtmlAnchorElement>() {
            anchor.set_href(&url);
            anchor.set_download(filename);
            let _ = anchor.click();
        }
    }
    let _ = Url::revoke_object_url(&url);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn download_text_file(_filename: &str, _content: &str, _mime: &str) {}
