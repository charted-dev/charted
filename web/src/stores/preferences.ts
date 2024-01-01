/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
 * Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

import { isBrowser } from '@noelware/utils';

const STORE = 'hoshi:user:prefs' as const;

/**
 * Type alias of a given Hoshi theme
 */
export type Theme = 'dark' | 'light' | 'system';

/**
 * Represents the preferences that a user can set once they're logged in. This
 * is kept in the browser.
 */
export interface Preferences {
    /**
     * Whether if animations are disabled, by default, this is `'auto'`, which
     * will detect via the browser.
     */
    disableAnimations: boolean | 'auto';

    /**
     * Controls all toast sensitivity, read more in the Radix Vue docs:
     * https://www.radix-vue.com/components/toast.html#sensitivity
     */
    toastType: 'foreground' | 'background';

    /**
     * Preferred theme. If `'system'` is used, then it'll use the
     * browser's preferred theme.
     */
    theme: Theme;
}

export const usePreferencesStore = defineStore(STORE, () => {
    // the theme is always present in `charted:color-scheme` since index.html does the
    // look-up automatically.
    const theme = ref(useLocalStorage<Theme>('charted:color-scheme', 'system', { writeDefaults: true }));
    const prefs = ref(
        useLocalStorage<Omit<Preferences, 'theme'>>(
            STORE,
            {
                disableAnimations: 'auto',
                toastType: 'foreground'
            },
            { deep: true, writeDefaults: true }
        )
    );

    const isAnimationDisabled = computed(() => {
        // assume that if we're in SSR environment (which we never are),
        // then disable animations anyway
        if (!isBrowser) return false;

        if (typeof prefs.value.disableAnimations === 'string' && prefs.value.disableAnimations === 'auto') {
            // if there is no preference, just enable it by default
            if (window.matchMedia('(prefers-reduced-motion: no-preference)').matches) {
                return false;
            }

            const mql = window.matchMedia('(prefers-reduced-motion: reduced)');
            return mql.matches;
        }

        // if `disableAnimations` is a boolean, then rely on that instead
        if (typeof prefs.value.disableAnimations === 'boolean') {
            return prefs.value.disableAnimations;
        }

        throw new Error(
            `Expected preference \`disableAnimations\` to be a boolean (true/false) or 'auto', received: ${JSON.stringify(
                prefs.value.disableAnimations
            )}`
        );
    });

    return {
        isAnimationDisabled,
        preferences: prefs,
        theme
    };
});

if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(usePreferencesStore, import.meta.hot));
}
