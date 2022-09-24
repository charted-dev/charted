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

import { Suspense, lazy, type FC } from 'react';
import { Routes, Route } from 'react-router-dom';

// lazy load all components
const Index = lazy(() => import('./views/index'));

const AppRouter: FC = () => (
  // the loader will be replaced with a cute spinner :3
  <Suspense fallback={<div>Loading...</div>}>
    <Routes>
      <Route index element={<Index />} />
    </Routes>
  </Suspense>
);

export default AppRouter;
