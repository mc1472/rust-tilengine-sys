use tilengine_sys::*;
use std::ffi::{CString, CStr};
use std::ptr;
unsafe fn panic_on_tln_error(){
    let error_code = TLN_GetLastError();
    if error_code != 0 {
        panic!("{:?}", CStr::from_ptr(TLN_GetErrorString(error_code)));
    }
}
fn main(){
    let load_path = CString::new("examples/assets/sonic").expect("Invalid load path");
    let load_path_ptr = load_path.as_ptr();
    let map_path = CString::new("Sonic_md_fg1.tmx").expect("Invalad map path");
    let map_path_ptr = map_path.as_ptr();
    unsafe {
        TLN_Init(400, 240, 1, 0, 20);
        panic_on_tln_error();
        TLN_SetLoadPath(load_path_ptr);
        panic_on_tln_error();
        let forgrund = TLN_LoadTilemap(map_path_ptr, ptr::null());
        panic_on_tln_error();
        TLN_SetLayer(0, ptr::null_mut(), forgrund);
        let mut frame = 0;
        TLN_CreateWindow(ptr::null(), 0);
        while TLN_ProcessWindow() == 1{
            TLN_DrawFrame(frame);
            frame += 1;
        }
        TLN_Deinit();
    }
}
