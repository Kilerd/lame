use lame_sys::ffi;
use std::ffi::CStr;

fn main() {
    println!("Testing LAME CPU feature detection...\n");

    unsafe {
        let gfp = ffi::lame_init();
        if gfp.is_null() {
            eprintln!("Failed to initialize LAME");
            return;
        }

        // Set basic parameters
        ffi::lame_set_in_samplerate(gfp, 44100);
        ffi::lame_set_num_channels(gfp, 1);
        ffi::lame_set_brate(gfp, 192);
        ffi::lame_set_quality(gfp, 4);

        println!("Initializing LAME parameters...");
        println!("(This should print CPU features detected)\n");

        // Initialize - this triggers CPU detection
        let result = ffi::lame_init_params(gfp);
        if result < 0 {
            eprintln!("Failed to initialize parameters");
            ffi::lame_close(gfp);
            return;
        }

        println!("\n===========================================");
        println!("LAME Version: {}", CStr::from_ptr(ffi::get_lame_version()).to_str().unwrap());
        println!("===========================================");

        // Test encode to ensure everything works
        let pcm: Vec<i16> = vec![0; 1152];
        let mut mp3_buffer: Vec<u8> = vec![0; 8192];

        let bytes = ffi::lame_encode_buffer(
            gfp,
            pcm.as_ptr(),
            std::ptr::null(),
            1152,
            mp3_buffer.as_mut_ptr(),
            8192,
        );

        if bytes >= 0 {
            println!("Test encode successful: {} bytes", bytes);
        } else {
            println!("Test encode failed: {}", bytes);
        }

        ffi::lame_close(gfp);
    }

    println!("\nCheck the output above for 'CPU features:' line");
    println!("It should list: MMX, SSE, SSE2, AVX, AVX2, FMA");
}
