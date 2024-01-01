/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
 * Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import { hasOwnProperty } from "@noelware/utils";
import { readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import openapi from "openapi-typescript";

// People could theoritically add their own JS_BINARY__ env if not on Bazel,
// but whatever I guess...
const INVOKED_BY_BAZEL = Object.keys(process.env).some((s) =>
  s.startsWith("JS_BINARY__"),
);

// aspect's rules-js exposes a JS_BINARY__RUNFILES environment variable, where we can find files
// from Bazel's runfiles!
//
// Since we expose 'assets/openapi.json' to runfiles, we can access it without breaking the
// Bazel sandbox.
const PWD =
  INVOKED_BY_BAZEL && hasOwnProperty(process.env, "JS_BINARY__RUNFILES")
    ? resolve(process.env.JS_BINARY__RUNFILES!, "org_noelware_charted_server")
    : resolve(process.cwd(), "../..");

const SCHEMA_FILE = resolve(PWD, "assets/openapi.json");

async function main() {
  const dts = await openapi(SCHEMA_FILE, {
    immutableTypes: true,
    supportArrayLength: true,
  });

  await writeFile(resolve(PWD, "types/js/src/generated.d.ts"), dts, {
    encoding: "utf-8",
  });
  console.log(
    `generate successfully in ${resolve(PWD, "types/js/src/generated.d.ts")}`,
  );
}

main().catch((ex) => {
  console.error(ex.message);
  process.exit(1);
});
