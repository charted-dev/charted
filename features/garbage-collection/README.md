# Garbage Collection

This is a feature that allows the API server to garbage-collect anything based off constraints that you configure.

This implements a simple query language using ANTLR4 to build constraint objects.

## Usage

To enable the GC feature, you will need to add the `gc` configuration property in the YAML format, or `garbageCollection {}` in the Kotlin DSL.

### YAML

```yaml
gc:
    inactive-repos:
        $object: Repository
        constraint: 'lastPublished >= timeSpan("30d")'
        actions: [delete]
    inactive-users:
        $object: User
        constraint: 'lastLoggedIn >= timeSpan("30d")'
        actions: [deactivate, 'email:deactivation']
    remove-big-images:
        $object: DockerImage
        constraint: 'imageSize >= byteSizeValue("1.5gb")'
        actions: [delete]
```

### Kotlin DSL

```kotlin
import org.noelware.charted.models.Repository
import org.noelware.charted.models.User
import org.noelware.charted.models.docker.Image
import org.noelware.charted.configuration.kotlin.dsl.gc.Actions

garbageCollection {
    action<Repository>("inactive-repos") {
        constraint {
            lastPublished >= "30d".toTimeSpan()
        }

        actions = listOf(Actions.Delete)
    }

    action<User>("inactive-users") {
        constraint {
            lastLoggedIn >= "30d".toTimeSpan()
        }

        actions = listOf(Actions.Deactivation, Actions.SendEmail("de-activiation"))
    }

    action<Image>("remove-big-images") {
        constraint {
            imageSize >= "1.5gb".toByteSizeValue()
        }

        actions = listOf(Actions.Delete)
        schedule {
            cron("* * 0 0 *")
        }
    }
}
```
