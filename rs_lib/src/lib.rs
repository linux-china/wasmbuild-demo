use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn hello(name: &str) -> String {
    return format!("Hello, {}!", name);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyIp {
    pub origin: String,
}

#[wasm_bindgen]
pub async fn myip() -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = "https://httpbin.org/ip";
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // `resp_value` is a `Response` object.
    let resp: Response = resp_value.dyn_into().unwrap();
    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    // Use serde to parse the JSON into a struct.
    let my_ip: MyIp = json.into_serde().unwrap();
    Ok(my_ip.origin)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(1, 2);
        assert_eq!(result, 3);
    }
}
