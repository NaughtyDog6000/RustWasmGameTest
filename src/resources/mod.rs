use bracket_terminal::EMBED;




// pub fn load_resources() {
//     bracket_terminal::link_resource!(RAW_FILE, "../../resources/fake_saves/save1.json");

//     let data = bracket_terminal::EMBED
//     .lock()
//     .get_resource("../../resources/Entities.json".to_string())
//     .unwrap();

//     let json_string = std::str::from_utf8(&data).expect("Unable to convert to string.");

// }



// -- NATIVE PLATFORM --

#[cfg(not(target_arch = "wasm32"))]
pub fn save_game_file_json_buf(json_buffer: Vec<u8>) {
    use std::{fs::File, io::Write};

    let mut file = File::create("./savegame.json").unwrap();
    file.write(&json_buffer).unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_game_file_vec_u8() -> Result<Vec<u8>, ()> {
    use std::fs;
    if !does_save_exist() {return Err(());}
    Ok(fs::read("./savegame.json").unwrap())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn does_save_exist() -> bool {
    use std::path::Path;

    Path::new("./savegame.json").exists()
}

// -- WEB PLATFORM --
#[cfg(target_arch = "wasm32")]
pub fn save_game_file_json_buf(json_buffer: Vec<u8>) {
    use web_sys::window;
    use web_sys::Storage;

    let json_string = std::str::from_utf8(&json_buffer).expect("Unable to convert to string.");

    let window = window().expect("should have a window");
    let local_storage = window.local_storage().expect("no local storage?");

    local_storage.unwrap().set_item("save", json_string).expect("failed to set item");
}

#[cfg(target_arch = "wasm32")]
pub fn load_game_file_vec_u8() -> Result<Vec<u8>, ()> {
    use web_sys::window;
    use web_sys::Storage;

    let window = window().expect("should have a window");
    let local_storage = window.local_storage().expect("no local storage?");

    if local_storage.is_none() { return Err(()); }

    let result = local_storage.unwrap().get_item("save");
    if result.is_err() { return Err(()); }
    return Ok(result.unwrap().unwrap().into());
}

#[cfg(target_arch = "wasm32")]
pub fn does_save_exist() -> bool {
    use web_sys::window;

    let window = window().expect("should have a window");
    let local_storage = window.local_storage().expect("no local storage?");

    if local_storage.is_none() { return false }

    let result = local_storage.unwrap().get_item("save");
    if result.is_err() { return false }
    return result.unwrap().is_some();
}