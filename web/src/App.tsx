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
import './styles/twemoji.css';

import type { FC } from 'react';
import CommandMenu from './components/CommandMenu';

const App: FC = () => <CommandMenu />;

export default App;

// import { MantineProvider, ColorSchemeProvider, type ColorScheme } from '@mantine/core';
// import { defineMantineTheme } from './mantine.config';
// import { useLocalStorage } from '@mantine/hooks';
// import { BrowserRouter } from 'react-router-dom';
// import { ErrorBoundary } from '~/components';
// import type { FC } from 'react';
// import AppRouter from './router';

// const App: FC = () => {
//   const [colorScheme, setColorScheme] = useLocalStorage<ColorScheme>({
//     key: 'pak.prefs.color-scheme',
//     defaultValue: 'light',
//     getInitialValueInEffect: true
//   });

//   const toggleColorScheme = (value?: ColorScheme) =>
//     setColorScheme(value || (colorScheme === 'dark' ? 'light' : 'dark'));

//   return (
//     <ColorSchemeProvider colorScheme={colorScheme} toggleColorScheme={toggleColorScheme}>
//       <MantineProvider withGlobalStyles withNormalizeCSS theme={defineMantineTheme(colorScheme)}>
//         <ErrorBoundary>
//           <BrowserRouter>
//             <AppRouter />
//           </BrowserRouter>
//         </ErrorBoundary>
//       </MantineProvider>
//     </ColorSchemeProvider>
//   );
// };

// export default App;
