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

package org.noelware.charted.gradle.plugins.docker;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import javax.inject.Inject;
import org.gradle.api.model.ObjectFactory;
import org.gradle.api.provider.Property;

public class DockerExtension {
    private final ArrayList<Dockerfile> dockerfiles = new ArrayList<>();
    private final Property<String> minDockerVersion;

    @Inject
    public DockerExtension(ObjectFactory objectFactory) {
        this.minDockerVersion = objectFactory.property(String.class).convention(">=20.10");
    }

    public Property<String> getMinDockerVersion() {
        return minDockerVersion;
    }

    public List<Dockerfile> getDockerfiles() {
        return Collections.unmodifiableList(dockerfiles);
    }

    public void addDockerfile(Dockerfile dockerfile) {
        dockerfiles.add(dockerfile);
    }
}
