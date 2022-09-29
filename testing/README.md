# Testing Framework
This subproject contains the framework for doing integration tests with the server and with a Kubernetes cluster with **minikube**
or with the [rancher/k3s](https://hub.docker.com/r/rancher/k3s) Docker image.

## Subprojects
- `:testing:helm` - Installer, and utility module for working with **Helm**
- `:testing:kubernetes` - Framework for bootstrapping Kubernetes (with Minikube or with [rancher/k3s](https://hub.docker.com/r/rancher/k3s))
- `:testing:server` - Framework for creating a minimal, testing instance of the REST API.

## Usage
### :testing:helm
```kotlin
plugins {
    `charted-java-module`
    `charted-module`
}

dependencies {
    implementation(project(":helm"))
}
```

```java
public class Program {
    public static void main(String[] args) {
        // This will install Helm in a temporary directory. This is useful
        // for GitHub Actions.
        // org.noelware.charted.testing.helm.download.DefaultHelmDownloader
        final HelmDownloader downloader = new DefaultHelmDownloader();
        
        // Use the #download(String) method to download a specific version,
        // omit the first argument to use the latest version.
        downloader.download("3.8.0");
        
        // Now, we can create the `Helm` utility class which will give
        // us an abstracted layer to work with the Helm CLI.
        // org.noelware.charted.testing.helm.cli.Helm
        final Helm helm = new Helm("<directory to helm>");
        helm.registry().login("[host]");
    }
}
```
