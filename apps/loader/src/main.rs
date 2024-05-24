#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]

#[cfg(feature = "axstd")]
use axstd::{apps_image::Image , println};
#[cfg(feature = "axstd")]
use axstd::{ vec::Vec , vec , exit};


const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_SHUTDOWN: usize = 3;
static mut ABI_TABLE: [usize; 16] = [0; 16];
const PLASH_START: usize = 0x22000000;

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
    return;
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
    return;
}

fn abi_shutdown(){
    exit(0);
}

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
        let mut code = unsafe { core::slice::from_raw_parts(apps_start.add(apps_head.app1_offset), apps_head.app1_size) };
        let mut data : Vec<u8> = vec![];
        code = paddle_to_usize( &mut data , code );

        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, code.len())
        };
        run_code.copy_from_slice(code);
        


        register_abi(SYS_HELLO, abi_hello as usize);
        register_abi(SYS_PUTCHAR, abi_putchar as usize);
        register_abi(SYS_SHUTDOWN, abi_shutdown as usize);
    
        println!("Execute Shut_down ...");
        let arg0: u8 = b'A';
    
        // execute app
        unsafe { core::arch::asm!("
            addi sp, sp, -16*8
            sd ra, 120(sp)
            sd t0, 112(sp)
            sd t1, 104(sp)
            sd t2, 96(sp)
            sd t3, 88(sp)
            sd t4, 80(sp)
            sd t5, 72(sp)
            sd t6, 64(sp)
            sd a0, 56(sp)
            sd a1, 48(sp)
            sd a2, 40(sp)
            sd a3, 32(sp)
            sd a4, 24(sp)
            sd a5, 16(sp)
            sd a6, 8(sp)
            sd a7, 0(sp)
            li      t0, {abi_num}
            slli    t0, t0, 3
            la      t1, {abi_table}
            add     t1, t1, t0
            ld      t1, (t1)
            jalr    ra , t1 , 0
            
            li      t2, {run_start}
            jalr    ra , t2 , 0
            ld ra, 120(sp)
            ld t0, 112(sp)
            ld t1, 104(sp)
            ld t2, 96(sp)
            ld t3, 88(sp)
            ld t4, 80(sp)
            ld t5, 72(sp)
            ld t6, 64(sp)
            ld a0, 56(sp)
            ld a1, 48(sp)
            ld a2, 40(sp)
            ld a3, 32(sp)
            ld a4, 24(sp)
            ld a5, 16(sp)
            ld a6, 8(sp)
            ld a7, 0(sp)
            addi sp, sp, 16*8",
            run_start = const RUN_START,
            abi_table = sym ABI_TABLE,
            //abi_num = const SYS_HELLO,
            abi_num = const SYS_SHUTDOWN,
            in("a0") arg0,
        )}

        println!("====================APP1_START_RUN====================");
        unsafe { core::arch::asm!("
            li      t2, {run_start}
            jalr    ra , t2 , 0 ",
            run_start = const RUN_START,
        )}
        println!("====================APP1_RETURN====================");
    }
    if apps_head.app2_size != 0 {
        let mut code = unsafe { core::slice::from_raw_parts(apps_start.add(apps_head.app2_offset), apps_head.app2_size) };

        let mut data : Vec<u8> = vec![];
        code = paddle_to_usize( &mut data , code );

        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, code.len())
        };
        run_code.copy_from_slice(code);


        println!("====================APP2_START_RUN====================");
        unsafe { core::arch::asm!("
            li      t2, {run_start}
            jalr    ra , t2 , 0 ",
            run_start = const RUN_START,
        )}
        println!("====================APP2_RETURN====================");

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

fn paddle_to_usize<'a: 'c, 'b: 'c , 'c>( data : &'a mut Vec<u8> , code : &'b [u8] ) -> &'c [u8]{
    if code.len() % 8 != 0 {
        let x = 8 - ( code.len() % 8 );
        *data = pad_slice_with_zeros(code,  x + code.len() );
        data
    }
    else{
        code
    }
}