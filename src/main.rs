fn main() {
    println!("Panicking locally");
    std::panic::catch_unwind(|| {
	panic!("panic locally");
    }).unwrap_err();

    println!("Panicking in core");
    std::panic::catch_unwind(|| {
	let mut buf = [0u8; 1];
	buf.copy_from_slice(&[1, 2]);
    }).unwrap_err();
}
