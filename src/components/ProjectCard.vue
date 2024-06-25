<script setup lang="ts">
import { computed } from "vue";
import RustChip from "./chips/RustChip.vue";
import EmptyChip from "./chips/EmptyChip.vue";
import { Template } from "../type";

const props = defineProps<{
  template: Template;
  projectName: string;
}>();

const color = computed(() => {
  switch (props.template) {
    case Template.Empty:
      return "grey-lighten-2";
    case Template.RustBinary:
    case Template.RustLibrary:
      return "brown-lighten-2";
    default:
      return "grey-lighten-2";
  }
});

const chips = computed(() => {
  switch (props.template) {
    case Template.Empty:
      return [EmptyChip];
    case Template.RustBinary:
    case Template.RustLibrary:
      return [RustChip];
    default:
      return [];
  }
});
</script>

<template>
  <v-card variant="flat" :color="color" hover
    ><v-card-title>Project Name: {{ projectName }}</v-card-title>

    <v-card-item>
      <v-chip-group v-for="(chip, index) in chips" :key="index">
        <component :is="chip"></component>
      </v-chip-group>
    </v-card-item>
  </v-card>
</template>
