package org.noelware.charted.engine.charts.tests

import org.noelware.charted.engine.charts.ChartsEngine
import org.noelware.charted.engine.charts.DefaultChartsEngine

class ChartEngineTests {
    private val engine: ChartsEngine = DefaultChartsEngine(MockStorageWrapper())
}
