<script setup lang="ts">
import { computed } from "vue";

const { height, width } = defineProps({
  height: {
    type: String,
    default: "25px",
  },
  width: {
    type: String,
    default: "25px",
  },
});

const style = computed(() => {
  return {
    height: height,
    width: width,
  };
});
</script>

<template>
  <div :style="style"></div>
</template>
