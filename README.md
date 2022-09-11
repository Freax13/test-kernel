# test kernel

This repo implements a simple kernel that can be used for testing https://github.com/rust-osdev/bootloader.

## Building an image

Run this command to build an image and start it in qemu.

```shell
$ cargo run -p runner
```

The image will be placed in `target/uefi.img` and can be flashed onto a USB flash drive for testing with real hardware.

## Expected output

The test kernel will do the following things:
- Clear the framebuffer.
- Draw a colored square into each corner of the screen to test if the framebuffer information provided by the bootloader is correct
- Log the bootloader version
- Log the offset of the physical memory mapping created by the bootloader
- Log the memory map provided by the bootloader
- Attempt to write all usable memory to check if the memory map is correct

Last but not least, the test kernel will log "Done". If this message isn't logged one of the tests caused a crash.

![image of the expected output](imgs/Screenshot.png)

Log messages will be displayed on the framebuffer (if one exists) and written to the first serial port.