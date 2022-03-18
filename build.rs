extern crate napi_build;

fn main() {
	if cfg!(target_env = "musl") {
		let _ = std::process::Command::new("sh")
			.arg("-c")
			.arg("cp /usr/lib/libsodium.so.23 target/release/deps")
			.output()
			.unwrap();
	}
	
    napi_build::setup();
}
