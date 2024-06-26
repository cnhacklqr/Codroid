<script setup lang="ts">
import { computed, ComputedRef, onMounted, ref, Ref } from "vue";
import ProjectCard from "./components/ProjectCard.vue";
import { useRouter } from "vue-router";
import { useAppGlobal } from "./stores/appGlobal";
import { commands, ProjectInfo, Template } from "./bindings";

const router = useRouter();
const appGlobal = useAppGlobal();

onMounted(() => (appGlobal.appBartitle = "CreateProject"));

// stepper
const step = ref(0);
const stepPrefixed = computed(() => step.value + 1);

const next = () => (step.value += 1);
const prev = () => {
  if (stepPrefixed.value === 1) {
    router.replace("/");
  } else {
    step.value -= 1;
  }
};

const templateAutocomplete: Array<Template> = [
  "empty",
  "rustBinary",
  "rustLibrary",
];

const projectName = ref("");
const projectTemplateInput: Ref<Template> = ref("empty");
const projectInfo: ComputedRef<ProjectInfo> = computed(() => {
  return {
    name: projectName.value,
    template: projectTemplateInput.value,
  };
});

// stepper
const finishedList: Record<number, ComputedRef<boolean>> = {
  [1]: computed(() =>
    (templateAutocomplete as string[]).includes(projectTemplateInput.value),
  ),
  [2]: computed(() => projectName.value !== ""),
  [3]: computed(() => false),
};

const stepperActionDisabled = computed(() => {
  const disableNext = !finishedList[stepPrefixed.value].value;
  if (disableNext) {
    return "next";
  } else {
    return undefined;
  }
});

const confirmCreation = () => {
  // todo: 调用后端创建项目，然后导航到项目编辑器
  commands.projectManagerCreateProject(projectInfo.value);
};
</script>

<template>
  <v-stepper v-model="step" alt-labels class="view">
    <v-stepper-header>
      <v-stepper-item
        title="Select Project Template"
        value="1"
        :complete="finishedList[1].value"
      ></v-stepper-item>

      <v-divider></v-divider>

      <v-stepper-item
        title="Configure Project"
        value="2"
        :complete="finishedList[2].value"
      ></v-stepper-item>

      <v-divider></v-divider>

      <v-stepper-item
        title="Confirm Project Information"
        value="3"
        :complete="finishedList[3].value"
      ></v-stepper-item>
    </v-stepper-header>

    <v-stepper-window>
      <v-stepper-window-item value="1">
        <v-sheet
          ><v-autocomplete
            v-model:model-value="projectTemplateInput"
            class="textFeild"
            auto-select-first
            label="Template"
            :items="templateAutocomplete"
          ></v-autocomplete
        ></v-sheet>
      </v-stepper-window-item>

      <v-stepper-window-item value="2">
        <v-sheet
          ><v-text-field
            v-model:model-value="projectName"
            class="textFeild"
            label="Project Name"
          ></v-text-field
        ></v-sheet>
      </v-stepper-window-item>

      <v-stepper-window-item value="3">
        <v-sheet>
          <ProjectCard :info="projectInfo" class="mb-4" />
          <v-btn color="primary" @click="confirmCreation">CREATE</v-btn>
        </v-sheet>
      </v-stepper-window-item>
    </v-stepper-window>

    <v-stepper-actions
      :disabled="stepperActionDisabled"
      @click:next="next"
      @click:prev="prev"
    >
    </v-stepper-actions>
  </v-stepper>
</template>

<style scoped>
.view {
  height: 100%;
  width: 100%;
}

.textFeild {
  padding-top: 25px;
}
</style>
