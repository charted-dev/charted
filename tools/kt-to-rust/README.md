# Kotlin data class to Rust struct

This tool generates Rust structs based off the given Kotlin class.

## Usage

```shell
./gradlew :tools:kt-to-rust:run
```

It will generate a Rust file in **tools/helm-plugin/src/api/generated_stub.rs** with the generated Rust structures with the Kotlin data classes
in `databases/postgres`.
