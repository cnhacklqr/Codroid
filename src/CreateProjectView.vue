<script setup lang="ts">
import { computed, inject, ref } from "vue";

const appBarTitle = inject("appBarTitle") as (arg: string) => void;
appBarTitle("Create New Project");

// stepper
const step = ref(0);
const stepPrefixed = computed(() => step.value + 1);

const next = () => (step.value += 1);
const prev = () => (step.value -= 1);

// project info
const projectName = ref("");
</script>

<template>
  <v-stepper v-model="step" alt-labels class="view">
    <v-stepper-header>
      <v-stepper-item
        title="Select Project Template"
        value="1"
        :complete="1 < stepPrefixed"
      ></v-stepper-item>

      <v-divider></v-divider>

      <v-stepper-item
        title="Configure Project"
        value="2"
        :complete="2 < stepPrefixed"
      ></v-stepper-item>
    </v-stepper-header>

    <v-stepper-window>
      <v-stepper-window-item value="1">
        <div class="text-subtitle">Project name</div>

        <v-sheet
          ><v-text-field
            v-model:model-value="projectName"
            class="textFeild"
            label="name"
          ></v-text-field
        ></v-sheet>
      </v-stepper-window-item>

      <v-stepper-window-item value="2">
        <div class="text-subtitle">Project name</div>

        <v-sheet
          ><v-text-field
            v-model:model-value="projectName"
            class="textFeild"
            label="name"
          ></v-text-field
        ></v-sheet>
      </v-stepper-window-item>
    </v-stepper-window>

    <v-stepper-actions @click:next="next" @click:prev="prev">
    </v-stepper-actions>
  </v-stepper>
</template>

<style scoped>
.view {
  height: 100%;
  width: 100%;
  position: fixed;
}

.textFeild {
  padding-top: 25px;
}
</style>
