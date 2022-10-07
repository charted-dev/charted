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

import { createStitches, PropertyValue } from '@stitches/react';

const fonts = {
  serif: ['ui-serif', 'Georgia', 'Cambria', '"Times New Roman"', 'Times', 'serif'],
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

const { getCssText, styled, createTheme, globalCss, theme, keyframes } = createStitches({
  theme: {
    fonts: {
      serif: `Cantarell, ${fonts.serif.join(', ')}`,
      sans: `Inter, ${fonts.sans.join(', ')}`,
      mono: `"JetBrains Mono", ${fonts.mono.join(', ')}`
    }
  },
  utils: {
    // #region margin
    mx: (value: PropertyValue<'marginLeft' | 'marginRight'>) => ({ marginLeft: value, marginRight: value }),
    my: (value: PropertyValue<'marginTop' | 'marginBottom'>) => ({ marginTop: value, marginBottom: value }),
    mt: (value: PropertyValue<'marginTop'>) => ({ marginTop: value }),
    mb: (value: PropertyValue<'marginBottom'>) => ({ marginBottom: value }),
    ml: (value: PropertyValue<'marginLeft'>) => ({ marginLeft: value }),
    mr: (value: PropertyValue<'marginRight'>) => ({ marginRight: value }),

    // #region padding
    px: (value: PropertyValue<'paddingLeft' | 'paddingRight'>) => ({ paddingLeft: value, paddingRight: value }),
    py: (value: PropertyValue<'paddingTop' | 'paddingBottom'>) => ({ paddingTop: value, paddingBottom: value }),
    pt: (value: PropertyValue<'paddingTop'>) => ({ paddingTop: value }),
    pb: (value: PropertyValue<'paddingBottom'>) => ({ paddingBottom: value }),
    pl: (value: PropertyValue<'paddingLeft'>) => ({ paddingLeft: value }),
    pr: (value: PropertyValue<'paddingRight'>) => ({ paddingRight: value })
  },
  media: {
    sm: '(min-width: 640px)',
    md: '(min-width: 768px)',
    lg: '(min-width: 1024px)',
    xl: '(min-width: 1280px)',
    '2xl': '(min-width: 1536px)',
    noMotion: '(prefers-reduced-motion: no-preference)'
  }
});

export const dark = createTheme('dark', {
  colors: {
    background: 'rgb(37,32,46)',
    textColor: 'rgb(63,63,70)'
  }
});

export const light = createTheme('pak-light', {
  colors: {
    background: 'rgb(255,214,240)',
    textColor: 'rgb(245,245,245)'
  }
});

// https://github.com/sindresorhus/modern-normalize
export const normalizeCss = globalCss({
  '*, ::before, ::after': {
    boxSizing: 'border-box'
  },

  html: {
    lineHeight: 1.15,
    '-webkit-text-size-adjust': '100%',
    '-moz-tab-size': 4,
    tabSize: 4
  },

  body: {
    margin: 0,
    padding: 0,
    fontFamily: '$sans'
  },

  hr: {
    height: 0,
    color: 'inherit'
  },

  'abbr[title]': {
    textDecoration: 'underlined dotted'
  },

  'b, strong': {
    fontWeight: 'bolder'
  },

  'code, kbd, samp, pre': {
    fontFamily: '$mono',
    fontSize: '1em'
  },

  small: {
    fontSize: '80%'
  },

  'sub, sup': {
    fontSize: '75%',
    lineHeight: 0,
    position: 'relative',
    verticalAlign: 'baseline'
  },

  sub: {
    bottom: '-0.25em'
  },

  sup: {
    top: '-0.5em'
  },

  table: {
    textIndent: 0,
    borderColor: 'inherit'
  },

  'button, input, optgroup, select, textarea': {
    fontFamily: 'inherit',
    fontSize: '100%',
    lineHeight: 1.15,
    margin: 0
  },

  'button, select': {
    textTransform: 'none'
  },

  'button, [type="button"], [type="reset"], [type="submit"]': {
    '-webkit-appearance': 'button'
  },

  '::-moz-focus-inner': {
    borderStyle: 'none',
    padding: 0
  },

  ':-moz-focusring': {
    outline: '1px dotted ButtonText'
  },

  ':-moz-ui-invalid': {
    boxShadow: 'none'
  },

  legend: {
    padding: 0
  },

  progress: {
    verticalAlign: 'baseline'
  },

  '::-webkit-inner-spin-button, ::-webkit-outer-spin-button': {
    height: 'auto'
  },

  '[type="search"]': {
    '-webkit-appearance': 'textfield',
    outlineOffset: '-2px'
  },

  '::-webkit-search-decoration': {
    '-webkit-appearance': 'none'
  },

  '::-webkit-file-upload-button': {
    '-webkit-appearance': 'button',
    font: 'inherit'
  },

  summary: {
    display: 'list-item'
  }
});

export { getCssText, styled, globalCss, theme, keyframes };
