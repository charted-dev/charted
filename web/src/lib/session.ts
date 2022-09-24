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

import { persist, combine, devtools } from 'zustand/middleware';
import type { Mutation } from './types';
import create from 'zustand';

export interface User {
  gravatar_email: string | null;
  description: string | null;
  avatar_hash: string | null;
  created_at: string;
  updated_at: string;
  username: string;
  flags: number;
  name: string | null;
  id: string;
}

export interface Session {
  refreshToken: string | null;
  accessToken: string | null;
  user: User | null;
}

export type SessionMutation = Mutation<Session>;

const sessionStore = persist(
  combine<Session, SessionMutation, [['zustand/persist', unknown]]>(
    {
      refreshToken: null,
      accessToken: null,
      user: null
    },
    (set, fetch) => ({
      get(key) {
        const session = fetch();
        if (Object.prototype.hasOwnProperty.call(session, key)) return session[key];

        return null;
      },
      set(key, value) {
        set((prev) => ({ ...prev, [key]: value }));
      },
      invalidate() {
        set({ accessToken: null, refreshToken: null, user: null });
      }
    })
  ),
  { name: 'pak.session' }
);

// @ts-expect-error It works, alright!
export const useSessionStore = create(process.env.NODE_ENV === 'development' ? devtools(sessionStore) : sessionStore);
