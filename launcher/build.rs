//! Embeds Horizon's icon (`packaging/icons/horizon.ico`) in the Windows executables, as icon
//! resource 1: Explorer and the task bar show it, and the window takes it as its icon.
fn main() {
    println!("cargo:rerun-if-env-changed=HORIZON_PORTABLE_PAYLOAD");
    if std::env::var_os("CARGO_FEATURE_PORTABLE").is_some() {
        let payload = std::env::var("HORIZON_PORTABLE_PAYLOAD").expect(
            "portable builds require HORIZON_PORTABLE_PAYLOAD; run tools/build-portable.ps1",
        );
        let path = std::path::Path::new(&payload)
            .canonicalize()
            .expect("portable payload missing");
        println!("cargo:rerun-if-changed={}", path.display());
        println!(
            "cargo:rustc-env=HORIZON_PORTABLE_PAYLOAD={}",
            path.display()
        );
    }
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let ico = format!("{dir}/../packaging/icons/horizon.ico");
    println!("cargo:rerun-if-changed={ico}");
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon(&ico);
        if let Err(e) = resource.compile() {
            panic!("embedding {ico}: {e}");
        }
    }
}
