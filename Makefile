# Makefile for Redme-9A OS kernel

# Target architecture
TARGET = aarch64-unknown-none
# Kernel binary name (matches Cargo.toml)
KERNEL = redme-9a-os
# Build profile (debug/release)
PROFILE ?= debug

# Output directory
CARGO_OUTPUT = target/$(TARGET)/$(PROFILE)
# Final ELF kernel
KERNEL_ELF = $(CARGO_OUTPUT)/$(KERNEL)
# Extracted binary (if needed)
KERNEL_BIN = $(CARGO_OUTPUT)/$(KERNEL).bin

# QEMU executable
QEMU = qemu-system-aarch64
QEMU_MACHINE = virt
QEMU_CPU = cortex-a53
QEMU_ARGS = -machine $(QEMU_MACHINE) -cpu $(QEMU_CPU) -nographic -serial mon:stdio

.PHONY: all build run debug release clean

all: build

build:
	cargo build --target=$(TARGET)

release:
	cargo build --target=$(TARGET) --release

# Run the kernel in QEMU (debug)
run: build
	$(QEMU) $(QEMU_ARGS) -kernel $(KERNEL_ELF)

# Run release kernel
run-release: release
	$(QEMU) $(QEMU_ARGS) -kernel $(KERNEL_ELF)

# Debug with GDB (optional)
debug: build
	$(QEMU) $(QEMU_ARGS) -kernel $(KERNEL_ELF) -S -s

# Convert ELF to raw binary (optional)
$(KERNEL_BIN): $(KERNEL_ELF)
	rust-objcopy -O binary $< $@

# Clean build artifacts
clean:
	cargo clean

# Show kernel size
size:
	rust-size $(KERNEL_ELF)
	rust-objdump -h $(KERNEL_ELF)