/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

import { Stopwatch } from '@noelware/utils';
import * as log from './util/logging';
import * as prettier from 'prettier';
import * as colors from 'colorette';
import { resolve } from 'node:path';

async function main() {
    const ROOT = Bun.fileURLToPath(new URL('..', import.meta.url));
    log.info(`root directory: ${ROOT}`);

    const config = await prettier.resolveConfig(resolve(ROOT, '.prettierrc.json'));
    if (config === null) {
        throw new Error(`was unable to resolve Prettier config in [${resolve(ROOT, '.prettierrc.json')}] ?!`);
    }

    const glob = new Bun.Glob('**/*.{ts,js,md,yaml,yml,json}');
    log.startGroup('formatting!');
    for await (const file of glob.scan({ cwd: ROOT })) {
        if (file.includes('node_modules') || file.includes('dist')) {
            continue;
        }

        const sw = Stopwatch.createStarted();
        log.info(
            `${colors.isColorSupported ? colors.bold(colors.magenta('START')) : 'START'}   ${resolve(ROOT, file)}`
        );

        // lazily create a Bun.File, which we will use later
        const fileObj = Bun.file(resolve(ROOT, file));
        const info = await prettier.getFileInfo(resolve(ROOT, file));
        if (info.inferredParser === null) {
            log.warn(
                `${colors.isColorSupported ? colors.bold(colors.gray('IGNORED')) : 'IGNORED'}   ${resolve(
                    ROOT,
                    file
                )} ${colors.isColorSupported ? colors.bold(`[${sw.stop()}]`) : `[${sw.stop()}]`}`
            );

            continue;
        }

        const contents = await fileObj.text();
        if (log.ci) {
            const correct = await prettier.check(contents, {
                parser: info.inferredParser,
                ...config
            });

            if (!correct) {
                log.error(
                    `${
                        colors.isColorSupported ? colors.bold(colors.red('FAILED')) : 'FAILED'
                    } file was not properly formatted. run \`bun run fmt\` outside of CI ${
                        colors.isColorSupported ? colors.bold(`[${sw.stop()}]`) : `[${sw.stop()}]`
                    }`,
                    {
                        file: resolve(ROOT, file)
                    }
                );

                continue;
            }

            log.info(
                `${colors.isColorSupported ? colors.bold(colors.magenta('END')) : 'END'}     ${resolve(ROOT, file)} ${
                    colors.isColorSupported ? colors.bold(`[${sw.stop()}]`) : ''
                }`
            );
        } else {
            const formatted = await prettier.format(contents, {
                parser: info.inferredParser,
                ...config
            });

            await Bun.write(fileObj, formatted, { createPath: false });

            log.info(
                `${colors.isColorSupported ? colors.bold(colors.magenta('END')) : 'END'}     ${resolve(ROOT, file)} ${
                    colors.isColorSupported ? colors.bold(`[${sw.stop()}]`) : ''
                }`
            );
        }
    }

    log.endGroup();
    process.exit(0);
}

main().catch((ex) => {
    log.error(ex);
    process.exit(1);
});
