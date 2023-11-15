#[cfg(target_os = "linux")]
use reqwest::blocking::Client;

#[cfg(target_os = "linux")]
use std::{env, fs, io::Cursor, path::PathBuf, time::Duration};

#[cfg(target_os = "linux")]
const LIB_URL: &str = "https://github.com/metrico/libchdb/releases/latest/download/libchdb.zip";
#[cfg(target_os = "linux")]
const USER_AGENT: &str = "Mozilla/5.0 (Linux x86_64; rv:115.0) Gecko/20100101 Firefox/115.0";

#[cfg(target_os = "linux")]
fn main() {
    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib_dir = outdir.join("lib");

    if !lib_dir.exists() {
        let client = Client::builder().user_agent(USER_AGENT).build().unwrap();
        let result = client
            .get(LIB_URL)
            .timeout(Duration::from_secs(600))
            .send()
            .expect("Unable to request libchdb");

        if result.status() != reqwest::StatusCode::OK {
            panic!("Unable to download libchdb");
        }

        let content = result.bytes().expect("Unable to read from response");
        let cursor = Cursor::new(content);

        let mut archive = zip::ZipArchive::new(cursor).expect("Unable to read zip archive");
        let extract_res = archive.extract(&outdir.join("lib"));

        if extract_res.is_err() {
            fs::remove_dir_all(&lib_dir).unwrap();
            eprintln!("Unable to extract libchdb");
            std::process::exit(1);
        }
    }

    println!("cargo:rustc-link-search={}", lib_dir.display());
    println!("cargo:rustc-link-lib=chdb");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("libchdb.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(outdir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(target_os = "linux"))]
fn main() {
    panic!("Supported for linux only")
}
