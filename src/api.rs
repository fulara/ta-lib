use externs::*;

pub fn ta_initialize() -> i32 {
    unsafe { TA_Initialize() }
}

pub fn ta_shutdown() -> i32 {
    unsafe { TA_Shutdown() }
}

/// simple moving average.
pub fn ta_sma(start_idx: usize, end_idx: usize, in_data: &[f64], period: usize, out_real: &mut [f64]) -> i32 {
    unsafe {
        let mut out_beg_index = 0;
        let mut out_nbe_index = 0;
        TA_SMA(start_idx as i32, end_idx as i32, &in_data[0], period as i32, &mut out_beg_index, &mut out_nbe_index, &mut out_real[0])
    }
}

pub enum TaMaType {
    SMA,
    EMA,
    WMA,
    DEMA,
    TEMA,
    TRIMA,
    KAMA,
    MAMA,
    T3,
}

impl TaMaType {
    fn as_i32(&self) -> i32 {
        match *self {
            TaMaType::SMA => 0,
            TaMaType::EMA => 1,
            TaMaType::WMA => 2,
            TaMaType::DEMA => 3,
            TaMaType::TEMA => 4,
            TaMaType::TRIMA => 5,
            TaMaType::KAMA => 6,
            TaMaType::MAMA => 7,
            TaMaType::T3 => 8
        }
    }
}

pub fn ta_bbands(start_idx: usize,
                 end_idx: usize,
                 in_data: &[f64],
                 period: usize,
                 opt_in_nb_dev_up: f64,
                 opt_in_nb_dev_down: f64,
                 ma_type: TaMaType,
                 out_data_upper_band: &mut [f64],
                 out_data_middle_band: &mut [f64],
                 out_data_lower_band: &mut [f64]) -> i32 {
    let mut out_beg_index = 0;
    let mut out_nbe_index = 0;

    unsafe {
        TA_BBANDS(start_idx as i32,
                  end_idx as i32,
                  &in_data[0],
                  period as i32,
                  opt_in_nb_dev_up,
                  opt_in_nb_dev_down,
                  ma_type.as_i32(),
                  &mut out_beg_index,
                  &mut out_nbe_index,
                  &mut out_data_upper_band[0],
                  &mut out_data_middle_band[0],
                  &mut out_data_lower_band[0])
    }
}

#[cfg(test)]
mod tests {
    use std::f64;

    macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !($x - $y < $d || $y - $x < $d) { panic!(); }
    }
}

    #[test]
    fn sma_test() {
        assert_eq! (0, super::ta_initialize());

        let data_in = [0., 1., 2., 3.];
        let mut data_out = [0., 0., 0., 0.];

        assert_eq! (0, super::ta_sma(0, data_in.len(), &data_in, 2, &mut data_out));

        assert_eq! (0.5, data_out[0]);
        assert_eq!(1.5, data_out[1]);
        assert_eq! (2.5, data_out[2]);
        assert_eq! (1.5, data_out[3]);

        assert_eq! (0, super::ta_shutdown());
    }

    #[test]
    fn bbands_test() {
        assert_eq! (0, super::ta_initialize());

        let data_in = [0., 1., 2.];
        let mut data_out_lower = [0., 0., 0.];
        let mut data_out_middle = [0., 0., 0.];
        let mut data_out_upper = [0., 0., 0.];

        assert_eq! (0, super::ta_bbands(0, data_in.len(), &data_in, 3, 1.0, 1.0, super::TaMaType::SMA, &mut data_out_upper, &mut data_out_middle, &mut data_out_lower));

        assert_eq! (1., data_out_middle[0]);
        assert_delta!(0.183503, data_out_lower[0], 0.0001);
        assert_delta!(1.816496, data_out_upper[0], 0.0001);

        assert_eq! (0, super::ta_shutdown());
    }
}