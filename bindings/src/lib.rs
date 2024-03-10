extern "C" {
    fn log(x: f64) -> f64;
}
extern "C" {
    fn pow(x: f64, y: f64) -> f64;
}

pub mod c_calls{
    use super::*;   
    pub fn r_pow(x: f64, y: f64) -> f64{
        let pow_from_c = unsafe { pow(x, y) };
        pow_from_c
    }
    pub fn r_log() -> f64{
        let log_from_c = unsafe { log(10.0) };
        log_from_c
    }
}


