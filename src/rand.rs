const RAND_MAX: usize = 32767;

// To ensure that random seeds are controllable, 
// this function is extremely stupid.
pub fn rand(seed: &mut usize) -> f64 {
    let mut _seed = *seed as u32;
    _seed = (((_seed as u64 * 1103515245) as u32) as u64 + 12345) as u32;
    *seed = _seed as usize;
    ((_seed as usize) >> 16 & RAND_MAX) as f64 / (RAND_MAX + 1) as f64
}
