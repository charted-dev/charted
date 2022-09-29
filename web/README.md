# charted-server Web UI

This is the source code of the bundled UI for interacting with your Helm Charts. It is a simple React and Vite application that is fully client-side since we don't need anything server-side, yet!

This is a seperate project outside of the **charted-server** repository. It's isolated to keep the main Kotlin server as a REST API rather than a bundled frontend and backend application.

## Development

For development, you will need to run the server in a different terminal, you can easily achieve it with `make run` or run `./gradlew :server:run`. You will need PostgreSQL and Redis installed.

Now, you can use the `./gradlew :web:dev` Gradle task or `yarn dev` command to start up the development server. It'll connect with **charted-server** under **localhost:3651** (or whatever port you specified in the `server.port` configuration) and start developing!
