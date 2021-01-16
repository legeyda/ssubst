

SOURCE_FILES:=Cargo.toml src/lib.rs src/main.rs src/queue.rs
OUTPUT_BINARY:=target/release/ssubst




.PHONY: build
bould: ${OUTPUT_BINARY}


${OUTPUT_BINARY}: ${SOURCE_FILES}
	cargo build -Clink-args=-static-libgcc --release

.PHONY: clean
clean:
	cargo clean




.PHONY: test
test: build
	${OUTPUT_BINARY} && false || true
	test -z "$$(echo '' | ${OUTPUT_BINARY} a b)"
	test "_$$(echo 'a' | ${OUTPUT_BINARY} a b)" = "_b"
	test "_$$(echo 'to be or not to be' | ${OUTPUT_BINARY} to be be to)" = "_be to or not be to"
	test "_$$(echo 'abc' | ${OUTPUT_BINARY} a x)" = "_xbc"
	test "_$$(echo 'abc' | ${OUTPUT_BINARY} c x)" = "_abx"
	test "_$$(echo 'привет, русские буквы' | ${OUTPUT_BINARY} р Р)" = "_пРивет, Русские буквы"
	echo tests passed ok
