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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.auditlogs

import org.noelware.ktor.endpoints.AbstractEndpoint

/**
 * Represents the Audit Logs API. Audit logs are a way to introspect any action
 * that was executed at a given time. For an example, if a repository member updates the repo's metadata, an audit log
 * will be fired.
 *
 * You can read more in [the documentation](https://charts.noelware.org/docs/features/audit-logs)!
 */
class AuditLogsApiEndpoints: AbstractEndpoint("/audit-logs")
