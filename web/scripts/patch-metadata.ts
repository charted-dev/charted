/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
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

import { readFile, writeFile } from 'fs/promises';
import { execSync } from 'child_process';

const main = async () => {
  console.log('Patching metadata.json file...');

  const template = await readFile('./metadata.json', 'utf-8').then((f) => JSON.parse(f));
  await writeFile(
    './dist/.metadata.json',
    JSON.stringify({
      build_date: new Date().toISOString(),
      commit_hash: execSync('git rev-parse --short=8 HEAD', { encoding: 'utf-8' }).trim(),
      version: template.version
    })
  );
};

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
