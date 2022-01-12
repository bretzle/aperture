const DEFAULT_STATE: u64 = 0x853c_49e6_748f_ea9b;
const DEFAULT_STREAM: u64 = 0xda3e_39cb_94b9_5bdb;
const MULT: u64 = 0x5851_f42d_4c95_7f2d;

pub struct Rng {
    state: u64,
    inc: u64,
}

impl Rng {
    pub fn new() -> Self {
        Self {
            state: DEFAULT_STATE,
            inc: DEFAULT_STREAM,
        }
    }
}
