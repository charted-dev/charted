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

import { type Component, type VNode } from 'vue';

const defaultDuration = 5000; // 5 seconds

export type ToastType = 'error' | 'warning' | 'notice';
type ToastMessage = string | VNode | (() => VNode);

export interface ToastOptions {
    toastRootType?: 'background' | 'foreground';
    duration?: number;
    action?: Component;
    title?: string;
    type: ToastType;
}

export interface ToastReturnType {
    dismiss(): void;
}

export interface ToastAPI {
    warning(message: ToastMessage, options?: Omit<ToastOptions, 'type'>): ToastReturnType;
    notice(message: ToastMessage, options?: Omit<ToastOptions, 'type'>): ToastReturnType;
    error(message: ToastMessage, options?: Omit<ToastOptions, 'type'>): ToastReturnType;
    open(message: ToastMessage, options: ToastOptions): ToastReturnType;

    toasts: ComputedRef<Toast[]>;
}

export interface Toast extends ToastOptions {
    toastRootType: 'background' | 'foreground';
    message: ToastMessage;
    id: number;

    onOpenChange?(open: boolean): void;
}

type ReducerAction =
    | {
          type: 'dismiss';
          toastId?: number;
      }
    | { type: 'open'; toast: Toast };

const state = ref<{ toasts: Toast[] }>({ toasts: [] });
const timerQueue = new Map<number, ReturnType<typeof setTimeout>>();
let counter = 0;

const genId = () => counter++ % Number.MAX_SAFE_INTEGER;
export const useToast = (): ToastAPI => ({
    toasts: computed(() => state.value.toasts),
    open,

    warning(message, options = {}) {
        return open(message, { ...options, type: 'warning' });
    },

    notice(message, options = {}) {
        return open(message, { ...options, type: 'notice' });
    },

    error(message, options = {}) {
        return open(message, { ...options, type: 'error' });
    }
});

const reducer = ({ type, ...data }: ReducerAction) => {
    switch (type) {
        case 'dismiss':
            {
                const { toastId } = (data as { toastId?: number }) || {};

                if (toastId !== undefined) {
                    state.value.toasts = state.value.toasts.filter((t) => t.id !== toastId);
                } else {
                    state.value.toasts = [];
                }
            }
            break;

        case 'open': {
            state.value.toasts = [...state.value.toasts, (data as { toast: Toast }).toast];
            break;
        }

        default:
            throw new Error(`Unknown reducer type for ToastAPI: ${type}; data=${JSON.stringify(data)}`);
    }
};

const open: ToastAPI['open'] = (message, options) => {
    const id = genId();
    const dismiss = (toastId?: number) => reducer({ type: 'dismiss', toastId });
    const duration = options.duration ?? defaultDuration;
    if (duration !== -1) {
        const onDismissTimeout = setTimeout(() => {
            reducer({ type: 'dismiss', toastId: id });
            timerQueue.delete(id);
        }, duration);

        timerQueue.set(id, onDismissTimeout);
    }

    let type = options.toastRootType;
    if (type !== undefined) {
        const prefs = usePreferencesStore();
        type = prefs.preferences.toastType;
    }

    const toast = {
        toastRootType: type as unknown as NonNullable<ToastOptions['toastRootType']>,
        ...options,
        message,
        id

        // onOpenChange(open) {
        //     if (!open) {
        //         dismiss();
        //     }
        // }
    } satisfies Toast;

    reducer({ type: 'open', toast });
    return { dismiss };
};
