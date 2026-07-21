fn main() {
  println!(
    "cargo:rustc-check-cfg=cfg(feature, values(\
       \"esp32\",\"esp32s2\",\"esp32s3\",\
       \"esp32c5\",\"esp32p4\"\
     ))"
  );
}
