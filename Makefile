
LOG_LEVEL=debug
LOG_STRING=core=$(LOG_LEVEL),world_gen=$(LOG_LEVEL),world=$(LOG_LEVEL)
LOG_ENV_STRING=RUST_LOG="$(LOG_STRING)"

.PHONY: all check release debug tests bench build_release build_debug clean tags resource_compress resource_extract

all: tags release

check: tests bench

release: build_release
	$(LOG_ENV_STRING) cargo run --release --bin world_gen

debug: build_debug
	$(LOG_ENV_STRING) cargo run --bin world_gen

tests: build_debug
	$(LOG_ENV_STRING) cargo test --lib -p core -p world

bench: build_debug
	cargo bench --lib -p world

build_release:
	cargo build --release --bin world_gen

build_debug:
	cargo build --bin world_gen

tags:
	ctags -R bin core/src world/src

resource_compress:
	tar cvzf resources.tar.gz resources/

resource_extract:
	tar xvzf resources.tar.gz

clean:
	rm -rf target core/target world/target

