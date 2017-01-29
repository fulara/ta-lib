# ta-lib [![Build Status](https://img.shields.io/crates/v/ta-lib.svg)](https://crates.io/crates/ta-lib)
rust wrapper for ta-lib http://ta-lib.org/

currently exports only
TA_SMA
TA_BBANDS

If you re interested in more functions feel free to add these. :)

# windows only
Currently works only with i686 windows, if you need ta-lib for other platforms either request it, or just create a pull request!

# Issues
Currently facing with following errors:
```
error: could not execute process `D:\..\ta-lib\target\debug\deps\ta_lib-8014bfb0f5d07767.exe` (never executed)
```

And when attempting to extract the code into integration tests:  
```
The application has failed to start because its side-by-side configuration is incorrect. Please see the application event log or use the command-line sxstrace.exe tool for more detail.
```

Not really sure how to resolve that.

# Usage 
Example usage would looks as follows

```
extern crate ta_lib;

fn main() {
    ta_lib::initialize();
    
    let data_in = [0., 1., 2., 3.];
    let mut data_out = [0., 0., 0., 0.];
    
    ta_lib::ta_sma(0, data_in.len(), &data_in, 2, &mut data_out));
    
    ta_lib::shutdown();
}
```

