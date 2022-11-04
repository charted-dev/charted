package org.noelware.charted.gradle.plugins.golang;

import de.undercouch.gradle.tasks.download.Download;
import de.undercouch.gradle.tasks.download.DownloadAction;
import de.undercouch.gradle.tasks.download.DownloadExtension;
import de.undercouch.gradle.tasks.download.DownloadTaskPlugin;
import groovy.lang.Closure;
import org.gradle.api.*;
import org.gradle.api.plugins.ExtensionAware;
import org.gradle.api.plugins.ExtraPropertiesExtension;
import org.gradle.api.plugins.PluginAware;
import org.gradle.api.tasks.Copy;

import java.io.File;
import java.io.IOException;
import java.util.Set;

public class GolangPlugin implements Plugin<Project> {
    public static final String GOLANG_PATH_EXT = "golang.path";

    @Override
    public void apply(Project project) {
        final GolangExtension ext = project.getExtensions().create("golang", GolangExtension.class, project);
        project.getPlugins().apply(DownloadTaskPlugin.class);

//        final DownloadExtension dlSpec = project.getExtensions().getByType(DownloadExtension.class);
//        if (!ext.getUseLocalInstall().getOrElse(false)) {
//            final String downloadUrl = ext.getDownloadUrl().get();
//            project.getLogger().info("Downloading Go from [" + downloadUrl + "]...");
//
//            final DownloadAction action = new DownloadAction(project);
//            try {
//                action.src(downloadUrl);
//                action.dest(new File(project.getRootProject().getBuildDir(), "golang.tar.gz"));
//                action.execute();
//            } catch (IOException e) {
//                throw new GradleException("Unable to download Go from URL [" + downloadUrl + "]:", e);
//            }
//
//            // Unzip file contents
//            final Copy unzipFile = project.getTasks().create("downloadAndUnzip", Copy.class, (spec) -> {
//                spec.from(project.tarTree(action.getDest()));
//                spec.into(new File(project.getRootProject().getBuildDir(), "golang"));
//            });
//
//            // probably a hacky way to execute this lol
//            // (gradle pls make this easier ;-;)
//            ((Copy)project.getTasks().getByName("downloadAndUnzip")).getTaskActions().forEach(c -> c.execute((Copy)project.getTasks().getByName("downloadAndUnzip")));
//
////            final ExtraPropertiesExtension extraExt = ((ExtensionAware)project).getExtensions().getExtraProperties();
////            extraExt.set(GOLANG_PATH_EXT, new File(project.getRootProject().getBuildDir(), "golang/bin").toString());
//        } else {
//            final ExtraPropertiesExtension extraExt = ((ExtensionAware)project).getExtensions().getExtraProperties();
//            extraExt.set(GOLANG_PATH_EXT, "go");
//        }
    }
}
