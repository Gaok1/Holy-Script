fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/holyScript.ico");
        res.compile().expect("Failed to compile Windows resources");
    }
}
