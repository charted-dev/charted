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

package org.noelware.charted.database.tests

import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.slf4j.logging
import org.junit.jupiter.api.AfterAll
import org.junit.jupiter.api.BeforeAll
import org.testcontainers.containers.PostgreSQLContainer
import java.util.concurrent.atomic.AtomicBoolean

abstract class AbstractDatabaseTest {
    private val databaseStarted = AtomicBoolean(false)
    private lateinit var container: PostgreSQLContainer<*>
    private val log by logging<AbstractDatabaseTest>()

    fun getHikariDatasource(): HikariDataSource = HikariDataSource(
        HikariConfig().apply {
            driverClassName = "org.postgresql.Driver"
            jdbcUrl = container.jdbcUrl
            username = "charted"
            password = "charted"
        }
    )

    @BeforeAll
    fun initContainer() {
        if (databaseStarted.get()) {
            throw IllegalStateException("#initContainer() has been called more than once.")
        }

        log.info("Creating PostgreSQL container...")
        container = PostgreSQLContainer("postgres:14")
            .withUsername("charted")
            .withPassword("charted")
            .withDatabaseName("charted")

        container.start()
        databaseStarted.set(true)
    }

    @AfterAll
    fun destroyContainer() {
        if (!databaseStarted.get()) {
            throw java.lang.IllegalStateException("#initContainer() wasn't called at all.")
        }

        log.warn("Destroying container...")
        container.close()
    }
}
