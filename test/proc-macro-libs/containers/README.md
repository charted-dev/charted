# [Testcontainers](https://github.com/testcontainers/testcontainers-rs) Proc Macros
This directory is the source code for implementing the `#[charted_testcontainers::test]` macro. This is supposed to be similar in how the [@Testcontainers annotation](https://java.testcontainers.org/test_framework_integration/junit_5) from the JUnit5 integration of [Testcontainers (Java)](https://github.com/testcontainers/testcontainers-java) work.

## Usage
```py
load("//:build/rust_project.bzl", "rust_project")

rust_project(
    name = "some_rust_project",
    include_tests = True,
    test_proc_macro_deps = ["//test/containers:charted_testcontainers"]
)
```

```toml
[dependencies]
charted-testcontainers = { version = "0.0.0-devel.0", path = "../test/containers" }
```

```rs
#[cfg(test)]
mod tests {
    use charted_testcontainers::{testcontainers, Context};
    use testcontainers::Container;

    /// Method to configure the testcontainers macro. This is where
    /// you initialize your containers.
    fn configure() {}

    #[testcontainers(configure)]
    async fn test(ctx: &Context) {
        let postgres_container = ctx.container::<Postgres>();
        assert!(postgres_container.is_some());
    }
}
```
