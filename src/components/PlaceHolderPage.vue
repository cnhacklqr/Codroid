<script setup lang="ts">
const props = defineProps({
  reason: {
    type: String,
    default: "",
  },
  description: {
    type: String,
    default: "here is a kiwi bird instead",
  },
});
</script>

<template>
  <var-space
    align="center"
    justify="center"
    direction="column"
    size="large"
    class="container"
  >
    <p class="placeholder">{{ props.reason }}</p>
    <font-awesome-icon icon="fa-solid fa-kiwi-bird" class="kiwi-bird" />
    <p class="placeholder">{{ props.description }}</p>
  </var-space>
</template>

<style scoped>
.container {
  height: 80vh;
}

.kiwi-bird {
  font-size: 10rem;
  color: gray;
}

.placeholder {
  font-size: 1.1rem;
  color: gray;
}
</style>
