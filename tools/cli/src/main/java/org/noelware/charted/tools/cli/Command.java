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

package org.noelware.charted.tools.cli;

import java.util.ArrayList;

/**
 * Represents a CLI command that is executable
 */
public abstract class Command {
    private final ArrayList<Command> subcommands = new ArrayList<>();

    /**
     * Returns all the subcommands this parent command has.
     */
    public ArrayList<Command> getSubcommands() {
        return subcommands;
    }

    /**
     * Registers a subcommand to this parent command.
     * @param subcommand The subcommand
     * @param <T> The instance that extends the {@link Command command interface}.
     */
    public <T extends Command> void registerSubcommand(T subcommand) {
        subcommands.add(subcommand);
    }

    abstract void execute(String[] args) throws Exception;
}
