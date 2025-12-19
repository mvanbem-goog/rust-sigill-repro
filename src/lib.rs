#[test]
#[should_panic]
fn panic_locally_and_everything_is_ok() {
    panic!("panic locally");
}

#[test]
#[should_panic]
fn panic_in_core_and_we_abort_instead() {
    let mut buf = [0u8; 1];
    buf.copy_from_slice(&[1, 2]);
}
