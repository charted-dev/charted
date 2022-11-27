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

import { info, warning, error } from '@actions/core';
import { ESLint } from 'eslint';

const main = async () => {
  info('[eslint] Starting linter...');

  const eslint = new ESLint({ useEslintrc: true });
  info('[eslint] Now linting frontend files...');

  const results = await eslint.lintFiles(['src/**/*.{ts,tsx}']);
  for (const result of results) {
    for (const message of result.messages) {
      const fn = message.severity === 1 ? warning : error;
      fn(`[web] ${result.filePath}:${message.line}:${message.column} [${message.ruleId}] :: ${message.message}`, {
        file: result.filePath,
        endColumn: message.endColumn,
        endLine: message.endLine,
        startColumn: message.column,
        startLine: message.line,
        title: `[${message.ruleId}] ${message.message}`
      });
    }
  }

  info('[eslint] Now linting server files...');
  // soon:tm:
};

main().catch((ex) => {
  console.error(ex);
  process.exit(0);
});
