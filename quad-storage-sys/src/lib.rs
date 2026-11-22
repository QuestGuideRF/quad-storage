use sapp_jsutils::{JsObject, JsObjectWeak};
#[no_mangle]
extern "C" fn quad_storage_crate_version() -> u32 {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();
    (major << 24) + (minor << 16) + patch
}
extern "C" {
    fn quad_storage_length() -> u32;
    fn quad_storage_has_key(i: u32) -> u32;
    fn quad_storage_key(i: u32) -> JsObject;
    fn quad_storage_has_value(key: JsObjectWeak) -> u32;
    fn quad_storage_get(key: JsObjectWeak) -> JsObject;
    fn quad_storage_set(key: JsObjectWeak, value: JsObjectWeak);
    fn quad_storage_remove(key: JsObjectWeak);
    fn quad_storage_clear();
}
fn js_to_option_string(object: JsObject, has: u32) -> Option<String> {
    if has == 1 {
        let mut result = String::new();
        object.to_string(&mut result);
        Some(result)
    } else {
        None
    }
}
pub fn len() -> usize {
    unsafe { quad_storage_length() as usize }
}
pub fn key(pos: usize) -> Option<String> {
    js_to_option_string(unsafe { quad_storage_key(pos as u32) }, unsafe {
        quad_storage_has_key(pos as u32)
    })
}
pub fn get(key: &str) -> Option<String> {
    let object = JsObject::string(key);
    let weak_object = object.weak();
    let result = js_to_option_string(unsafe { quad_storage_get(weak_object) }, unsafe {
        quad_storage_has_value(weak_object)
    });
    drop(object);
    result
}
pub fn set(key: &str, value: &str) {
    unsafe {
        quad_storage_set(JsObject::string(key).weak(), JsObject::string(value).weak());
    }
}
pub fn remove(key: &str) {
    unsafe {
        quad_storage_remove(JsObject::string(key).weak());
    }
}
pub fn clear() {
    unsafe {
        quad_storage_clear();
    }
}