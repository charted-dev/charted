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

package org.noelware.charted.search.elastic

import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

val INDEX_SETTINGS = mapOf(
    "charted-users" to buildJsonObject {
        put(
            "settings",
            buildJsonObject {
                put("number_of_shards", 5)
            }
        )

        put(
            "mappings",
            buildJsonObject {
                put(
                    "properties",
                    buildJsonObject {
                        put(
                            "description",
                            buildJsonObject {
                                put("type", "text")
                            }
                        )

                        put(
                            "username",
                            buildJsonObject {
                                put("type", "text")
                            }
                        )

                        put(
                            "email",
                            buildJsonObject {
                                put("type", "keyword")
                            }
                        )

                        put(
                            "name",
                            buildJsonObject {
                                put("type", "text")
                            }
                        )

                        put(
                            "id",
                            buildJsonObject {
                                put("type", "long")
                                put("index", true)
                            }
                        )
                    }
                )
            }
        )
    },

    "charted-repos" to buildJsonObject {
        put(
            "settings",
            buildJsonObject {
                put("number_of_shards", 5)
            }
        )

        put(
            "mappings",
            buildJsonObject {
                put(
                    "properties",
                    buildJsonObject {
                        put(
                            "description",
                            buildJsonObject {
                                put("type", "text")
                            }
                        )

                        put(
                            "updated_at",
                            buildJsonObject {
                                put("type", "date")
                            }
                        )

                        put(
                            "created_at",
                            buildJsonObject {
                                put("type", "date")
                            }
                        )

                        put(
                            "owner_id",
                            buildJsonObject {
                                put("type", "long")
                            }
                        )

                        put(
                            "name",
                            buildJsonObject {
                                put("type", "text")
                            }
                        )

                        put(
                            "type",
                            buildJsonObject {
                                put("type", "keyword")
                            }
                        )

                        put(
                            "id",
                            buildJsonObject {
                                put("type", "long")
                                put("index", true)
                            }
                        )
                    }
                )
            }
        )
    },

    "charted-orgs" to buildJsonObject {
        put(
            "settings",
            buildJsonObject {
                put("number_of_shards", 5)
            }
        )

        put(
            "mappings",
            buildJsonObject {
                put(
                    "properties",
                    buildJsonObject {
                        put(
                            "verified_publisher",
                            buildJsonObject {
                                put("type", "boolean")
                            }
                        )

                        put(
                            "description",
                            buildJsonObject {
                                put("type", "text")
                                put("index", true)
                            }
                        )

                        put(
                            "updated_at",
                            buildJsonObject {
                                put("type", "date")
                            }
                        )

                        put(
                            "created_at",
                            buildJsonObject {
                                put("type", "date")
                            }
                        )

                        put(
                            "handle",
                            buildJsonObject {
                                put("type", "keyword")
                                put("index", true)
                            }
                        )

                        put(
                            "name",
                            buildJsonObject {
                                put("type", "text")
                                put("index", true)
                            }
                        )

                        put(
                            "id",
                            buildJsonObject {
                                put("type", "long")
                                put("index", true)
                            }
                        )
                    }
                )
            }
        )
    },

    "charted-org-members" to buildJsonObject {
        put(
            "settings",
            buildJsonObject {
                put("number_of_shards", 5)
            }
        )

        put(
            "mappings",
            buildJsonObject {
                put(
                    "properties",
                    buildJsonObject {
                        put(
                            "description",
                            buildJsonObject {
                                put("type", "text")
                                put("index", true)
                            }
                        )

                        put(
                            "updated_at",
                            buildJsonObject {
                                put("type", "date")
                            }
                        )

                        put(
                            "joined_at",
                            buildJsonObject {
                                put("type", "date")
                            }
                        )

                        put(
                            "username",
                            buildJsonObject {
                                put("type", "text")
                            }
                        )

                        put(
                            "email",
                            buildJsonObject {
                                put("type", "keyword")
                            }
                        )

                        put(
                            "name",
                            buildJsonObject {
                                put("type", "text")
                            }
                        )

                        put(
                            "id",
                            buildJsonObject {
                                put("type", "long")
                            }
                        )
                    }
                )
            }
        )
    },

    "charted-repo-members" to buildJsonObject {
        put(
            "settings",
            buildJsonObject {
                put("number_of_shards", 5)
            }
        )

        put(
            "mappings",
            buildJsonObject {
                put(
                    "properties",
                    buildJsonObject {
                        put(
                            "description",
                            buildJsonObject {
                                put("type", "text")
                                put("index", true)
                            }
                        )

                        put(
                            "updated_at",
                            buildJsonObject {
                                put("type", "date")
                            }
                        )

                        put(
                            "created_at",
                            buildJsonObject {
                                put("type", "date")
                            }
                        )

                        put(
                            "username",
                            buildJsonObject {
                                put("type", "text")
                                put("index", true)
                            }
                        )

                        put(
                            "email",
                            buildJsonObject {
                                put("type", "keyword")
                                put("index", true)
                            }
                        )

                        put(
                            "name",
                            buildJsonObject {
                                put("type", "text")
                                put("index", true)
                            }
                        )

                        put(
                            "id",
                            buildJsonObject {
                                put("type", "long")
                                put("index", true)
                            }
                        )
                    }
                )
            }
        )
    }
)

