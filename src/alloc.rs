use std::alloc::*;


#[unsafe(no_mangle)]
pub unsafe fn _vx_alloc(size : u32, align : u32) -> u32 {
    let layout = unsafe { Layout::from_size_align_unchecked(size as usize, align as usize) };
    let ptr = unsafe { alloc(layout) };
    if (ptr.is_null()) { handle_alloc_error(layout); }
    ptr as u32
}


#[unsafe(no_mangle)]
pub unsafe fn _vx_dealloc(ptr : u32, size : u32, align : u32) -> () {
    let layout = unsafe { Layout::from_size_align_unchecked(size as usize, align as usize) };
    unsafe { dealloc(ptr as *mut u8, layout) };
}
