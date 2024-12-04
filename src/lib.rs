use base64::{engine::general_purpose, Engine as _};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
#[wasm_bindgen]
pub fn grayscale(encode: &str) -> String {
    log(&"Graysacle".into());

    let base64_to_vector = general_purpose::STANDARD.decode(encode);

    let mut image = match base64_to_vector {
        Ok(value) => load_from_memory(&value).unwrap(),

        Err(err) => {
            log(&format!("Error: {:?}", err).into());
            return "Error".into();
        }
    };
    log(&"Image loaded".into());

    image = image.grayscale();
    log(&"Graysacle effect successful".into());

    let mut buffer = vec![];
    if let Err(err) = image.write_to(&mut buffer, Png) {
        log(&format!("Error writing image: {:?}", err).into());
        return "Error".into();
    }

    log(&"New image created".into());

    let encoded_img = general_purpose::STANDARD.encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_img);
    return data_url;
}
