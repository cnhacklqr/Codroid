<script setup lang="ts">
import { provide, ref } from "vue";

const appBarTitle = ref("");
const appBarSubTitle = ref("");

provide("appBarTitle", appBarTitle.value);
provide("appBarSubTitle", appBarSubTitle.value);
</script>

<template>
  <v-app class="rounded rounded-md">
    <v-app-bar>
      <template #prepend>
        <v-app-bar-nav-icon></v-app-bar-nav-icon>
      </template>
    </v-app-bar>

    <v-main
      ><router-view v-slot="{ Component }">
        <transition name="fade">
          <component :is="Component" />
        </transition> </router-view
    ></v-main>
  </v-app>
</template>
