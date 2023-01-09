use wasm_bindgen::prelude::*;

use wasm_bindgen::{prelude::wasm_bindgen};

mod WasmPtr{
  macro_rules! wasm_ptr_transform {
    ($ptr: expr) => {
      unsafe {
        use wasm_bindgen::__rt::WasmRefCell;
        let js: &mut WasmRefCell<T> = Box::leak(Box::from_raw($ptr as *mut WasmRefCell<T>));
        js.get_mut()
      }
    };
  }
  
  // js ptr to rust ptr mut
  pub fn transform_wasm_ptr_mut<T>(addr: *mut T) -> *mut T  {
    wasm_ptr_transform!(addr)
  }
  
  // js ptr to rust ptr
  pub fn transform_wasm_ptr<T>(addr: *const T) -> *const T  {
    wasm_ptr_transform!(addr)
  }
  
  // get object mut ref from ptr
  pub fn leak_wasm_ptr_mut<T>(addr: *mut T) -> &'static mut T {
    wasm_ptr_transform!(addr)
  }
  
  // get object ref from ptr
  pub fn leak_wasm_ptr<T>(addr: *const T) -> &'static T {
    wasm_ptr_transform!(addr)
  }
}


#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  // The `console.log` is quite polymorphic, so we can bind it with multiple
  // signatures. Note that we need to use `js_name` to ensure we always call
  // `log` in JS.
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_u32(a: u32);

  // Multiple arguments too!
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_many(a: &str, b: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}

mod style;
mod node;
mod animation;
mod refresh;
mod math;
