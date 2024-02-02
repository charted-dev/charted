# charted devtools

This repository is for helping to run charted's sources. Since charted's source code can seem intimidating at first, it's not really since this exists.

The devtool itself lives inside of the `//:devtools` Bazel target, which you can easily use with the convienent `./dev` script in the root directory.

-   `./dev services start [service]` - Starts a microservice in `./services`
-   `./dev services stop [service]` - Stops a microservice, if it is running.
-   `./dev helm-plugin` - Launches the Helm plugin
-   `./dev docker down` - Destroys the Docker compose project.
-   `./dev docker up` - Starts up the Docker compose project.
-   `./dev sync-deps` - Synchronizes crate dependencies
-   `./dev server` - Runs the API server
-   `./dev cli` - Runs the CLI and feeds the rest of the arguments after `cli` to the CLI
-   `./dev web` - Starts the web server in development mode, it will launch `./dev server` in the background

## Environment Variables

-   `DEV_REBUILD=...` - Recompiles the devtools binary and re-cache it in .cache/dev. This should be only for debugging or developing the devtool.
