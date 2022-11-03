package org.noelware.charted.gradle.plugins.scoop;

import org.gradle.api.DefaultTask;

public class GenerateScoopBucket extends DefaultTask {
    public final String TEMPLATE = """
    {
        "version": "%s",
        "homepage": "https://charts.noelware.org",
        "license": "Apache-2.0",
        "description": "You know, for Helm Charts?",
        "architecture": {
            "64bit": {
                "url": "https://artifacts.noelware.cloud/download/charted/server/v%s/charted-server.zip",
                "bin": ["charted-server.ps1"],
                "hash": "%s"    
            }
        }
    }        
    """;
}
