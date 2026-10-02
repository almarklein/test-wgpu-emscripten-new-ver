use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub async fn request_adapter_js() -> Result<JsValue, JsValue> {
    let global = js_sys::global();

    let navigator =
        js_sys::Reflect::get(&global, &"navigator".into())?;

    let gpu =
        js_sys::Reflect::get(&navigator, &"gpu".into())?;

    if gpu.is_undefined() {
        return Err("navigator.gpu is undefined".into());
    }

    let request_adapter =
        js_sys::Reflect::get(&gpu, &"requestAdapter".into())?;

    let promise =
        js_sys::Function::from(request_adapter).call0(&gpu)?;

    let adapter =
        JsFuture::from(js_sys::Promise::from(promise)).await?;

    if adapter.is_null() || adapter.is_undefined() {
        return Err("requestAdapter() returned null".into());
    }

    // Return raw adapter
    Ok(adapter)
}

#[wasm_bindgen]
pub fn backend_features() -> String {
    format!("{:?}", wgpu::Instance::enabled_backend_features())
}

#[wasm_bindgen]
pub async fn request_adapter_wgpu() -> Result<JsValue, JsValue> {

    let instance =  wgpu::Instance::new(
        wgpu::InstanceDescriptor::new_without_display_handle(),

    );

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e:?}")))?;

    // Just return the adapter's info as a simple JS value for testing.
    let info = adapter.get_info();
    Ok(JsValue::from_str(&format!("{info:?}")))

}


fn main() {}
