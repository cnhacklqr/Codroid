<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useRouter } from "vue-router";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

interface Payload {
  message: string;
}

let unlisten: Promise<UnlistenFn> | null = null;

const router = useRouter();
const setupProcess = ref(["Waiting until setup process complete..."]);
const setupCompleted = ref(false);

const init_resources = async () => {
  await invoke("init_resources").then(() => {
    setupCompleted.value = true;
  });
};

onMounted(async () => {
  unlisten = listen<Payload>("setup-process", (event) => {
    const { message } = event.payload;
    if (setupProcess.value.length >= 3) {
      setupProcess.value.shift();
    }
    setupProcess.value.push(message);
  });

  init_resources();
  await unlisten;
});

onUnmounted(() => {
  unlisten?.then((unlisten) => unlisten());
});

const routeToCreateProjectView = () => {
  router.push("createProject");
};

const routeToOpenProjectView = () => {
  router.push("openProjectView");
};

const routeToSettingsView = () => {
  router.push("settings");
};

const routeToAboutView = () => {
  router.push("about");
};
</script>

<template>
  <div class="container">
    <v-icon icon="mdi-android-studio" class="titleIcon"></v-icon>
    <h1 class="title">Codroid</h1>

    <v-list class="menu">
      <v-list-group>
        <template #activator="{ props }">
          <v-list-item
            v-bind="props"
            title="Project"
            prepend-icon="mdi-code-braces"
            append-icon="mdi-menu-right"
            rounded="xl"
          >
          </v-list-item>
        </template>

        <v-list-item
          title="Create new project"
          prepend-icon="mdi-creation"
          rounded="xl"
          :disabled="!setupCompleted"
          @click="routeToCreateProjectView"
        >
        </v-list-item>

        <v-list-item
          title="Open Project"
          prepend-icon="mdi-folder"
          rounded="xl"
          :disabled="!setupCompleted"
          @click="routeToOpenProjectView"
        >
        </v-list-item>
      </v-list-group>

      <v-list-item
        title="Settings"
        subtitle="configure Codroid"
        prepend-icon="mdi-cogs"
        append-icon="mdi-menu-right"
        rounded="xl"
        :disabled="!setupCompleted"
        @click="routeToSettingsView"
      >
      </v-list-item>

      <v-list-item
        title="About"
        subtitle="author, licenses, etc."
        prepend-icon="mdi-information-outline"
        append-icon="mdi-menu-right"
        rounded="xl"
        @click="routeToAboutView"
      >
      </v-list-item>
    </v-list>

    <div
      v-for="(message, index) in setupProcess"
      :key="index"
      class="setupProcess"
    >
      {{ message }}
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
}

.titleIcon {
  margin: auto;
  font-size: 130px;
  padding-top: 100px;
}

.title {
  margin: auto;
  font-size: 30px;
  padding-top: 75px;
}

.menu {
  margin: auto;
  padding-top: 25px;
  width: 80%;
}

.setupProcess {
  margin: auto;
  color: #00000057;
  font-size: small;
  white-space: pre-wrap;
}
</style>
