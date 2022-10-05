use std::process::Command;

use anyhow::Result;

fn main() -> Result<()> {
    let out_gpt_path = &AsRef::as_ref("target/uefi.img");
    bootloader::UefiBoot::new(AsRef::as_ref(env!(
        "CARGO_BIN_FILE_TEST_KERNEL_test-kernel"
    )))
    .create_disk_image(out_gpt_path)?;

    let out_bios_path = &AsRef::as_ref("target/bios.img");
    bootloader::BiosBoot::new(AsRef::as_ref(env!(
        "CARGO_BIN_FILE_TEST_KERNEL_test-kernel"
    )))
    .create_disk_image(out_bios_path)?;

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    cmd.arg("-drive")
        .arg(format!("format=raw,file={}", out_gpt_path.display()));
    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}
