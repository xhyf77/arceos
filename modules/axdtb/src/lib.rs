#![no_std]
extern crate alloc;
use core::iter::IntoIterator;
use core::result;
use alloc::string::String;
pub use dtb_parser::DeviceTree;
use alloc::vec::Vec;
use core::slice;
use alloc::vec;
use dtb_parser::prop::PropertyValue;
#[macro_use]
extern crate axlog;
// 参考类型定义
pub struct DtbInfo {
    pub memory_addr: usize,
    pub memory_size: usize,
    pub mmio_regions: Vec<(usize, usize)>,
}

impl DtbInfo {
    pub fn new() -> DtbInfo{
        DtbInfo{
            memory_addr : 0 ,
            memory_size : 0 ,
            mmio_regions : vec![]
        }
    }
}

#[repr(C)]
#[derive(Debug)]
struct DtbHeader {
    magic: u32,
    total_size: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
}

#[derive(Debug)]
pub enum DtbParseError {
    InvalidMagicNumber,
    InvalidStructure,
    Other(String), 
}

// 参考函数原型
pub fn parse_dtb(dtb_pa: usize) -> Result<DtbInfo,DtbParseError> {
    // 这里就是对axdtb组件的调用，传入dtb指针，解析后输出结果。这个函数和axdtb留给大家实现
    let mut x = 0;
    let mut dtb = DtbInfo::new();
    unsafe{
        let bytes: &[u8] = slice::from_raw_parts(dtb_pa as *const u8, 28);
        let header: &DtbHeader = &*(bytes.as_ptr() as *const DtbHeader);
        info!("{:0x}" , header.magic);
        if header.magic == 0xedfe0dd0 {
            // 计算有效数据部分
            info!("{:?}" , header );
            let valid_data: &[u8] = slice::from_raw_parts(dtb_pa as *const u8, header.total_size as usize);
            let tree = DeviceTree::from_bytes(valid_data).unwrap();
            for device in tree.into_iter(){
                info!("{}" , device.type_name() );
                if device.type_name() == "memory" {
                    for i in device.props(){
                        if i.name() == "reg" {
                            match i.value(){
                                PropertyValue::Address( a , b) => {
                                    dtb.memory_addr = *a as usize;
                                    dtb.memory_size = *b as usize;
                                }

                                _ => {
                                    ()
                                }
                            }
                        }
                    }
                }
                if device.type_name() == "virtio_mmio" {
                    for i in device.props(){
                        if i.name() == "reg" {
                            match i.value(){
                                PropertyValue::Address( a , b) => {
                                    dtb.mmio_regions.push( ( *a as usize , *b as usize ) );
                                }
                                _ => {
                                    ()
                                }
                            }
                        }
                    }
                }
            }
            Result::Ok(dtb)
        }
        else {
            Result::Err(DtbParseError::InvalidMagicNumber)       
        }
    }
}