use std::process::Command;

use anyhow::Result;

fn main() -> Result<()> {
    let boot_partition_path = AsRef::as_ref("target/boot.fat");
    bootloader::create_boot_partition(
        AsRef::as_ref(env!("CARGO_BIN_FILE_TEST_KERNEL_test-kernel")),
        boot_partition_path,
    )?;
    let out_gpt_path = &AsRef::as_ref("target/uefi.img");
    bootloader::create_uefi_disk_image(boot_partition_path, out_gpt_path)?;

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    cmd.arg("-enable-kvm");
    cmd.arg("-drive")
        .arg(format!("format=raw,file={}", out_gpt_path.display()));
    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}
