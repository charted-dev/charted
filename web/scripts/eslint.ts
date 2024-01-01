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

// @ts-ignore
import { FlatESLint } from 'eslint/use-at-your-own-risk';
import { Stopwatch } from '@noelware/utils';
import * as log from './util/logging';
import type { ESLint } from 'eslint';
import * as colors from 'colorette';
import { resolve } from 'node:path';

async function main() {
    const ROOT = Bun.fileURLToPath(new URL('..', import.meta.url));
    log.info(`root directory: ${ROOT}`);

    const linter = new FlatESLint({
        allowInlineConfig: true,
        fix: !log.ci,
        cwd: ROOT
    });

    const glob = new Bun.Glob('**/*.ts');
    const formatter = await linter.loadFormatter('codeframe');

    log.startGroup(`linting directory [${resolve(ROOT)}]`);
    for await (const file of glob.scan({ cwd: ROOT })) {
        if (file.includes('node_modules') || file.includes('dist')) {
            continue;
        }

        const sw = Stopwatch.createStarted();
        log.info(
            `${colors.isColorSupported ? colors.bold(colors.magenta('START')) : 'START'}   ${resolve(ROOT, file)}`
        );

        const contents = await Bun.file(resolve(ROOT, file)).text();
        const results: ESLint.LintResult[] = await linter.lintText(contents, {
            filePath: resolve(ROOT, file)
        });

        if (!log.ci) {
            const shouldPrint = formatter.format(results);
            shouldPrint.length > 0 && console.log(shouldPrint);
        } else {
            for (const result of results) {
                for (const msg of result.messages) {
                    switch (msg.severity) {
                        case 0:
                            continue;

                        case 1:
                            log.warn(
                                `[${msg.ruleId || '(unknown rule)'}] ${msg.message} (line ${msg.line}:${msg.column})`
                            );
                            continue;

                        case 2:
                            log.error(
                                `${
                                    colors.isColorSupported ? colors.bold(colors.red('FAILED')) : 'FAILED'
                                } file [${file}] has failed to lint properly; run \`bun run lint\` outside of CI to fix it: ${
                                    msg.ruleId || '(unknown rule)'
                                }: ${msg.message}`,
                                {
                                    startColumn: msg.endColumn,
                                    endColumn: msg.endColumn,
                                    startLine: msg.line,
                                    endLine: msg.endLine,
                                    title: `[${msg.ruleId || '(unknown)'}] ${msg.message}`,
                                    file: file
                                }
                            );
                    }
                }
            }
        }

        log.info(
            `${colors.isColorSupported ? colors.bold(colors.magenta('END')) : 'END'}     ${resolve(ROOT, file)} ${
                colors.isColorSupported ? colors.bold(`[${sw.stop()}]`) : ''
            }`
        );
    }

    log.endGroup();
}

main().catch((ex) => {
    log.error(ex);
    process.exit(1);
});
