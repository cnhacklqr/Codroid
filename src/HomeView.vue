<script setup lang="ts">
import { ref, onMounted, Ref, computed } from "vue";
import { useRouter } from "vue-router";
import { useAppGlobal } from "./stores/appGlobal";
import { commands, events } from "./bindings";

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
const doningSetup = ref(true);

onMounted(async () => {
  appGlobal.appBartitle = "Home";

  events.setupProcess.listen((event) => {
    const payload = event.payload;

    setupProcessStep.value = payload.currentStep;
    setupProcessStepMax.value = payload.maxStep;
    setupProcessText.value = payload.message;
  });

  commands.init().then(() => (doningSetup.value = false));
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
  <var-space direction="column" align="center" justify="center">
    <font-awesome-icon icon="fa-solid fa-code" size="5x" />
    <h1 class="title">Codroid</h1>

    <var-loading
      :loading="doningSetup"
      :description="setupProgressPercent.toString()"
      style="width: 500px"
    >
      <var-paper>
        <var-menu same-width>
          <var-cell title="Project" description="Open / Create A Project">
            <template #icon>
              <font-awesome-icon
                icon="fa-solid fa-file-code"
                class="code-icon"
              />
            </template>
          </var-cell>

          <template #menu>
            <var-cell @click="routeToOpenProjectView">Open</var-cell>
            <var-cell @click="routeToCreateProjectView">Create</var-cell>
          </template>
        </var-menu>

        <var-cell
          title="Settings"
          description="Configure Codroid"
          @click="routeToSettingsView"
        >
          <template #icon>
            <var-space>
              <font-awesome-icon icon="fa-solid fa-cog" class="icon" />
            </var-space>
          </template>
        </var-cell>

        <var-cell
          title="About"
          description="Author, licenses, and etc"
          @click="routeToAboutView"
        >
          <template #icon>
            <font-awesome-icon icon="fa-solid fa-circle-info" class="icon" />
          </template>
        </var-cell>
      </var-paper>
    </var-loading>
  </var-space>
</template>

<style scoped>
.menu {
  width: 200px;
}

.code-icon {
  margin-right: 15px;
  font-size: 1.5em;
  margin-left: 2px;
}

.icon {
  margin-right: 15px;
  font-size: 1.5em;
}
</style>
