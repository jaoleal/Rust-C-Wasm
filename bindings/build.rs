
fn main() {
  cc::Build::new()
    .file("C/bindtomath.h")
    .compile("bindtomath");
}