# charted-server Web UI

This is the source code of the optional, but recommended way for interacting with your Helm Charts. It is a simple React and Vite application that is fully client-side since we don't need anything server-side, yet!

This is a seperate project outside of the **charted-server** repository. It's isolated to keep the main Kotlin server as a REST API rather than a bundled frontend and backend application.

You can use the `cr.noelware.cloud/charted/web` OR `ghcr.io/charted-dev/charted/web` Docker images to run it in a contained environment with Kubernetes.

## Environment Variables

The web server has support to connect to Sentry with the `WEB_SENTRY_DSN` environment variable

## Development

For development, you will need to run the server in a different terminal! You can achieve it with the `make run` command in the Makefile of the root repository, or run `./gradlew :server:run` to run alongside Gradle.

Now, you can run the `./gradlew :web:dev` Gradle task (or `yarn dev` if you have it installed) to start the Vite development server. It'll try to connect to server (by default, it's `http://localhost:3651`). If the server is not running, then you will get an error saying that it is not running, and the web application will keep calling the `/heartbeat` endpoint to check if the server is OK.

## Production

You can build the Docker image using the `./gradlew :web:distribution:docker:linuxX64` task to use the production bundle in a form of the Docker image that is published to Noelware's Cloud Registry.

You can run the `./gradlew :web:distribution:chart:bundleChart` to bundle the Kubernetes manifests and upload it to Noelware's Charts Registry.

Also, `./gradlew :web:buildDistro` will build the production bundle that Vite emits in `dist/`.
