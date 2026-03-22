#[test]
fn zero_write_is_error() {
    let mut buf = [0u8];
    let writer =
        ai_flate2::write::DeflateEncoder::new(&mut buf[..], ai_flate2::Compression::default());
    assert!(writer.finish().is_err());
}
