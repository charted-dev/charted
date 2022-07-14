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

package org.noelware.charted.search.elasticsearch.tests

// import org.junit.Test
// import org.noelware.charted.common.data.ElasticsearchConfig
// import org.noelware.charted.search.elasticsearch.ElasticsearchClient
// import org.noelware.charted.testing.containers.AbstractElasticsearchContainerTest
// import kotlin.test.assertEquals
//
// class ElasticsearchTests: AbstractElasticsearchContainerTest() {
//    private val client: ElasticsearchClient
//        get() {
//            val nodes = listOf(getContainer().httpHostAddress)
//            return ElasticsearchClient(ElasticsearchConfig(nodes = nodes), indexDataWhenInitialized = false)
//        }
//
//    @Test
//    fun `can we add data`() {
//        client.connect()
//        assertEquals("8.3.0", client.serverVersion)
//    }
// }
