package org.noelware.charted.gradle.plugins.golang;

import org.gradle.api.Action;
import org.gradle.process.ExecResult;
import org.gradle.process.ExecSpec;

/**
 * Represents an interface that has the #exec method from the {@link org.gradle.process.ExecOperations exec operations interface} or
 * a {@link org.gradle.api.Project Gradle project}.
 */
public interface ExecOperationsLike {
    ExecResult exec(Action<? super ExecSpec> action);
}
