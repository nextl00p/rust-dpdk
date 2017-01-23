use std::path::Path;
use std::env;

fn main() {

	let link_list = vec![
	    "rte_eal",
	];

	if let Ok(dpdk_path) = env::var("RTE_SDK") {
		let path = Path::new(&dpdk_path);
		let path = path.join("lib");
		if let Some(path_string) = path.to_str() {
			println!("cargo:rustc-link-search=native={}", path_string);

			for link in &link_list {
				println!("cargo:rustc-link-lib=static={}", link);
			}
		}

	} else {
		println!("error: DPDK is not found");
		std::process::exit(-1);
	}
}