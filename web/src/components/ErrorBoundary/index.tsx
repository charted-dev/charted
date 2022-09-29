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

import { Component, ErrorInfo, type PropsWithChildren } from 'react';

/**
 * Represents the error boundary
 */
class ErrorBoundary extends Component<PropsWithChildren<{}>, Record<string, any>> {
  constructor(props: PropsWithChildren<{}>) {
    super(props);
    this.state = { error: null };
  }

  static getDerivedStateFromError(error: any) {
    return { error };
  }

  override componentDidCatch(error: Error, info: ErrorInfo) {
    // TODO: find a way to send to Sentry. Since we don't know what
    //       Sentry instance to use at build-time, this is a bit tricky.
  }

  override render() {
    if (this.state.error !== null) return <h1>Something went wrong. :(</h1>;
    return this.props.children;
  }
}

export default ErrorBoundary;
