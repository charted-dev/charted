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

// This only happens when you run `yarn dev`, so let's force it as "development"
// rather than production.
if (!process.env.NODE_ENV) {
  // @ts-ignore
  process.env.NODE_ENV = 'development';
}

import { asyncExitHook } from 'exit-hook';
import { createLoggerFactory } from './logging';
import { getMetadata } from './metadata';
import { read } from './config';
import Server from './server';

const metadata = getMetadata();

const configPath = process.env.CHARTED_WEB_CONFIG_PATH;
const config = await read(configPath);

const factory = createLoggerFactory(config);
const mainLogger = factory.getLogger('main');

mainLogger.info(`charted web UI v${metadata.version} (${metadata.commit_hash})`);

const server = new Server(config);
await server.start();

asyncExitHook(
  async () => {
    mainLogger.warn('Web UI is shutting down...');
    server.stop();
  },
  { minimumWait: 5000 }
);
