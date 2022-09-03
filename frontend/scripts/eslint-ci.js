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

// @ts-check

const { info, warning, error } = require('@actions/core');
const { assertIsError } = require('@noelware/utils');
const { ESLint } = require('eslint');

const main = async () => {
  info('starting linter...');

  const eslint = new ESLint({
    useEslintrc: true
  });

  info('linting frontend files...');
  const results = await eslint.lintFiles(['src/**/*.{ts,tsx}']);

  for (const result of results) {
    for (const message of result.messages) {
      const fn = message.severity === 1 ? warning : error;
      fn(`${result.filePath}:${message.line}:${message.column} [${message.ruleId}] :: ${message.message}`, {
        file: result.filePath,
        endColumn: message.endColumn,
        endLine: message.endLine,
        startColumn: message.column,
        startLine: message.line,
        title: `[${message.ruleId}] ${message.message}`
      });
    }
  }

  info('linting server files...');
  try {
    const resultsBack = await eslint.lintFiles(['server/**/*.ts']);
    for (const result of resultsBack) {
      for (const message of result.messages) {
        const fn = message.severity === 1 ? warning : error;
        fn(`${result.filePath}:${message.line}:${message.column} [${message.ruleId}] :: ${message.message}`, {
          file: result.filePath,
          endColumn: message.endColumn,
          endLine: message.endLine,
          startColumn: message.column,
          startLine: message.line,
          title: `[${message.ruleId}] ${message.message}`
        });
      }
    }
  } catch (e) {
    assertIsError(e);
    if (!e.message.includes('No files matching')) throw e;
  }
};

main().catch((ex) => {
  console.error(ex);
  process.exit(0);
});