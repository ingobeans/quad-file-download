#[cfg(target_arch = "wasm32")]
use sapp_jsutils::{JsObject,JsObjectWeak};

#[no_mangle]
extern "C" fn quad_file_download_crate_version() -> u32 {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();

    (major << 24) + (minor << 16) + patch
}

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn quad_file_download(path: JsObjectWeak, bytes: JsObjectWeak);
}

pub fn download(path: &str, bytes: &[u8])->Result<(),std::io::Error> {
	#[cfg(target_arch = "wasm32")]
	{
		unsafe {
			let object = JsObject::buffer(bytes);
			quad_file_download(JsObject::string(path).weak(),object.weak());
			Ok(())
		}
	}
	#[cfg(not(target_arch = "wasm32"))]
	{
		let dialog = rfd::FileDialog::new().set_file_name(path);
		let path = dialog.save_file();
		if let Some(path) = path {
			std::fs::write(path, bytes)
		} else {
			Err(std::io::Error::other("File dialog was cancelled"))
		}
	}
}
