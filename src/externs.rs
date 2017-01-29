use ::libc;

extern {
    pub fn TA_Initialize() -> i32;
    pub fn TA_Shutdown() -> i32;
    pub fn TA_SMA(start_idx: libc::c_int,
              end_idx: libc::c_int,
              in_real: *const libc::c_double,
              opt_in_time_period: libc::c_int,
              out_beg_index: *mut libc::c_int,
              out_nbe_element: *mut libc::c_int,
              out_real: *mut libc::c_double) -> i32;
}