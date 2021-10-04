use std::process::Command;

fn main() {
    
    // WASM builder
    let wasm_pack = Command::new("wasm-pack")
        .current_dir("./wasm")
        .args(["build", "--target", "web"])
        .output()
        .expect("Failed to pack wasm");

    println!("Output from building wasm: ");
    println!("{}", String::from_utf8(wasm_pack.stderr).expect("Failed converting utf8 stderr"));

    // Server builder
    let server_builder = Command::new("cargo")
        .current_dir("./server")
        .args(["build", "--release"])
        .output()
        .expect("Failed to build server");

    println!("Output from building server: ");
    println!("{}", String::from_utf8(server_builder.stderr).expect("Failed converting utf8 stderr"));

    // Move server to root
    let move_server = Command::new("mv")
        .args(["server/target/release/torquelearn", "."])
        .output()
        .expect("Failed moving server");

    println!("Output from moving server: ");
    println!("{}", String::from_utf8(move_server.stderr).expect("Failed converting utf8 stderr"));

    // Delete old WASM
    let delete_wasm = Command::new("rm")
        .args(["-rf", "pkg"])
        .output()
        .expect("Failed deleting WASM");

    println!("Output from deleting WASM: ");
    println!("{}", String::from_utf8(delete_wasm.stderr).expect("Failed converting utf8 stderr"));

    // Move WASM to root
    let move_wasm = Command::new("mv")
        .args(["wasm/pkg", "."])
        .output()
        .expect("Failed moving WASM");

    println!("Output from moving WASM: ");
    println!("{}", String::from_utf8(move_wasm.stderr).expect("Failed converting utf8 stderr"));
}
