default:
	rm -rf target
	cargo rustc -- -C link-arg=--script=./linker.ld -C code-model=small -C relocation-model=static
	aarch64-none-elf-objcopy -O binary target/aarch64-unknown-none/debug/rustpi ./kernel8.img