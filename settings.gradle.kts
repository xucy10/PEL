pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "ZalithLauncher2"

include(":ZalithLauncher")
include(":ColorPicker")
include(":LayerController")
include(":Terracotta")
include(":LWJGL")