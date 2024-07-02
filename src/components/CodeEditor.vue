<script setup lang="ts">
import { ref } from "vue";
import CodeMirror from "vue-codemirror6";
import { tomorrow } from "./codemirrorTheme";

const value = ref("");

const extensions = [tomorrow];
</script>

<template>
  <code-mirror v-model="value" basic :extensions="extensions" />
</template>
