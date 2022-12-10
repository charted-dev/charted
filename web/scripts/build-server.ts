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

import esbuildPathsPlugin from 'esbuild-plugin-alias-path';
import { readdir } from '@noelware/utils';
import { build } from 'esbuild';
import { join, resolve } from 'path';

const main = async () => {
  console.log('Building server code...');

  const serverPath = join(process.cwd(), 'server');
  const entrypoints = await readdir(serverPath, { extensions: [/.ts?x$/] });

  await build({
    entryPoints: entrypoints,
    outdir: join(process.cwd(), 'dist'),
    format: 'esm',
    tsconfig: join(serverPath, 'tsconfig.json'),
    plugins: [
      esbuildPathsPlugin({
        alias: {
          '~/*': resolve(serverPath)
        },
        cwd: serverPath
      })
    ],
    define: {
      'process.env.NODE_ENV': JSON.stringify('production')
    },
    banner: {
      js: [
        '/*',
        ' * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.',
        ' * Copyright 2022 Noelware <team@noelware.org>',
        ' *',
        ' * Licensed under the Apache License, Version 2.0 (the "License");',
        ' * you may not use this file except in compliance with the License.',
        ' * You may obtain a copy of the License at',
        ' *',
        ' *    http://www.apache.org/licenses/LICENSE-2.0',
        ' *',
        ' * Unless required by applicable law or agreed to in writing, software',
        ' * distributed under the License is distributed on an "AS IS" BASIS,',
        ' * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.',
        ' * See the License for the specific language governing permissions and',
        ' * limitations under the License.',
        ' */',
        '\n'
      ].join('\n')
    }
  });
};

main().catch((ex) => {
  console.error(ex);
  process.exit(1);
});
