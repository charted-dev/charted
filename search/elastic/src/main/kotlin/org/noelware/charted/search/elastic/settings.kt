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
    "charted_users" to buildJsonObject {
        put(
            "settings",
            buildJsonObject {
                put(
                    "analysis",
                    buildJsonObject {
                        put(
                            "analyzer",
                            buildJsonObject {
                                put(
                                    "ngram",
                                    buildJsonObject {
                                        put("tokenizer", "ngram")
                                    }
                                )
                            }
                        )
                    }
                )

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
                                put("analyzer", "ngram")
                                put("index", true)
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
    },

    "charted_repos" to buildJsonObject {
        put(
            "settings",
            buildJsonObject {
                put(
                    "analysis",
                    buildJsonObject {
                        put(
                            "analyzer",
                            buildJsonObject {
                                put(
                                    "ngram",
                                    buildJsonObject {
                                        put("tokenizer", "ngram")
                                    }
                                )
                            }
                        )
                    }
                )

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
                                put("analyzer", "ngram")
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

    "charted_orgs" to buildJsonObject {
        put(
            "settings",
            buildJsonObject {
                put(
                    "analysis",
                    buildJsonObject {
                        put(
                            "analyzer",
                            buildJsonObject {
                                put(
                                    "ngram",
                                    buildJsonObject {
                                        put("tokenizer", "ngram")
                                    }
                                )
                            }
                        )
                    }
                )

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
                                put("analyzer", "ngram")
                                put("index", true)
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

    "charted_org_members" to buildJsonObject {
        put(
            "settings",
            buildJsonObject {
                put(
                    "analysis",
                    buildJsonObject {
                        put(
                            "analyzer",
                            buildJsonObject {
                                put(
                                    "ngram",
                                    buildJsonObject {
                                        put("tokenizer", "ngram")
                                    }
                                )
                            }
                        )
                    }
                )

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
                                put("analyzer", "ngram")
                                put("index", true)
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
    },

    "charted_repo_members" to buildJsonObject {
        put(
            "settings",
            buildJsonObject {
                put(
                    "analysis",
                    buildJsonObject {
                        put(
                            "analyzer",
                            buildJsonObject {
                                put(
                                    "ngram",
                                    buildJsonObject {
                                        put("tokenizer", "ngram")
                                    }
                                )
                            }
                        )
                    }
                )

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
                                put("analyzer", "ngram")
                                put("index", true)
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
