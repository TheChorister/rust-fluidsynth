extern crate libc;
use ffi::*;


#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum VoiceAddMod {
    Overwrite = 0,
    Add, 
    Default,
}



// TODO impl related types (e.g. generators, modulators etc.)

pub struct Voice {
    c_fluid_voice: *mut fluid_voice_t
}

impl Voice {
	pub fn get_actual_key(&self) -> i32 {
		unsafe {
			fluid_voice_get_actual_key(self.to_raw())
		}
	}

	pub fn get_actual_velocity(&self) -> i32 {
		unsafe {
			fluid_voice_get_actual_velocity(self.to_raw())
		}
	}
	
	pub fn get_channel(&self) -> i32 {
		unsafe {
			fluid_voice_get_channel(self.to_raw())
		}
	}

	pub fn get_id(&self) -> u32 {
		unsafe {
			fluid_voice_get_id(self.to_raw())
		}
	}

	pub fn get_key(&self) -> i32 {
		unsafe {
			fluid_voice_get_key(self.to_raw())
		}
	}

	pub fn get_velocity(&self) -> i32 {
		unsafe {
			fluid_voice_get_velocity(self.to_raw())
		}
	}

	pub fn is_on(&self) -> bool {
		unsafe {
			fluid_voice_is_on(self.to_raw()) == 1
		}
	}

	pub fn is_playing(&self) -> bool {
		unsafe {
			fluid_voice_is_playing(self.to_raw()) == 1
		}
	}

	pub fn is_sostenuto(&self) -> bool {
		unsafe {
			fluid_voice_is_sostenuto(self.to_raw()) == 1
		}
	}

	pub fn is_sustained(&self) -> bool {
		unsafe {
			fluid_voice_is_sustained(self.to_raw()) == 1
		}
	}

    pub fn from_raw(voice: *mut fluid_voice_t) -> Voice {
        Voice {
            c_fluid_voice: voice
        }
    }

    pub fn to_raw(&self) -> *mut fluid_voice_t {
    	self.c_fluid_voice
    }
}

