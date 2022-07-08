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

package org.noelware.charted.database.clickhouse.tests

import com.clickhouse.jdbc.JdbcConfig
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.slf4j.logging
import org.intellij.lang.annotations.Language
import org.noelware.charted.common.SetOnceGetValue
import org.testcontainers.containers.ClickHouseContainer
import org.testcontainers.containers.wait.strategy.HttpWaitStrategy
import org.testcontainers.utility.DockerImageName
import java.sql.ResultSet
import kotlin.test.AfterTest
import kotlin.test.BeforeTest

abstract class AbstractClickHouseTest {
    private val _container = SetOnceGetValue<ClickHouseContainer>()
    private val log by logging<AbstractClickHouseTest>()

    val container: ClickHouseContainer
        get() = _container.value

    private val hikariDataStore: HikariDataSource
        get() = HikariDataSource(
            HikariConfig().apply {
                jdbcUrl = container.jdbcUrl
                driverClassName = "com.clickhouse.jdbc.ClickHouseDriver"

                addDataSourceProperty(JdbcConfig.PROP_WRAPPER_OBJ, "true")
            }
        )

    fun sql(@Language("sql") sql: String): ResultSet? {
        if (!_container.wasSet()) {
            throw IllegalStateException("#startContainer() was never called, can't query SQL [$sql]")
        }

        val stmt = hikariDataStore.connection.createStatement()
        stmt.execute(sql)

        val set = stmt.resultSet
        if (!set.next()) return null

        return set
    }

    @BeforeTest
    fun startContainer() {
        log.info("Starting ClickHouse container!")

        val image = DockerImageName
            .parse("clickhouse/clickhouse-server:22.6.2.12-alpine")
            .asCompatibleSubstituteFor("yandex/clickhouse-server")

        _container.value = ClickHouseContainer(image)
        _container.value.setWaitStrategy(HttpWaitStrategy().forPort(8123))
        container.start()
    }

    @AfterTest
    fun destroyContainer() {
        if (!_container.wasSet()) {
            throw IllegalStateException("#startContainer() was never called, can't destroy nothing!")
        }

        log.warn("Destroying container...")
        container.stop()
    }
}
