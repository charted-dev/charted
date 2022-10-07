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

import { reportWebVitals } from './web-vitals';
import { createRoot } from 'react-dom/client';
import { StrictMode } from 'react';
import App from './App';

console.info(
  `%c     _           _         _
 ___| |_ ___ ___| |_ ___ _| |
|  _|   | .'|  _|  _| -_| . |
|___|_|_|__,|_| |_| |___|___|`,
  'color:#F4B5D5;font-weight:bold;',
  '\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~'
);

const rootEl = document.getElementById('root');
if (!rootEl) throw new Error('Cannot find root element!');

const root = createRoot(rootEl);
root.render(
  <StrictMode>
    <App />
  </StrictMode>
);

if (import.meta.env.DEV) {
  reportWebVitals((metric) => {
    console.log(
      `[${metric.name} / ${metric.rating} (navigation: ${metric.navigationType})] Took ~${
        metric.delta === metric.value
          ? `${metric.value.toFixed(2)}ms (first reporting web vital)`
          : `${(metric.delta - metric.value).toFixed(2)}ms`
      } to complete`
    );
  });
}
