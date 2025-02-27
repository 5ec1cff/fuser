use std::env;

fn main() {
    #[cfg(not(feature = "libfuse"))]
    {
        let out_dir = env::var("CARGO_CFG_TARGET_OS").unwrap();
        if out_dir.as_str() != "android" && out_dir.as_str() != "linux" {
            unimplemented!("Building without libfuse is only supported on Linux, target: {}", out_dir.as_str());
        }
    }

    #[cfg(feature = "libfuse")]
    {
        #[cfg(target_os = "macos")]
        {
            if pkg_config::Config::new()
                .atleast_version("2.6.0")
                .probe("fuse") // for macFUSE 4.x
                .map_err(|e| eprintln!("{}", e))
                .is_ok()
            {
                println!("cargo:rustc-cfg=feature=\"libfuse2\"");
            } else {
                pkg_config::Config::new()
                    .atleast_version("2.6.0")
                    .probe("osxfuse") // for osxfuse 3.x
                    .map_err(|e| eprintln!("{}", e))
                    .unwrap();
                println!("cargo:rustc-cfg=feature=\"libfuse2\"");
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            // First try to link with libfuse3
            if pkg_config::Config::new()
                .atleast_version("3.0.0")
                .probe("fuse3")
                .map_err(|e| eprintln!("{e}"))
                .is_ok()
            {
                println!("cargo:rustc-cfg=feature=\"libfuse3\"");
            } else {
                // Fallback to libfuse
                pkg_config::Config::new()
                    .atleast_version("2.6.0")
                    .probe("fuse")
                    .map_err(|e| eprintln!("{e}"))
                    .unwrap();
                println!("cargo:rustc-cfg=feature=\"libfuse2\"");
            }
        }
    }
}
