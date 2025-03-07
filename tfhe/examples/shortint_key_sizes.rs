#[path = "../benches/utilities.rs"]
mod utilities;
use crate::utilities::{write_to_json, OperatorType};

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use tfhe::shortint::keycache::{NamedParam, KEY_CACHE};
use tfhe::shortint::parameters::{
    PARAM_MESSAGE_1_CARRY_1, PARAM_MESSAGE_2_CARRY_2, PARAM_MESSAGE_3_CARRY_3,
    PARAM_MESSAGE_4_CARRY_4,
};

fn write_result(file: &mut File, name: &str, value: usize) {
    let line = format!("{name},{value}\n");
    let error_message = format!("cannot write {name} result into file");
    file.write_all(line.as_bytes()).expect(&error_message);
}

fn client_server_key_sizes(results_file: &Path) {
    let shortint_params_vec = vec![
        PARAM_MESSAGE_1_CARRY_1,
        PARAM_MESSAGE_2_CARRY_2,
        PARAM_MESSAGE_3_CARRY_3,
        PARAM_MESSAGE_4_CARRY_4,
    ];
    File::create(results_file).expect("create results file failed");
    let mut file = OpenOptions::new()
        .append(true)
        .open(results_file)
        .expect("cannot open results file");

    let operator = OperatorType::Atomic;

    println!("Generating shortint (ClientKey, ServerKey)");
    for (i, params) in shortint_params_vec.iter().enumerate() {
        println!(
            "Generating [{} / {}] : {}",
            i + 1,
            shortint_params_vec.len(),
            params.name().to_lowercase()
        );

        let keys = KEY_CACHE.get_from_param(*params);

        // Client keys don't have public access to members, but the keys in there are small anyways
        // let cks = keys.client_key();
        let sks = keys.server_key();
        let ksk_size = sks.key_switching_key_size_bytes();
        let test_name = format!("shortint_key_sizes_{}_ksk", params.name());

        write_result(&mut file, &test_name, ksk_size);
        write_to_json(&test_name, *params, params.name(), "KSK", &operator);

        println!(
            "Element in KSK: {}, size in bytes: {}",
            sks.key_switching_key_size_elements(),
            ksk_size,
        );

        let bsk_size = sks.bootstrapping_key_size_bytes();
        let test_name = format!("shortint_key_sizes_{}_bsk", params.name());

        write_result(&mut file, &test_name, bsk_size);
        write_to_json(&test_name, *params, params.name(), "BSK", &operator);

        println!(
            "Element in BSK: {}, size in bytes: {}",
            sks.bootstrapping_key_size_elements(),
            bsk_size,
        );

        // Clear keys as we go to avoid filling the RAM
        KEY_CACHE.clear_in_memory_cache()
    }
}

fn main() {
    let work_dir = std::env::current_dir().unwrap();
    println!("work_dir: {}", std::env::current_dir().unwrap().display());
    // Change workdir so that the location of the keycache matches the one for tests
    let mut new_work_dir = work_dir;
    new_work_dir.push("tfhe");
    std::env::set_current_dir(new_work_dir).unwrap();

    let results_file = Path::new("shortint_key_sizes.csv");
    client_server_key_sizes(results_file)
}
