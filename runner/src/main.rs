use anyhow::Result;
use std::env;
use std::process::Command;

fn main() -> Result<()> {
    let kernel_path = env::var("KERNEL_PATH")
        .unwrap_or(String::from("target/x86_64-unknown-none/debug/test-kernel"));
    let out_gpt_path = &AsRef::as_ref("target/uefi.img");
    bootloader::UefiBoot::new(AsRef::as_ref(&kernel_path)).create_disk_image(out_gpt_path)?;

    let out_bios_path = &AsRef::as_ref("target/bios.img");
    bootloader::BiosBoot::new(AsRef::as_ref(&kernel_path)).create_disk_image(out_bios_path)?;

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    cmd.arg("-drive")
        .arg(format!("format=raw,file={}", out_gpt_path.display()));
    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}
