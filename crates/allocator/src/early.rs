use core::{alloc::Layout, ptr::NonNull};

use crate::{AllocError, AllocResult};


pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    base_pos:usize,
    total_size:usize ,
    used_bytes:usize ,
    byte_pos:usize,
    used_pages:usize ,
    page_pos:usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE>{

    pub const fn new() -> Self{
        Self{
            base_pos:0,
            total_size:0,
            used_bytes:0,
            byte_pos:0,
            used_pages:0,
            page_pos:0,
        }
    }

    pub fn init(&mut self, start: usize, size: usize) {
        self.base_pos = start;
        self.total_size = size;
        self.byte_pos = self.base_pos;
        self.page_pos = self.base_pos + self.total_size - 1;
    }

    pub fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>>{
        let align = layout.align();
        let size = layout.size();
        let new_offset = (self.byte_pos + align - 1) & !(align - 1);
        if new_offset + size - 1 > self.page_pos {
            return AllocResult::Err(AllocError::NoMemory);
        }
        else {
            self.used_bytes = self.used_bytes + size;
            self.byte_pos = new_offset + size;
        }
        let ptr = NonNull::new(new_offset as *mut u8 ).unwrap();
        AllocResult::Ok(ptr)
    }

    pub fn dealloc(&mut self, _pos: NonNull<u8>, layout: Layout) {
        let size = layout.size();
        self.used_bytes -= size;
    }
    
    pub fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let left = align_pow2.trailing_zeros();
        let align_pow2 = align_pow2 / PAGE_SIZE;
        if !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }
        
        let mut start = self.page_pos - num_pages * PAGE_SIZE + 1 ;
        start = ( start >> left ) << left;
        if start < self.byte_pos {
            AllocResult::Err(AllocError::NoMemory)
        }
        else {
            self.used_pages += num_pages;
            self.page_pos = start - 1;
            Ok(start)
        }
    }

    pub fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        return;
    }

    pub fn used_bytes(&self) -> usize {
        self.used_bytes
    }

    pub fn available_bytes(&self) -> usize {
        self.page_pos - self.byte_pos + 1
    }

    pub fn used_pages(&self) -> usize {
        self.used_pages
    }

    pub fn available_pages(&self) -> usize {
        let mut start = self.byte_pos;
        start = (start + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        if start > self.page_pos {
            0
        }
        else {
            ( self.page_pos + 1 - start ) / PAGE_SIZE
        }
    }
}