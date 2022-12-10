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

import type { Configuration } from './config';
import { useLoggerFactory } from './logging';
import { assertIsError } from '@noelware/utils';
import { STATUS_CODES } from 'http';
import { gracefulExit } from 'exit-hook';
import type { Logger } from './logging/Logger';
import * as routes from './routes';
import pTimeout from 'p-timeout';
import fastify from 'fastify';
import http from '@aero/http';
import next from 'next';

/**
 * Represents the web UI's health status.
 *
 * - **green** represents both the API server and Web UI are running fine
 * - **yellow** represents that the web UI is running, but the API server is not communicating with the web UI. Display a error
 *   on the web UI side.
 * - **red** is the default when {@link Server} is constructed.
 */
export type ServerHealth = 'green' | 'red' | 'yellow';

export default class Server {
  private _serverHealthInterval?: NodeJS.Timeout;
  private hasProbedApiServer = false;
  private available: [api: boolean, web: boolean] = [false, false];
  private server: ReturnType<typeof fastify>;
  private app!: ReturnType<typeof next>;
  private log: Logger;

  constructor(private config: Configuration) {
    const loggerFactory = useLoggerFactory();

    this.log = loggerFactory.getLogger('charted', 'web', 'server');
    this.server = fastify({
      logger: this.log.inner,
      bodyLimit: 7886432000, // 750mb should be a good default, i think
      exposeHeadRoutes: true,
      requestIdHeader: 'X-Request-Id',
      return503OnClosing: true
    });
  }

  /**
   * Returns the web UI's {@link ServerHealth health}.
   */
  get health(): ServerHealth {
    if (this.available[0] && this.available[1]) return 'green';
    else if (!this.available[0] && this.available[1]) return 'yellow';
    else return 'red';
  }

  /**
   * Returns the absolute URL for the API server.
   */
  get apiServerUrl() {
    return `http${this.config.charted.ssl !== undefined ? 's' : ''}://${this.config.charted.host}:${
      this.config.charted.port
    }`;
  }

  private async performHealthcheck() {
    this.log.debug(`Performing healthcheck on endpoint [${this.apiServerUrl}/heartbeat]`);
    const old = this.health;

    try {
      const res = await http(`${this.apiServerUrl}/heartbeat`)
        .header('User-Agent', 'Noelware/charted-web (healthcheck-probe)')
        .get();

      if (res.statusCode !== 200) {
        throw new Error(
          `Received a non-successful status code [${res.statusCode} ${STATUS_CODES[res.statusCode] ?? 'Unknown'}]`
        );
      }

      const level = this.hasProbedApiServer ? 'debug' : 'info';
      this.log[level](`API server on address [${this.apiServerUrl}] is available!`);

      if (!this.hasProbedApiServer) this.hasProbedApiServer = true;
      this.available[0] = true;
    } catch (e) {
      assertIsError(e);

      // If the API server was not probed by #start(), just throw the error
      // since we will shut down the web ui.
      if (!this.hasProbedApiServer) throw e;

      // Just log a error and update the state so the Next app can show a "API server is not well" message
      this.available[0] = false;
      this.log.error(
        { error: e },
        `Unable to send a request to ${this.apiServerUrl}/health (changed status from ${old} -> ${this.health})`
      );
    }
  }

  async start() {
    this.log.info(`Starting web UI on address [http://${this.config.server.host}:${this.config.server.port}]`);
    // if (this.config.proxy !== undefined) {
    //   this.log.info(`Configuring ${this.config.proxy.length} prox${this.config.proxy.length === 1 ? 'y' : 'ies'}...`);
    //   for (const proxy of this.config.proxy) {
    //     if (!proxy.serverPath.startsWith('/')) {
    //       this.log.warn(`Skipping proxy (${JSON.stringify(proxy)}) due to serverPath not starting with /!`);
    //       continue;
    //     }

    //     if (proxy.path.startsWith('http')) {
    //       this.log.info(
    //         `  Proxy will be configured from http://${this.config.server.host}:${this.config.server.port}${proxy.serverPath} -> ${proxy.path}`
    //       );
    //     } else {
    //       this.log.info(
    //         `  Proxy will be configured from http://${this.config.server.host}:${this.config.server.port}${proxy.serverPath} -> ${proxy.path}`
    //       );
    //     }

    //     this.server.register(httpProxy, {
    //       upstream: proxy.path,
    //       http2: false,
    //       rewritePrefix: proxy.serverPath
    //     });
    //   }
    // }

    this.log.info('Starting health probe...');
    try {
      await this.performHealthcheck();
    } catch (e) {
      assertIsError(e);

      this.log.error({ error: e }, `Unable to send a request to ${this.apiServerUrl}/heartbeat:`);
      gracefulExit(1);
    }

    if (this.config.charted.healthcheck !== undefined) {
      if (this.config.charted.healthcheck !== null) {
        this._serverHealthInterval = setInterval(
          () => this.performHealthcheck.bind(this),
          typeof this.config.charted.healthcheck.interval === 'number'
            ? this.config.charted.healthcheck.interval ?? 30000
            : 30000
        );
      }
    }

    this.log.info('Starting Next.js application...');
    this.app = next({
      dev: process.env.NODE_ENV === 'development',
      dir: process.env.NODE_ENV === 'development' ? process.cwd() : undefined,
      conf: {
        productionBrowserSourceMaps: process.env.NODE_ENV === 'production',
        reactStrictMode: true,
        redirects: async () => {
          return [
            // Proxies all routes like "/~/<userOrOrg>/index.yaml" (i.e, https://charts.noelware.org/~/noelware/index.yaml -> https://cdn.noelware.cloud/charts/metadata/<noelware org id>/index.yaml)
            {
              source: '/~/:userOrOrg/index.yaml',
              destination: `${this.apiServerUrl}/cdn/metadata/:userOrOrg/index.yaml`,
              permanent: true
            }
          ];
        },
        experimental: {
          optimizeCss: true,
          swcMinify: process.env.NODE_ENV === 'production'
        },
        eslint: {
          ignoreDuringBuilds: true
        }
      }
    });

    await this.app.prepare();

    const handle = this.app.getRequestHandler();
    this.server.all('*', async (req, reply) => {
      for (const [header, value] of Object.entries(reply.getHeaders())) {
        reply.raw.setHeader(header, value as any);
      }

      reply.hijack();
      return handle(req.raw, reply.raw).then(() => reply.hijack());
    });

    this.server.get('/_web/health', routes.health.bind(this));
    this.server.get('/_web/stats', routes.stats.bind(this));

    return pTimeout(
      new Promise<void>((resolve, reject) =>
        this.server.listen({ port: this.config.server.port, host: this.config.server.host }, (error) => {
          if (error !== null) return reject(error);

          this.available[1] = true;
          resolve();
        })
      ),
      { milliseconds: 15000 }
    );
  }

  async stop() {
    await this.server.close();
    await this.app?.close();
  }
}
