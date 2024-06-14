<script setup lang="ts">
import { computed, ComputedRef, inject, Ref, ref } from "vue";
import RustChip from "./components/chips/RustChip.vue";
import EmptyChip from "./components/chips/EmptyChip.vue";

const appBarTitle = inject("appBarTitle") as (arg: string) => void;
appBarTitle("Create New Project");

// stepper
const step = ref(0);
const stepPrefixed = computed(() => step.value + 1);

const next = () => {
  step.value += 1;
  if (stepPrefixed.value === 3) {
    const max = projectCardColors.length - 1;
    const randFloat = Math.random() * (max + 1);
    const randIndex = Math.floor(randFloat);

    projectCardColor.value = projectCardColors[randIndex];
  }
};
const prev = () => (step.value -= 1);

// project info
enum TemplateId {
  RustBinary,
  RustLibrary,
  Empty,
}

const templates: Record<string, { id: TemplateId; icon: string }> = {
  ["Rust Binary"]: { id: TemplateId.RustBinary, icon: "fa:fab fa-rust" },
  ["Rust Library"]: { id: TemplateId.RustLibrary, icon: "fa:fab fa-rust" },
  ["Empty"]: { id: TemplateId.Empty, icon: "mdi-null" },
};

const templateAutocomplete: Array<string> = Object.keys(templates);

const projectName = ref("");
const projectTemplateInput = ref("");
const projectTemplateType = computed(() => {
  const input = projectTemplateInput.value;
  return templates[input].id;
});

// chips
const showRustChip = computed(() => {
  switch (projectTemplateType.value) {
    case TemplateId.RustBinary:
    case TemplateId.RustLibrary:
      return true;
    default:
      return false;
  }
});

const showEmptyChip = computed(() => {
  switch (projectTemplateType.value) {
    case TemplateId.Empty:
      return true;
    default:
      return false;
  }
});

// project card
const projectCardColors = [
  "light-blue-lighten-3",
  "teal-lighten-3",
  "indigo-lighten-3",
  "red-lighten-3",
  "purple-lighten-3",
  "light-green-lighten-3",
  "blue-grey-lighten-3",
];

const projectCardColor: Ref<string | undefined> = ref(undefined);

// stepper
const finishedList: Record<number, ComputedRef<boolean>> = {
  [1]: computed(() =>
    templateAutocomplete.includes(projectTemplateInput.value),
  ),
  [2]: computed(() => projectName.value !== ""),
  [3]: computed(() => true),
};

const stepperActionDisabled = computed(() => {
  const disableNext =
    stepPrefixed.value === 3 || !finishedList[stepPrefixed.value].value;
  const disablePrev = stepPrefixed.value === 1;

  if (disableNext && disablePrev) {
    return true;
  } else if (disablePrev) {
    return "prev";
  } else if (disableNext) {
    return "next";
  } else {
    return undefined;
  }
});
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
        title="Finish"
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
            auto-select-first="true"
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
        <v-card variant="flat" :color="projectCardColor" hover
          ><v-card-title>Project {{ projectName }}</v-card-title>

          <v-card-item>
            <RustChip v-if="showRustChip" />
            <EmptyChip v-if="showEmptyChip" />
          </v-card-item>
        </v-card>
      </v-stepper-window-item>
    </v-stepper-window>

    <v-stepper-actions
      :disabled="stepperActionDisabled"
      @click:next="next"
      @click:prev="prev"
    >
      <v-btn>Click me</v-btn>
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
