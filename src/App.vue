<script setup lang="ts">
import { provide, ref } from "vue";
import { exit } from "@tauri-apps/plugin-process";
import { useRouter } from "vue-router";

const router = useRouter();

const appBarTitle = ref("");
provide("appBarTitle", (newTitle: string) => (appBarTitle.value = newTitle));

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
    <v-app-bar prominent>
      <v-app-bar-nav-icon
        @click.stop="showSidebar = !showSidebar"
      ></v-app-bar-nav-icon>

      <v-app-bar-title>{{ appBarTitle }}</v-app-bar-title>
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
