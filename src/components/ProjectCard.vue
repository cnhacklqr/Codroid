<script setup lang="ts">
import { computed } from "vue";
import RustChip from "./chips/RustChip.vue";
import EmptyChip from "./chips/EmptyChip.vue";
import { ProjectInfo } from "../bindings";

const props = defineProps<{
  info: ProjectInfo;
}>();

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
  <var-card :title="info.name">
    <template #extra>
      <component
        :is="chip"
        v-for="(chip, index) in chips"
        :key="index"
      ></component>
    </template>
  </var-card>
</template>
