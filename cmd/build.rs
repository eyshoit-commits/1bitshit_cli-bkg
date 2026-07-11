fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set("InternalName", "bitshit.exe");
        res.set("FileDescription", "bitshit");
        res.set("ProductName", "bitshit");
        res.set("OriginalFilename", "bitshit.exe");
        res.set("LegalCopyright", "Copyright © 2026 bitshit Technologies");
        res.set("CompanyName", "bitshit Technologies");
        // Alpha Version Alignment Lock
        res.set("FileVersion", "0.0.1.0");
        res.set("ProductVersion", "0.0.1.0");

        res.set_manifest(
            r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="asInvoker" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#,
        );

        // 🚀 Set the bitshit Taskbar & Executable Icon
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let icon_path = std::path::Path::new(&manifest_dir).parent().unwrap().join("assets").join("logo.ico");
        res.set_icon(icon_path.to_str().unwrap());

        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {}", e);
        }
    }
}
