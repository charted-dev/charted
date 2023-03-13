/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.models.flags

/**
 * Represents all the API Key scopes available. This must be in-sync with [SCOPES].
 */
public sealed class ApiKeyScope(public val key: String) {
    /**
     * Scopes for accessing the Users REST controller.
     */
    public object User {
        /**
         * Allows the API key to access non-sensitive user information
         */
        public object Access: ApiKeyScope("user:access")

        /**
         * Allows the API key to update any user metadata
         */
        public object Update: ApiKeyScope("user:update")

        /**
         * Allows the API key to delete the user that this key belongs
         * to.
         */
        public object Delete: ApiKeyScope("user:delete")

        /**
         * Allows the API key to access user-connection metadata
         */
        public object Connections: ApiKeyScope("user:connections")

        /**
         * Allows the API key to access user notifications. This scope
         * is currently not used as of v0.4-nightly
         */
        public object Notifications: ApiKeyScope("user:notifications")

        /**
         * Allows access to the User Avatars REST controller.
         */
        public object Avatar {
            /**
             * Allows this API key to update the user's avatar that this key
             * belongs to
             */
            public object Update: ApiKeyScope("user:avatar:update")
        }

        /**
         * Allows access to the User Sessions REST controller
         */
        public object Sessions {
            /**
             * Allows the API key to access all user session IDs. This doesn't expose
             * any JWT tokens that were created for each session.
             */
            public object List: ApiKeyScope("user:sessions:list")
        }
    }

    /**
     * Scopes for accessing the Repositories REST controller.
     */
    public object Repositories {
        /**
         * Allows the API key to access non-sensitive repository metadata
         */
        public object Access: ApiKeyScope("repo:access")

        /**
         * Allows the API key to create repositories in the user's name
         * or the organization that this user owns/has access
         */
        public object Create: ApiKeyScope("repo:create")

        /**
         * Allows the API key to update any repository metadata
         */
        public object Update: ApiKeyScope("repo:update")

        /**
         * Allows the API key to delete the repository that this key
         * has access to
         */
        public object Delete: ApiKeyScope("repo:delete")

        /**
         * Allows the API key to write any repository metadata
         */
        public object Write: ApiKeyScope("repo:write")

        /**
         * Scopes for accessing the Repository Icons REST controller
         */
        public object Icons {
            /**
             * Allows the API key to update the repository's icon
             */
            public object Update: ApiKeyScope("repo:icons:update")
        }

        /**
         * Scopes for accessing the Repository Releases REST controller
         */
        public object Releases {
            public object Create: ApiKeyScope("repo:releases:create")
            public object Delete: ApiKeyScope("repo:releases:delete")
            public object Update: ApiKeyScope("repo:releases:update")
        }

        /**
         * Scopes for accessing the Repository Members REST controller
         */
        public object Members {
            public object List: ApiKeyScope("repo:members:list")
            public object Kick: ApiKeyScope("repo:members:kick")
            public object Update: ApiKeyScope("repo:members:update")

            /**
             * Scopes for accessing the Repository Member Invites REST controller
             */
            public object Invites {
                public object Access: ApiKeyScope("repo:members:invites:access")
                public object Create: ApiKeyScope("repo:members:invites:create")
                public object Delete: ApiKeyScope("repo:members:invites:delete")
            }
        }

        /**
         * Scopes for accessing the Repository Webhooks REST controller
         */
        public object Webhooks {
            /**
             * Allows the API key to list all webhooks available. This doesn't list
             * off the endpoints or authentication key that the endpoint might require.
             */
            public object List: ApiKeyScope("repo:webhooks:list")

            /**
             * Allows the API key to create webhooks.
             */
            public object Create: ApiKeyScope("repo:webhooks:create")

            /**
             * Allows the API key to update any webhook metadata.
             */
            public object Update: ApiKeyScope("repo:webhooks:update")

            /**
             * Allows the API key to delete webhooks
             */
            public object Delete: ApiKeyScope("repo:webhooks:delete")

            /**
             * Scopes for accessing the Repository Webhook Events REST controller
             */
            public object Events {
                /**
                 * Allows the API key to access the webhook events API. This can reveal
                 * sensitive information.
                 */
                public object Access: ApiKeyScope("repo:webhooks:events:access")

                /**
                 * Allows the API key to delete any webhook event they wish to.
                 */
                public object Delete: ApiKeyScope("repo:webhooks:events:delete")
            }
        }
    }

    /**
     * Scopes to access the API Keys REST controller
     */
    public object ApiKeys {
        /**
         * Allows the API key to list other API keys that the user who owns
         * this API key has.
         */
        public object Access: ApiKeyScope("apikeys:view")
        public object Create: ApiKeyScope("apikeys:create")
        public object Update: ApiKeyScope("apikeys:update")
        public object Delete: ApiKeyScope("apikeys:delete")
    }

    public object Organizations {
        public object Access: ApiKeyScope("org:access")
        public object Create: ApiKeyScope("org:create")
        public object Update: ApiKeyScope("org:update")
        public object Delete: ApiKeyScope("org:delete")

        /**
         * Scopes for accessing the Organization Members REST controller
         */
        public object Members {
            public object List: ApiKeyScope("org:members:list")
            public object Kick: ApiKeyScope("org:members:kick")
            public object Update: ApiKeyScope("org:members:update")
            public object Invites: ApiKeyScope("org:members:invites")
        }

        /**
         * Scopes for accessing the Organization Webhooks REST controller
         */
        public object Webhooks {
            /**
             * Allows the API key to list all webhooks available. This doesn't list
             * off the endpoints or authentication key that the endpoint might require.
             */
            public object List: ApiKeyScope("org:webhooks:list")

            /**
             * Allows the API key to create webhooks.
             */
            public object Create: ApiKeyScope("org:webhooks:create")

            /**
             * Allows the API key to update any webhook metadata.
             */
            public object Update: ApiKeyScope("org:webhooks:update")

            /**
             * Allows the API key to delete webhooks
             */
            public object Delete: ApiKeyScope("org:webhooks:delete")

            /**
             * Scopes for accessing the Repository Webhook Events REST controller
             */
            public object Events {
                /**
                 * Allows the API key to access the webhook events API. This can reveal
                 * sensitive information.
                 */
                public object Access: ApiKeyScope("org:webhooks:events:list")

                /**
                 * Allows the API key to delete any webhook event they wish to.
                 */
                public object Delete: ApiKeyScope("org:webhooks:events:delete")
            }
        }
    }

    /**
     * Allows the API key to access admin information. The user needs to be
     * an administrator to access the Admin REST controller
     */
    public object Admin {
        /**
         * Allows the API key to access admin statistics that can be sensitive.
         */
        public object Stats: ApiKeyScope("admin:stats")
    }
}
