use super::JavaThread;

#[derive(Copy, Clone, Debug)]
pub struct HashState {
	state_x: u32,
	state_y: u32,
	state_z: u32,
	state_w: u32,
}

impl HashState {
	const INIT_STATE_Y: u32 = 842502087;
	const INIT_STATE_Z: u32 = 0x8767; // (int)(3579807591LL & 0xffff);
	const INIT_STATE_W: u32 = 273326509;

	pub(super) fn new(seed: u32) -> Self {
		HashState {
			state_x: seed,
			state_y: Self::INIT_STATE_Y,
			state_z: Self::INIT_STATE_Z,
			state_w: Self::INIT_STATE_W,
		}
	}
}

impl JavaThread {
	pub fn marsaglia_xor_shift_hash(&self) -> u32 {
		let hash_state = self.hash_state.get();
		let mut t = hash_state.state_x;
		t ^= (t << 11);

		let mut v = hash_state.state_w;
		v = (v ^ (v >> 19)) ^ (t ^ (t >> 8));

		let new_state = HashState {
			state_x: hash_state.state_y,
			state_y: hash_state.state_z,
			state_z: hash_state.state_w,
			state_w: v,
		};
		self.hash_state.set(new_state);

		v
	}
}
