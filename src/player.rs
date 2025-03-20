use crate::*;


unsafe extern "C" {
    fn _vx_player_send_chat(session_id : u64, msg_ptr : u32, msg_size : u32) -> ();
    fn _vx_player_send_sound(session_id : u64, sound_id_ptr : u32, sound_id_size : u32, cat : u32, volume : f32, pitch : f32, seed : u64) -> ();
}


pub struct Player { session_id : u64 }
impl Player {

    pub fn from_session_id(session_id : u64) -> Self {
        Self { session_id }
    }

    pub fn send_chat(&self, msg : &str) -> () {
        unsafe { _vx_player_send_chat(self.session_id, msg.as_ptr() as u32, msg.as_bytes().len() as u32); }
    }

    pub fn play_sound(&self, sound_id : &str, cat : SoundCat, volume : f32, pitch : f32, seed : u64) -> () {
        unsafe { _vx_player_send_sound(self.session_id, sound_id.as_ptr() as u32, sound_id.as_bytes().len() as u32, cat as u32, volume, pitch, seed); }
    }

}
