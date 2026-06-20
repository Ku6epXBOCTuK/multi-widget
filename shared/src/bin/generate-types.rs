use std::process::Command;

use shared::{Activity, WsEvent};
use specta::Types;
use specta_typescript::Typescript;

fn main() {
    let types = Types::default()
        .register::<Activity>()
        .register::<WsEvent>();

    let output_path = "./frontend/src/lib/generated-bindings.ts";

    Typescript::default()
        .export_to(output_path, &types, specta_serde::Format)
        .expect("Failed to export typescript bindings");

    println!("Export complete to: {}", output_path);

    let frontend_dir = ".\\frontend";

    let command_status = Command::new("cmd")
        .current_dir(frontend_dir)
        .arg("/C")
        .arg("npm")
        .arg("run")
        .arg("format")
        .status();

    match command_status {
        Ok(_) => println!("Format complete"),
        Err(err) => println!("Failed to format with prettier: {:?}", err),
    }
}
