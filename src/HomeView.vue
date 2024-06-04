<script setup lang="ts">
import "@mdui/icons/android.js";
import "mdui/components/list.js";
import "mdui/components/list-item.js";
import "mdui/components/list-subheader.js";
import "@mdui/icons/add.js";
import "@mdui/icons/settings.js";
import "@mdui/icons/info.js";
import { useRouter } from "vue-router";

const router = useRouter();

function routeToProjectView() {
  router.push("/project");
}
</script>

<template>
  <div class="container">
    <mdui-icon-android class="titleIcon" />
    <h1 class="title">Codroid</h1>

    <mdui-list>
      <mdui-list-item
        headline="Open Project"
        description="...and start coding"
        @click="routeToProjectView"
      >
        <span slot="icon"><mdui-icon-add /></span>
      </mdui-list-item>

      <mdui-list-item headline="Settings" description="configure Codroid">
        <span slot="icon"><mdui-icon-settings /></span>
      </mdui-list-item>

      <mdui-list-item headline="About" description="author, licenses, etc.">
        <span slot="icon"><mdui-icon-info /></span>
      </mdui-list-item>
    </mdui-list>
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
