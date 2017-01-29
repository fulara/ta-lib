fn main() {
    println!("cargo:rustc-link-lib=static=ta_common_cdd");
    println!("cargo:rustc-link-lib=static=ta_abstract_cdd");
    println!("cargo:rustc-link-lib=static=ta_func_cdd");
    println!("cargo:rustc-link-lib=static=ta_libc_cdd");
    //println!("cargo:rustc-link-search=static=D:\\Progsy\\IdeaProjects\\ta-lib");
    println!("cargo:rustc-link-search=ta-lib\\lib");
}