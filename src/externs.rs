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

    pub fn TA_BBANDS(start_idx: libc::c_int,
                     end_idx: libc::c_int,
                     in_real: *const libc::c_double,
                     opt_in_time_period: libc::c_int,
                     opt_in_nb_dev_up: libc::c_double,
                     opt_in_nb_dev_dn: libc::c_double,
                     opt_in_ta_matype: libc::c_int,
                     out_beg_index: *mut libc::c_int,
                     out_nbe_element: *mut libc::c_int,
                     out_real_upper_band: *mut libc::c_double,
                     out_real_middle_band: *mut libc::c_double,
                     out_real_lower_band: *mut libc::c_double) -> i32;
}