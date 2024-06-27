<script setup lang="ts">
import { ref } from "vue";
import { exit } from "@tauri-apps/plugin-process";
import { useRouter } from "vue-router";
import { useAppGlobal } from "./stores/appGlobal";

const router = useRouter();
const appGlobal = useAppGlobal();

const showSidebar = ref(false);

const exitApp = async () => {
  await exit(0);
};

const routeToHomeView = () => {
  router.push("/");
};
</script>

<template>
  <v-app class="rounded rounded-md">
    <v-app-bar :elevation="0">
      <v-app-bar-nav-icon
        @click.stop="showSidebar = !showSidebar"
      ></v-app-bar-nav-icon>

      <v-app-bar-title>{{ appGlobal.appBartitle }}</v-app-bar-title>
    </v-app-bar>

    <v-navigation-drawer v-model="showSidebar">
      <v-list>
        <v-list-item
          title="Home"
          prepend-icon="mdi-home"
          rounded="xl"
          @click="routeToHomeView"
        >
        </v-list-item>

        <v-list-item
          title="Exit"
          prepend-icon="mdi-exit-to-app"
          rounded="xl"
          @click="exitApp"
        >
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main
      ><router-view v-slot="{ Component }">
        <transition name="fade">
          <component :is="Component" />
        </transition> </router-view
    ></v-main>
  </v-app>
</template>

<style>
* {
  user-select: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  -ms-user-select: none;
  -khtml-user-select: none;
}
</style>
