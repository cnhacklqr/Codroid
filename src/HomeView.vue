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
        prepend-icon="mdi-code-braces"
        append-icon="mdi-menu-right"
        @click="routeToProjectView"
      >
      </v-list-item>

      <v-list-item
        title="Settings"
        subtitle="configure Codroid"
        prepend-icon="mdi-cogs"
        append-icon="mdi-menu-right"
      >
      </v-list-item>

      <v-list-item
        title="About"
        subtitle="author, licenses, etc."
        prepend-icon="mdi-information-outline"
        append-icon="mdi-menu-right"
      >
      </v-list-item>
    </v-list>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  padding: 100px;
}

.titleIcon {
  margin: auto;
  font-size: 130px;
}

.title {
  margin: auto;
  font-size: 30px;
  padding-bottom: 25px;
}
</style>
