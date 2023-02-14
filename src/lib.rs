use wasm_bindgen::prelude::*;

mod wasm_ptr {
  macro_rules! wasm_ptr_transform {
    /*
      rust传给js的对象，是一个WasmRefCell, 来自这个包：use wasm_bindgen::__rt::WasmRefCell;
          pub struct WasmRefCell<T: ?Sized> {
              borrow: Cell<usize>,
              value: UnsafeCell<T>,
          }
      如果通过对象的指针获取对象那么需要解引用WasmRefCell，不能除暴的用指针 + borrow的offset来获取value，因为编译器会对齐字节。
    */
    ($ptr: expr) => {
      unsafe {
        use wasm_bindgen::__rt::WasmRefCell;
        let js: &mut WasmRefCell<T> = Box::leak(Box::from_raw($ptr as *mut WasmRefCell<T>));
        js.get_mut()
      }
    };
  }


  pub fn transform_mut<T>(addr: *mut T) -> *mut T  {
    wasm_ptr_transform!(addr)
  }

  // js ptr to rust ptr
  pub fn transform<T>(addr: *const T) -> *const T {
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
