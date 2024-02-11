// (c) Copyright 2019-2023 OLX
fn main() {
    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-lib=vips");
        println!("cargo:rustc-link-lib=glib-2.0");
        println!("cargo:rustc-link-lib=gobject-2.0");
    }
    #[cfg(target_os = "windows")]
    {
        use std::{env, path::Path};

        println!("cargo:rustc-link-lib=libvips");
        println!("cargo:rustc-link-lib=libglib-2.0");
        println!("cargo:rustc-link-lib=libgobject-2.0");
        let dir = env::var("LIBVIPS_LIB_DIR")
            .unwrap_or_else(|_| panic!("Must specify LIBVIPS_LIB_DIR env variable for windows"));
        println!(
            "cargo:rustc-link-search=native={}",
            Path::new(&dir).display()
        );
    }
}
