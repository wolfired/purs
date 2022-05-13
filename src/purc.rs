extern "C" {
    pub fn purc_max(x: u32, y: u32) -> u32;
    pub fn purc_call_rs(cb: Option<fn()>);
}
