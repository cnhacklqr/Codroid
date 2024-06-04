<script setup lang="ts">
import "mdui/components/layout.js";
import "mdui/components/layout-item.js";
import "mdui/components/layout-main.js";
import "mdui/components/top-app-bar.js";
import "mdui/components/top-app-bar-title.js";
import "mdui/components/navigation-rail.js";
import "mdui/components/navigation-rail-item.js";
import "@mdui/icons/file-open.js";
</script>

<template>
  <mdui-layout>
    <mdui-top-app-bar class="example-top-app-bar">
      <mdui-top-app-bar-title>Top App Bar</mdui-top-app-bar-title>
    </mdui-top-app-bar>
    <mdui-navigation-rail value="Files">
      <mdui-navigation-rail-item alue="Files">
        Files
        <span slot="icon"><mdui-icon-file-open /></span>
      </mdui-navigation-rail-item>
    </mdui-navigation-rail>
    <mdui-layout-main class="example-layout-main" style="min-height: 300px">
      Main
    </mdui-layout-main>
  </mdui-layout>
</template>
