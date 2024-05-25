#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]


#[cfg(feature = "axstd")]
use axstd::{apps_image::Image , println , print};
#[cfg(feature = "axstd")]
use axstd::exit;


const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_SHUTDOWN: usize = 3;
static mut ABI_TABLE: [usize; 16] = [0; 16];
static mut CALL_TABLE: [usize; 16] = [0; 16];
const PLASH_START: usize = 0x22000000;

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

fn register_call(num: usize, handle: usize) {
    unsafe { CALL_TABLE[num] = handle; }
}

fn print_hello(){
    abi_hello();
    return;
}

fn putchar(c: char){
    abi_putchar(c);
    return;
}

fn shutdown(){
    abi_shutdown();
}

fn puts( s:usize , len:usize ){
    let ptr = s as *const u8;
    let st = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len)) };
    for i in st.chars(){
        putchar(i);
    }
    println!("");
    return;
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
    return;
}

fn abi_putchar(c: char) {
    print!("{}" , c );
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

        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, code.len())
        };
        run_code.copy_from_slice(code);
        


        register_abi(SYS_HELLO, abi_hello as usize);
        register_abi(SYS_PUTCHAR, abi_putchar as usize);
        register_abi(SYS_SHUTDOWN, abi_shutdown as usize);

        register_call(SYS_HELLO, print_hello as usize);
        register_call(SYS_PUTCHAR, puts as usize);
        register_call(SYS_SHUTDOWN, shutdown as usize);
        
        println!("Execute lab4 ...");
    
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

            la      a7, {call_table}
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
            call_table = sym CALL_TABLE,
        )}
    }
    if apps_head.app2_size != 0 {
        let mut code = unsafe { core::slice::from_raw_parts(apps_start.add(apps_head.app2_offset), apps_head.app2_size) };

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
