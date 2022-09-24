# charted-server Web UI
This is the source code of the bundled UI for interacting with your Helm Charts. It is a simple React and Vite application that is fully client-side since we don't need anything server-side, yet!

## Production
To build a production version of the web UI, you will need to run `yarn build` and run `./gradlew :server:collectWebUI` to export the web UI to the JAR file that the server can statically host.

## Development
For development, you will need the server running with the `make run` command, or with `./gradlew :server:run` Gradle task. Then, you can start the server with `yarn dev` in `web/` or run `./gradlew :web:dev` to run the development version of the web UI.

When using the **./gradlew :web:dev** command, it need to know if you installed Node.js or not, by default, it will not install the one it wants. If you want it to install the Node.js it requires, you can:

- Run `asdf install` if you're using **asdf-vm**,
- Run the `./gradlew :web:extractInstallation` to download the Node.js tarball/zip file, and extract it into `$ROOT/build/nodejs/{version}`.
