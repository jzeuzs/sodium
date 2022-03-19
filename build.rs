extern crate napi_build;

fn main() {
        // doing this manually because of issues in musl
	if cfg!(target_env = "musl") {
		let _ = std::process::Command::new("sh")
			.arg("-c")
			.arg("cp /usr/lib/libsodium.a target/release/deps/libsodium.a")
			.output()
			.unwrap();
	}

    napi_build::setup();
}
