<script setup lang="ts">
import { ref, onMounted, onUnmounted, inject } from "vue";
import { useRouter } from "vue-router";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

const appBarTitle = inject("appBarTitle") as (arg: string) => void;
appBarTitle("Home");

const router = useRouter();
const setupProcess = ref(["Waiting until setup process complete..."]);
const setupCompleted = ref(false);

let unlisten: Promise<UnlistenFn> | null = null;
let setupProcessValue = 0;

onMounted(async () => {
  interface Payload {
    message: string;
  }

  unlisten = listen<Payload>("setup-process", (event) => {
    const { message } = event.payload;
    if (setupProcess.value.length >= 3) {
      setupProcess.value.shift();
    }
    setupProcess.value.push(message);
    setupProcessValue += 100 / 3;
  });

  invoke("init_resources").then(() => {
    setupCompleted.value = true;
  });
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
            subtitle="Manage Projects"
            prepend-icon="mdi-code-braces"
            rounded="pill"
          >
          </v-list-item>
        </template>

        <v-list-item
          title="Create new project"
          prepend-icon="mdi-creation"
          rounded="pill"
          :disabled="!setupCompleted"
          @click="routeToCreateProjectView"
        >
        </v-list-item>

        <v-list-item
          title="Open Project"
          prepend-icon="mdi-folder"
          rounded="pill"
          :disabled="!setupCompleted"
          @click="routeToOpenProjectView"
        >
        </v-list-item>
      </v-list-group>

      <v-list-item
        title="Settings"
        subtitle="Configure Codroid"
        prepend-icon="mdi-cogs"
        rounded="pill"
        :disabled="!setupCompleted"
        @click="routeToSettingsView"
      >
      </v-list-item>

      <v-list-item
        title="About"
        subtitle="Author, licenses, and etc."
        prepend-icon="mdi-information-outline"
        rounded="pill"
        @click="routeToAboutView"
      >
      </v-list-item>
    </v-list>

    <v-expansion-panels style="width: 75%; margin: auto">
      <v-expansion-panel title="Setup Process Details" :elevation="0">
        <v-expansion-panel-text>
          <v-progress-linear
            v-model="setupProcessValue"
            stream
            color="black"
          ></v-progress-linear>
          <div
            v-for="(message, index) in setupProcess"
            :key="index"
            class="setupProcess"
          >
            {{ message }}
          </div>
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>
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
  text-align: center;
}
</style>
