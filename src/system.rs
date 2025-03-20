use core::time::Duration;


unsafe extern "C" {
    safe fn _vx_sleep(secs : u64, subsec_nanos : u32) -> ();
}

pub fn sleep(dur : Duration) -> () {
    _vx_sleep(dur.as_secs(), dur.subsec_nanos());
}
