#![no_main]
#![no_std]

//#[cfg(all(unix, target_arch = "wasm32"))]
//use core::wasm32::unreachable;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
    //unreachable!()
}

// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     loop {}
// }

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn drawRotozoom(pixels: &mut [u8]) {
    let count = pixels.len();

    // TODO: implement the rotation effect ...

    for idx in (0..count).step_by(4) {
        pixels[idx] = 0xff;     // Red color component
        pixels[idx + 1] = 0xff; // Green color component
        pixels[idx + 2] = 0;    // Blue color component
        pixels[idx + 3] = 0xff; // Alpha color component
    }
}
