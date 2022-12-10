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

import '@fontsource/jetbrains-mono/index.css';
import '@fontsource/cantarell/index.css';
import '@fontsource/inter/index.css';

import '../styles/tailwind.css';
import '../styles/twemoji.css';

import type { FC, PropsWithChildren, ReactElement, ReactNode } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import type { AppProps } from 'next/app';
import type { NextPage } from 'next';

export type PageWithLayout<P = {}, IP = P> = NextPage<P, IP> & {
  withLayout?(page: ReactElement): ReactNode;
};

export type NextAppProps = AppProps & { Component: PageWithLayout };

const Application: FC<PropsWithChildren<NextAppProps>> = ({ Component, pageProps }) => {
  const queryClient = new QueryClient();
  const withLayout = Component.withLayout || ((page) => page);

  return <QueryClientProvider client={queryClient}>{withLayout(<Component {...pageProps} />)}</QueryClientProvider>;
};

export default Application;
