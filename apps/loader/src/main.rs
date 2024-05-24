#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]
#[cfg(feature = "axstd")]
use axstd::println;
#[cfg(feature = "axstd")]
use axstd::apps_image::Image;
#[cfg(feature = "axstd")]
use axstd::vec::Vec;
const PLASH_START: usize = 0x22000000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;

    println!("Load payload ...");

    let apps_head = unsafe {
        core::ptr::read(apps_start as *const Image)
    };

    let code = unsafe { core::slice::from_raw_parts(apps_start , 8) };

    const RUN_START: usize = 0xffff_ffc0_8010_0000;

    if apps_head.app1_size != 0 {
        println!("APP1_CONTENT");
        let mut code = unsafe { core::slice::from_raw_parts(apps_start.add(apps_head.app1_offset), apps_head.app1_size) };

        println!("load code {:?}; address [{:?}]", code, code.as_ptr());

        let data : Vec<u8>;
        if code.len() % 8 != 0 {
            let x = 8 - ( code.len() % 8 );
            data = pad_slice_with_zeros(code,  x + code.len() );
            code = &data;
        }
        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, code.len())
        };
        run_code.copy_from_slice(code);
        println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());
        
        println!("====================APP1_START_RUN====================");
        unsafe { core::arch::asm!("
            li      t2, {run_start}
            jalr    ra , t2 , 0 ",
            run_start = const RUN_START,
        )}
        println!("====================APP1_RETURN====================");

        println!("Content:");
        while code.len() != 0 {
            println!("0x{:016x}", bytes_to_usize(&code[..8]));
            code = &code[8..code.len()];
        }
    }
    if apps_head.app2_size != 0 {
        println!("APP2_CONTENT");
        let mut code = unsafe { core::slice::from_raw_parts(apps_start.add(apps_head.app2_offset), apps_head.app2_size) };

        println!("load code {:?}; address [{:?}]", code, code.as_ptr());

        let data : Vec<u8>;
        if code.len() % 8 != 0 {
            let x = 8 - ( code.len() % 8 );
            data = pad_slice_with_zeros(code,  x + code.len() );
            code = &data;
        }


        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, code.len())
        };
        run_code.copy_from_slice(code);
        println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());

        println!("====================APP2_START_RUN====================");
        unsafe { core::arch::asm!("
            li      t2, {run_start}
            jalr    ra , t2 , 0 ",
            run_start = const RUN_START,
        )}
        println!("====================APP2_RETURN====================");

        println!("Content:");
        while code.len() != 0 {
            println!("0x{:016x}", bytes_to_usize(&code[..8]));
            code = &code[8..code.len()];
        }
    }

    println!("Load payload ok!");
}

#[inline]
fn bytes_to_usize(bytes: &[u8]) -> usize {
    usize::from_be_bytes(bytes.try_into().unwrap())
}

fn pad_slice_with_zeros(slice: &[u8], target_len: usize) -> Vec<u8> {
    let mut padded_vec = slice.to_vec();
    padded_vec.resize(target_len, 0);
    padded_vec
}