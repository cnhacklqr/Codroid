<script setup lang="ts">
import "@mdui/icons/android.js";
import "@mdui/icons/add.js";
import "@mdui/icons/settings.js";
import "@mdui/icons/info.js";
import { useRouter } from "vue-router";
import { VList, VListItem } from "vuetify/components";

const router = useRouter();

function routeToProjectView() {
  router.push("/project");
}
</script>

<template>
  <div class="container">
    <mdui-icon-android class="titleIcon" />
    <h1 class="title">Codroid</h1>

    <v-list>
      <v-list-item
        title="Open Project"
        subtitle="...and start coding"
        @click="routeToProjectView"
      >
        <template #prepend>
          <mdui-icon-add />
        </template>
      </v-list-item>

      <v-list-item title="Settings" subtitle="configure Codroid">
        <template #prepend>
          <mdui-icon-settings />
        </template>
      </v-list-item>

      <v-list-item title="About" subtitle="author, licenses, etc.">
        <template #prepend>
          <mdui-icon-info />
        </template>
      </v-list-item>
    </v-list>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
}

.titleIcon {
  margin: auto;
  font-size: 130px;
}

.title {
  margin: auto;
  font-size: 30px;
}
</style>
