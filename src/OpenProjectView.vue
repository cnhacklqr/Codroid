<script setup lang="ts">
import { onMounted, ref, Ref } from "vue";
import ProjectCard from "./components/ProjectCard.vue";
import { useAppGlobal } from "./stores/appGlobal";
import { useProjectInfoGlobal } from "./stores/projectInfoGlobal";
import { useRouter } from "vue-router";
import { commands, events, ProjectInfo, ProjectInfos } from "./bindings";

const appGlobal = useAppGlobal();
const projectInfoGlobal = useProjectInfoGlobal();
const router = useRouter();

const projectInfos: Ref<ProjectInfos | null> = ref(null);

const updateProjectInfo = () => {
  commands
    .projectManagerProjectInfos()
    .then((infos) => (projectInfos.value = infos));
};

onMounted(() => {
  appGlobal.appBartitle = "OpenProject";
  updateProjectInfo();

  events.projectManagerUpdate.listen(() => updateProjectInfo());

  commands.projectManagerInitWatcher();
});

const openProject = (info: ProjectInfo) => {
  projectInfoGlobal.projectInfo = info;
  router.push("project");
};
</script>

<template>
  <v-item-group selected-class="bg-primary">
    <v-col md="4">
      <ProjectCard
        v-for="(project, index) in projectInfos?.infos"
        :key="index"
        :info="project"
        class="mb-4"
        @click="openProject(project)"
      />
    </v-col>
  </v-item-group>
</template>
