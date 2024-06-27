<script setup lang="ts">
import { ref, onMounted, onUnmounted, Ref, computed } from "vue";
import { useRouter } from "vue-router";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import Spacer from "./components/layout/Spacer.vue";
import { useAppGlobal } from "./stores/appGlobal";

const appGlobal = useAppGlobal();
const router = useRouter();

const setupProcessText = ref("");
const setupProcessStep = ref(0);
const setupProcessStepMax: Ref<number | null> = ref(null);
const setupProgressPercent = computed(() => {
  if (setupProcessStepMax.value === null) {
    return 0;
  } else {
    return (
      (setupProcessStep.value / setupProcessStepMax.value!) *
      100
    ).toFixed(2);
  }
});
const showSetupProgess = computed(
  () => setupProcessStepMax.value !== null && !setupCompleted.value
);
const showSetupProcessText = computed(() => !setupCompleted.value);
const setupCompleted = ref(false);

let unlisten: Promise<UnlistenFn> | null = null;

onMounted(async () => {
  appGlobal.appBartitle = "Home";

  interface SetupProcess {
    currentStep: number;
    maxStep: number;
    message: string;
  }

  unlisten = listen<SetupProcess>("setup-process", (event) => {
    const payload = event.payload;

    setupProcessStep.value = payload.currentStep;
    setupProcessStepMax.value = payload.maxStep;
    setupProcessText.value = payload.message;
  });

  invoke("init").then(() => {
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
  router.push("openProject");
};

const routeToSettingsView = () => {
  router.push("settings");
};

const routeToAboutView = () => {
  router.push("about");
};
</script>

<template>
  <v-container class="container">
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

    <Spacer />

    <v-progress-linear
      v-if="showSetupProgess"
      :model-value="setupProgressPercent"
      class="setupProcessLinear"
      stream
      color="grey-darken-1"
    ></v-progress-linear>

    <div
      v-if="showSetupProcessText"
      class="setupProcessText text-grey-lighten-1"
    >
      {{ setupProcessText }} {{ setupProgressPercent }}%
    </div>
  </v-container>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  width: 80%;
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
  width: 100%;
}

.setupProcessLinear {
  margin: auto;
}

.setupProcessText {
  margin: auto;
  font-size: small;
  white-space: pre-wrap;
  text-align: center;
}
</style>
