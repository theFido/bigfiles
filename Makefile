# commands
CMD_GET_RT = rustup target add
CMD_RELEASE = cargo build --release --target=

# rust targets
OSX_RT = x86_64-apple-darwin
WIN_RT = x86_64-pc-windows-msvc
LIN_RT = x86_64-unknown-linux-gnu
MSL_RT = x86_64-unknown-linux-musl

# targets
run-win:
	cls
	cargo run
run:
	clear
	cargo run
clean:
	cargo clean
list:
	rustup target list
toolchain:
	rustup update
	rustup component add clippy
	$(CMD_GET_RT) $(OSX_RT) 
	$(CMD_GET_RT) $(WIN_RT)
	$(CMD_GET_RT) $(LIN_RT)
	$(CMD_GET_RT) $(MSL_RT)
lint:
	# cargo fix -Z unstable-options --clippy
	# cargo clippy
release: lint
	cargo build --release
release-all: lint
	$(CMD_RELEASE)$(OSX_RT)
	$(CMD_RELEASE)$(WIN_RT)
	$(CMD_RELEASE)$(LIN_RT)
	$(CMD_RELEASE)$(MSL_RT)