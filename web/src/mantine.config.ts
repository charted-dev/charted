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

import type { ColorScheme, MantineThemeOverride } from '@mantine/core';

const fonts = {
  serif: ['ui-serif', 'Georgia', 'Cambira', '"Times New Roman"', 'Times', 'serif'],
  mono: [
    'ui-monospace',
    'SFMono-Regular',
    'Menlo',
    'Monaco',
    'Consolas',
    '"Liberation Mono"',
    '"Courier New"',
    'monospace'
  ],
  sans: [
    'ui-sans-serif',
    'system-ui',
    '-apple-system',
    'BlinkMacSystemFont',
    '"Segoe UI"',
    'Roboto',
    '"Helvetica Neue"',
    'Arial',
    '"Noto Sans"',
    'sans-serif',
    '"Apple Color Emoji"',
    '"Segoe UI Emoji"',
    '"Segoe UI Symbol"',
    '"Noto Color Emoji"',
    'sans-serif'
  ]
};

export const defineMantineTheme = (colorScheme: ColorScheme): MantineThemeOverride => ({
  colorScheme,
  fontFamily: `Inter, ${fonts.sans.join(', ')}`,
  fontFamilyMonospace: `"JetBrains Mono", ${fonts.mono.join(', ')}`,
  black: '#262336',
  white: '#F8F8F8',
  headings: {
    fontFamily: `Cantarell, ${fonts.serif.join(', ')}`
  }
});
