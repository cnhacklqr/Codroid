<script setup lang="ts">
import { computed, ref } from "vue";

const step = ref(0);
const stepPrefixed = computed(() => step.value + 1);

const next = () => (step.value += 1);
const prev = () => (step.value -= 1);
</script>

<template>
  <v-stepper v-model="step" alt-labels class="view">
    <v-stepper-header>
      <v-stepper-item
        title="Name your project"
        value="1"
        :complete="1 < stepPrefixed"
      ></v-stepper-item>

      <v-divider></v-divider>

      <v-stepper-item
        title="Select Template"
        value="2"
        :complete="2 < stepPrefixed"
      ></v-stepper-item>
    </v-stepper-header>

    <v-stepper-window>
      <v-stepper-window-item value="1"></v-stepper-window-item>
    </v-stepper-window>

    <v-stepper-actions
      @click:next="next"
      @click:prev="prev"
    ></v-stepper-actions>
  </v-stepper>
</template>

<style scoped>
.view {
  height: 100%;
  width: 100%;
  position: fixed;
}
</style>
