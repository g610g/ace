fn main(){
    #[cfg(target_os="linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/x86_64-linux-gnu");
}