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

import { type CommonServerOptions, type PluginOption, defineConfig } from 'vite';
import { fileURLToPath } from 'url';
import { readFile } from 'fs/promises';
import { execSync } from 'child_process';
import { resolve } from 'path';
import autoImports from 'unplugin-auto-import/vite';
import vueDevtools from 'vite-plugin-vue-devtools';
import vueRouter from 'unplugin-vue-router/vite';
import vueJsx from '@vitejs/plugin-vue-jsx';
import vue from '@vitejs/plugin-vue';

export default defineConfig(async ({ command }) => {
    const proxy: CommonServerOptions['proxy'] =
        command === 'build'
            ? {}
            : {
                  '/api': {
                      target: 'http://localhost:3651',
                      changeOrigin: true,
                      rewrite(path) {
                          return path.replace(/^\/api/, '');
                      }
                  }
              };

    const plugins: PluginOption[] = [
        autoImports({
            vueTemplate: true,
            imports: ['@vueuse/core', '@vueuse/head', 'pinia', 'vue', 'vue-router'],
            dirs: ['src/components', 'src/composables', 'src/stores'],
            dts: './auto-imports.d.ts'
        }),
        vueRouter({
            dts: true,
            routesFolder: resolve(fileURLToPath(new URL('./src/views', import.meta.url)))
        }),
        vue(),
        vueJsx()
    ];

    if (command === 'serve') {
        plugins.push(vueDevtools());
    }

    return {
        define: {
            __RUNTIME_CONFIG: JSON.stringify({
                buildDate: new Date().toISOString(),
                gitCommit: (() => {
                    try {
                        return execSync('git rev-parse --short=8 HEAD', { encoding: 'utf-8' }).trim();
                    } catch {
                        return 'unknown';
                    }
                })(),
                version: await readFile(resolve(__dirname, '../.charted-version'), 'utf-8')
                    .then((v) => v.trim())
                    .catch((_) => '0.0.0-devel.0')
            })
        },
        resolve: {
            alias: [
                {
                    find: '~/',
                    replacement: resolve(fileURLToPath(new URL('./src/', import.meta.url)))
                }
            ]
        },
        plugins,
        server: {
            proxy
        }
    };
});
