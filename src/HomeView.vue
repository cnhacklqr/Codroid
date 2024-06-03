<script setup lang="ts">
import '@mdui/icons/android.js';
import 'mdui/components/list.js';
import 'mdui/components/list-item.js';
import 'mdui/components/list-subheader.js';
import '@mdui/icons/add.js';
import '@mdui/icons/settings.js';
import '@mdui/icons/info.js';
import { useRouter } from 'vue-router';

const router = useRouter()

function routeToProjectView() {
  router.push("/project")
}
</script>

<template>
  <div class="container">
    <mdui-icon-android class="titleIcon"></mdui-icon-android>
    <h1 class="title">Codroid</h1>

    <mdui-list class="menu">
      <mdui-list-item headline="Open Project" description="...and start coding" @click="routeToProjectView">
        <template v-slot:icon>
<span ><mdui-icon-add></mdui-icon-add></span>
</template>
      </mdui-list-item>

      <mdui-list-item headline="Settings" description="configure codroid">
        <template v-slot:icon>
<span ><mdui-icon-settings></mdui-icon-settings></span>
</template>
      </mdui-list-item>

      <mdui-list-item headline="About" description="author, licenses, etc.">
        <template v-slot:icon>
<span ><mdui-icon-info></mdui-icon-info></span>
</template>
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
  padding-top: 25px;
}

.title {
  margin: auto;
  font-size: 30px;
  padding-top: 10px;
}

.menu {
  margin: auto;
  padding: 25px;
  padding-top: 80px;
}
</style>