val INDEX_MAPPINGS_WITHOUT_SETTINGS = mapOf(
    "charted-users" to buildJsonObject {
        put(
            "properties",
            buildJsonObject {
                put(
                    "description",
                    buildJsonObject {
                        put("type", "text")
                    }
                )

                put(
                    "username",
                    buildJsonObject {
                        put("type", "text")
                    }
                )

                put(
                    "email",
                    buildJsonObject {
                        put("type", "keyword")
                    }
                )

                put(
                    "name",
                    buildJsonObject {
                        put("type", "text")
                    }
                )
            }
        )
    },

    "charted-repos" to buildJsonObject {
        put(
            "properties",
            buildJsonObject {
                put(
                    "description",
                    buildJsonObject {
                        put("type", "text")
                    }
                )

                put(
                    "updated_at",
                    buildJsonObject {
                        put("type", "date")
                    }
                )

                put(
                    "created_at",
                    buildJsonObject {
                        put("type", "date")
                    }
                )

                put(
                    "owner_id",
                    buildJsonObject {
                        put("type", "long")
                    }
                )

                put(
                    "name",
                    buildJsonObject {
                        put("type", "text")
                    }
                )

                put(
                    "type",
                    buildJsonObject {
                        put("type", "keyword")
                    }
                )
            }
        )
    },

    "charted-orgs" to buildJsonObject {
        put(
            "properties",
            buildJsonObject {
                put(
                    "verified_publisher",
                    buildJsonObject {
                        put("type", "boolean")
                    }
                )

                put(
                    "description",
                    buildJsonObject {
                        put("type", "text")
                        put("index", true)
                    }
                )

                put(
                    "updated_at",
                    buildJsonObject {
                        put("type", "date")
                    }
                )

                put(
                    "created_at",
                    buildJsonObject {
                        put("type", "date")
                    }
                )

                put(
                    "handle",
                    buildJsonObject {
                        put("type", "keyword")
                        put("index", true)
                    }
                )

                put(
                    "name",
                    buildJsonObject {
                        put("type", "text")
                        put("index", true)
                    }
                )
            }
        )
    },

    "charted-org-members" to buildJsonObject {
        put(
            "properties",
            buildJsonObject {
                put(
                    "description",
                    buildJsonObject {
                        put("type", "text")
                        put("index", true)
                    }
                )

                put(
                    "updated_at",
                    buildJsonObject {
                        put("type", "date")
                    }
                )

                put(
                    "joined_at",
                    buildJsonObject {
                        put("type", "date")
                    }
                )

                put(
                    "username",
                    buildJsonObject {
                        put("type", "text")
                    }
                )

                put(
                    "email",
                    buildJsonObject {
                        put("type", "keyword")
                    }
                )

                put(
                    "name",
                    buildJsonObject {
                        put("type", "text")
                    }
                )
            }
        )
    },

    "charted-repo-members" to buildJsonObject {
        put(
            "properties",
            buildJsonObject {
                put(
                    "description",
                    buildJsonObject {
                        put("type", "text")
                        put("index", true)
                    }
                )

                put(
                    "updated_at",
                    buildJsonObject {
                        put("type", "date")
                    }
                )

                put(
                    "created_at",
                    buildJsonObject {
                        put("type", "date")
                    }
                )

                put(
                    "username",
                    buildJsonObject {
                        put("type", "text")
                        put("index", true)
                    }
                )

                put(
                    "email",
                    buildJsonObject {
                        put("type", "keyword")
                        put("index", true)
                    }
                )

                put(
                    "name",
                    buildJsonObject {
                        put("type", "text")
                        put("index", true)
                    }
                )
            }
        )
    },
)
