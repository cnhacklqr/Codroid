<script setup lang="ts">
import { computed, ComputedRef, inject, ref } from "vue";
import ProjectCard from "./components/ProjectCard.vue";
import { Template } from "./type";
import { useRouter } from "vue-router";
import { ProjectInfo } from "./type";
import { invoke } from "@tauri-apps/api/core";

const appBarTitle = inject("appBarTitle") as (arg: string) => void;
appBarTitle("Create New Project");

const router = useRouter();

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

const templateAutocomplete = Object.values(Template).filter(
  (value) => typeof value === "string",
);

const projectName = ref("");
const projectTemplateInput = ref("");
const projectInfo: ComputedRef<ProjectInfo> = computed(() => {
  const template =
    Template[projectTemplateInput.value as keyof typeof Template];
  return {
    name: projectName.value,
    template,
  };
});

// stepper
const finishedList: Record<number, ComputedRef<boolean>> = {
  [1]: computed(() =>
    templateAutocomplete.includes(projectTemplateInput.value),
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
  invoke("project_manager_create_project", {
    projectInfo: projectInfo.value,
  });
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
