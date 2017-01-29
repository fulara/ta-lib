use externs::*;

pub fn ta_initialize() -> i32 {
    unsafe { TA_Initialize() }
}

pub fn ta_shutdown() -> i32 {
    unsafe { TA_Shutdown() }
}

/// simple moving average.
pub fn sma(start_idx: usize, end_idx: usize, in_data: &[f64], period: usize, out_real: &mut [f64]) -> i32 {
    unsafe {
        //let in_data_ptr = &mut in_data[0] as *mut f64;
        let mut out_beg_index = 0;
        let mut out_nbe_index = 0;
        TA_SMA(start_idx as i32, end_idx as i32, &in_data[0], period as i32, &mut out_beg_index, &mut out_nbe_index, &mut out_real[0])
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn sma_test() {
        assert_eq!(0, super::ta_initialize());

        let data_in = [0., 1., 2., 3.];
        let mut data_out = [0., 0., 0., 0.];

        assert_eq!(0, super::sma(0, data_in.len(), &data_in, 2, &mut data_out));

        assert_eq!(0.5, data_out[0]);
        assert_eq!(1.5, data_out[1]);
        assert_eq!(2.5, data_out[2]);
        assert_eq!(1.5, data_out[3]);

        assert_eq!(0, super::ta_shutdown());
    }
}