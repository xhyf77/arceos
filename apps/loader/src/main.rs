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

const CALL_HELLO: usize = 1;
const CALL_PUTS: usize = 2;
const CALL_SHUTDOWN: usize = 3;
const CALL_PUTCHAR: usize = 4;

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


    unsafe { init_app_page_table(); }
    unsafe { switch_app_aspace(); }

    const RUN_START_1: usize = 0x4010_0000;
    const RUN_START_2: usize = 0x4020_0000;

    if apps_head.app1_size != 0 {
        let mut code = unsafe { core::slice::from_raw_parts(apps_start.add(apps_head.app1_offset), apps_head.app1_size) };

        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START_1 as *mut u8, code.len())
        };
        run_code.copy_from_slice(code);
        


        register_abi(SYS_HELLO, abi_hello as usize);
        register_abi(SYS_PUTCHAR, abi_putchar as usize);
        register_abi(SYS_SHUTDOWN, abi_shutdown as usize);

        register_call(CALL_HELLO, print_hello as usize);
        register_call(CALL_PUTS, puts as usize);
        register_call(CALL_SHUTDOWN, shutdown as usize);
        register_call(CALL_PUTCHAR, putchar as usize);
        println!("Execute week2_lab5 ...");
    
        println!("====================APP1_START_RUN===================");
        println!("APP1_ADDRESS : 0x{:0x}       APP1_SPACE_SIZE : 0x{:0x}" , RUN_START_1 , RUN_START_2 - RUN_START_1 );
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
            run_start = const RUN_START_1,
            call_table = sym CALL_TABLE,
        )}
        println!("====================APP1_EXIT===================");
    }
    if apps_head.app2_size != 0 {
        let mut code = unsafe { core::slice::from_raw_parts(apps_start.add(apps_head.app2_offset), apps_head.app2_size) };

        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START_2 as *mut u8, code.len())
        };
        run_code.copy_from_slice(code);


        println!("====================APP2_START_RUN====================");
        println!("APP2_ADDRESS : 0x{:0x}       APP2_SPACE_SIZE : 0x{:0x}" , RUN_START_2 , RUN_START_2 - RUN_START_1 );
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
            run_start = const RUN_START_2,
            call_table = sym CALL_TABLE,
        )}
        println!("====================APP2_EXIT===================");

    }

    println!("Load payload ok!");
}

#[inline]
fn bytes_to_usize(bytes: &[u8]) -> usize {
    usize::from_be_bytes(bytes.try_into().unwrap())
}


#[link_section = ".data.app_page_table"]
static mut APP_PT_SV39: [u64; 512] = [0; 512];

unsafe fn init_app_page_table() {
    // 0x8000_0000..0xc000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[2] = (0x80000 << 10) | 0xef;
    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[0x102] = (0x80000 << 10) | 0xef;

    // 0x0000_0000..0x4000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[0] = (0x00000 << 10) | 0xef;

    // For App aspace!
    // 0x4000_0000..0x8000_0000, VRWX_GAD, 1G block
    APP_PT_SV39[1] = (0x80000 << 10) | 0xef;
}

unsafe fn switch_app_aspace() {
    use riscv::register::satp;
    let page_table_root = APP_PT_SV39.as_ptr() as usize - axconfig::PHYS_VIRT_OFFSET;
    satp::set(satp::Mode::Sv39, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}