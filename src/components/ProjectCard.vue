<script setup lang="ts">
import { computed } from "vue";
import RustChip from "./chips/RustChip.vue";
import EmptyChip from "./chips/EmptyChip.vue";
import { ProjectInfo } from "../bindings";

const props = defineProps<{
  info: ProjectInfo;
}>();

const color = computed(() => {
  switch (props.info.template) {
    case "empty":
    default:
      return "grey-lighten-2";
    case "rustBinary":
    case "rustLibrary":
      return "brown-lighten-2";
  }
});

const chips = computed(() => {
  switch (props.info.template) {
    case "empty":
      return [EmptyChip];
    case "rustBinary":
    case "rustLibrary":
      return [RustChip];
    default:
      return [];
  }
});
</script>

<template>
  <v-card variant="flat" :color="color" hover
    ><v-card-title>Project Name: {{ info.name }}</v-card-title>

    <v-card-item>
      <v-chip-group v-for="(chip, index) in chips" :key="index" draggable>
        <component :is="chip"></component>
      </v-chip-group>
    </v-card-item>
  </v-card>
</template>
