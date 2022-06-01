# Docker Registry OCI Proxy
This is the library to interact with the reverse-proxying of a Docker registry, made for Ktor.

Will this be a library under Noelware? - Probably.

## Usage
```kotlin
fun Application.module() {
    install(OciProxyPlugin) {
        registryUri = "http://localhost:5000"
    }
}
```
