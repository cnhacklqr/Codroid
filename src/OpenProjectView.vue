<script setup lang="ts">
import { onMounted, onUnmounted, ref, Ref } from "vue";
import { ProjectInfos } from "./type";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import ProjectCard from "./components/ProjectCard.vue";
import { useAppGlobal } from "./stores/appGlobal";

const appGlobal = useAppGlobal();

const projectInfos: Ref<ProjectInfos | null> = ref(null);
let unlisten: Promise<UnlistenFn> | null = null;

const updateProjectInfo = () => {
  invoke("project_manager_project_infos").then(
    (infos) => (projectInfos.value = infos as ProjectInfos)
  );
};

onMounted(() => {
  appGlobal.appBartitle = "OpenProject";
  updateProjectInfo();

  unlisten = listen<ProjectInfos>("project-list-update", () => {
    updateProjectInfo();
  });

  invoke("project_manager_init_watcher");
});

onUnmounted(() => unlisten?.then((unlisten) => unlisten()));
</script>

<template>
  <v-item-group selected-class="bg-primary">
    <v-col md="4">
      <ProjectCard
        v-for="(project, index) in projectInfos?.infos"
        :key="index"
        :info="project"
        class="mb-4"
      />
    </v-col>
  </v-item-group>
</template>
