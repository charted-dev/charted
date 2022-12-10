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

import type { FastifyReply, FastifyRequest } from 'fastify';
import { getMetadata } from '~/metadata';
import type Server from '~/server';

export function stats(this: Server, _: FastifyRequest, reply: FastifyReply) {
  const metadata = getMetadata();
  const memoryUsage = process.memoryUsage();

  return reply.status(200).send({
    success: true,
    data: {
      commit_hash: metadata.commit_hash,
      build_date: metadata.build_date,
      version: metadata.version,
      process: {
        uptime: Math.floor(process.uptime() * 1000),
        memory_usage: {
          heap_used: memoryUsage.heapUsed,
          heap_total: memoryUsage.heapTotal,
          rss: memoryUsage.rss
        }
      }
    }
  });
}
