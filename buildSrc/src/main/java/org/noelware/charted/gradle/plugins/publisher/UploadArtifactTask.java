package org.noelware.charted.gradle.plugins.publisher;

import org.gradle.api.DefaultTask;
import org.gradle.work.DisableCachingByDefault;

@DisableCachingByDefault(because = "Not worth caching.")
public class UploadArtifactTask extends DefaultTask {}
