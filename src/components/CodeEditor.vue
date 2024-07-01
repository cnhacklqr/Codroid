<script setup lang="ts">
import { ref } from "vue";
import CodeMirror from "vue-codemirror6";

const value = ref("");
</script>

<template>
  <code-mirror v-model="value" basic />
</template>
