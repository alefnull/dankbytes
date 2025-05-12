fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res
      .set_icon("./assets/leaf.ico")
      .set("ProductName", "Dank Bytes");

    if let Err(e) = res.compile() {
      panic!("Failed to compile resources: {}", e);
    }
  }
}
