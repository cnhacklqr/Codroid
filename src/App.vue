<script setup lang="ts">
import { exit } from "@tauri-apps/plugin-process";
import { useRouter } from "vue-router";
import { useAppGlobal } from "./stores/appGlobal";

const router = useRouter();
const appGlobal = useAppGlobal();

const exitApp = async () => {
  await exit(0);
};

const routeToHomeView = () => {
  router.push("/");
};
</script>

<template>
  <var-app-bar :title="appGlobal.appBartitle" round class="appBar">
    <template #right>
      <var-menu>
        <var-button color="transparent" text-color="#fff" round text>
          <var-icon name="menu" :size="24" />
        </var-button>

        <template #menu>
          <var-cell ripple icon="home" @click="routeToHomeView">Home</var-cell>
          <var-cell ripple icon="close-circle" @click="exitApp">Exit</var-cell>
        </template>
      </var-menu>
    </template>
  </var-app-bar>

  <router-view v-slot="{ Component }">
    <transition name="fade">
      <component :is="Component" />
    </transition>
  </router-view>
</template>

<style>
.appBar {
  margin-bottom: 5vh;
}

* {
  user-select: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  -ms-user-select: none;
  -khtml-user-select: none;
}

body {
  transition:
    background-color 0.25s,
    color 0.25s;
  color: var(--color-text);
  background-color: var(--color-body);
}
</style>
