#[cfg(target_arch = "wasm32")]
use sapp_jsutils::{JsObject, JsObjectWeak};

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

/// Open file dialog to save the bytes to a file.
///
/// `filename` is requested file name
///
/// `bytes` is file data
///
/// if `filter` is Some, only show files of the same type in the file picker. The &str contained will be the name of the filter
pub fn download(filename: &str, bytes: &[u8], filter: Option<&str>) -> Result<(), std::io::Error> {
    #[cfg(target_arch = "wasm32")]
    {
        unsafe {
            let object = JsObject::buffer(bytes);
            quad_file_download(JsObject::string(filename).weak(), object.weak());
            Ok(())
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let extension = filename.split(".").last();
        let mut dialog = rfd::FileDialog::new().set_file_name(filename);
        if let Some(extension) = extension {
            if let Some(filter) = filter {
                dialog = dialog.add_filter(filter, &vec![extension]);
            }
        }
        let path = dialog.save_file();
        if let Some(path) = path {
            std::fs::write(path, bytes)
        } else {
            Err(std::io::Error::other("File dialog was cancelled"))
        }
    }
}
