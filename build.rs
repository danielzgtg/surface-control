use std::env;
use std::path::PathBuf;
use clap_complete::shells;

#[allow(dead_code)]
#[path = "src/cli/mod.rs"]
mod cli;

#[allow(dead_code)]
#[path = "src/sys/mod.rs"]
mod sys;



fn main() {
    let outdir: PathBuf = env::var_os("CARGO_TARGET_DIR")
        .or_else(|| env::var_os("OUT_DIR"))
        .unwrap()
        .into();

    let rootdir = env::current_dir().unwrap();

    let reg = cli::build();
    let mut app = reg.cli();

    clap_complete::generate_to(shells::Bash, &mut app, "surface", &outdir).unwrap();
    clap_complete::generate_to(shells::Zsh, &mut app, "surface", &outdir).unwrap();
    clap_complete::generate_to(shells::Fish, &mut app, "surface", &outdir).unwrap();

    // copy config files
    let files = [
        "systemd/surface-rapl.service",
        "systemd/surface-rapl.sh",
    ];

    for file in files {
        let src = rootdir.join(file);
        let tgt = outdir.join(file);

        std::fs::create_dir_all(tgt.parent().unwrap()).unwrap();
        std::fs::copy(src, tgt).unwrap();
    }
}
