// icon + version info for the exe
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico")
            .set("FileDescription", "snaptap - Snap Tap / SOCD for A/D and W/S")
            .set("ProductName", "snaptap")
            .set("CompanyName", "attent10nc")
            .set("LegalCopyright", "MIT License")
            .set("OriginalFilename", "snaptap.exe");
        res.compile().expect("resource compile failed");
    }
}
