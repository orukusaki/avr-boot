#![feature(exit_status_error)]

use avr_tester::{AvrTester, AvrTesterBuilder};
use std::{path::Path, process::Command};

pub fn avr(test: &str, target: &str, hal: &str) -> AvrTester {
    eprintln!("Building firmware");

    let module_dir = Path::new("..").join("avr-boot-examples");

    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg(format!("--target=../.cargo/targets/{}.json", target))
        .arg(format!("--bin={}", test))
        .arg(format!("--features={}", hal))
        .current_dir(&module_dir)
        .status()
        .expect("Couldn't build firmware")
        .exit_ok()
        .expect("Couldn't build firmware");

    let firmware = module_dir
        .join("target")
        .join(target)
        .join("release")
        .join(format!("{}.elf", test));

    eprintln!("Starting test");

    AvrTesterBuilder::new(target)
        .with_clock_of_16_mhz()
        .load(firmware)
}

#[cfg(test)]
mod atmega;

#[cfg(test)]
mod tiny;
