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

import { defineConfig } from 'vite';
import { resolve } from 'path';
import react from '@vitejs/plugin-react';

const MODE = Object.prototype.hasOwnProperty.call(process.env, 'NODE_ENV') ? process.env.NODE_ENV! : 'development';
const serverUrl = Object.prototype.hasOwnProperty.call(process.env, 'SERVER')
  ? process.env.SERVER!
  : 'http://localhost:3651';

export default defineConfig({
  envPrefix: 'CHARTED_',
  plugins: [react()],
  mode: MODE,
  resolve: {
    alias: [
      {
        find: '~',
        replacement: resolve(process.cwd(), 'src')
      }
    ]
  },
  server: {
    port: 4000,
    proxy:
      MODE === 'development'
        ? {
            '/api': {
              target: serverUrl,
              changeOrigin: true,
              rewrite: (path) => path.replace(/^\/api/, '')
            }
          }
        : undefined
  }
});
