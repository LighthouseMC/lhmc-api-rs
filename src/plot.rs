unsafe extern "C" {
    safe fn _vx_plot_id() -> u64;
    fn _vx_plot_name_fmt(out_ptr : u32) -> ();
}


pub struct Plot(());
impl Plot {

    pub fn id() -> u64 { _vx_plot_id() }

    pub fn name_fmt() -> Result<String, ()> {
        let mut ptr_size = (0u32, 0u32,);
        unsafe { _vx_plot_name_fmt(&mut ptr_size as *mut _ as u32) };
        let (ptr, size,) = ptr_size;
        let name = unsafe { String::from_raw_parts(ptr as *mut u8, size as usize, size as usize) };
        if (name.is_empty()) { Err(()) } else { Ok(name) }
    }

}
