<script setup lang="ts">
import { computed, onMounted, ref, Ref } from "vue";
import ProjectCard from "./components/ProjectCard.vue";
import { useAppGlobal } from "./stores/app-global";
import { useProjectInfoGlobal } from "./stores/projectInfoGlobal";
import { useRouter } from "vue-router";
import { commands, events, ProjectInfo, ProjectInfos } from "./bindings";

const appGlobal = useAppGlobal();
const projectInfoGlobal = useProjectInfoGlobal();
const router = useRouter();
const showDeletePopup = ref(false);

const projectInfos: Ref<ProjectInfos | null> = ref(null);
const showPlaceholder = computed(() => {
  return (
    projectInfos.value === null ||
    Object.values(projectInfos.value.infos).length === 0
  );
});

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

const deleteProject = (info: ProjectInfo) => {
  commands.projectManagerRemoveProject(info.name);
  showDeletePopup.value = false;
};
</script>

<template>
  <div>
    <var-list v-if="!showPlaceholder">
      <var-cell>
        <var-space align="start" justify="start">
          <var-menu
            v-for="(project, index) in projectInfos?.infos"
            :key="index"
          >
            <ProjectCard v-ripple :info="project" class="project-card" />

            <template #menu>
              <var-cell v-ripple @click="openProject(project)">Open</var-cell>
              <var-cell v-ripple color="primary" @click="showDeletePopup = true"
                >Delete</var-cell
              >
              <var-popup v-model:show="showDeletePopup" :default-style="false">
                <var-result
                  class="result"
                  type="warning"
                  :title="`Delete Project: \'${project.name}\' ?`"
                  description="This is unrecoverable!"
                >
                  <template #footer>
                    <var-space direction="row" align="center">
                      <var-button type="danger" @click="deleteProject(project)">
                        Confirm
                      </var-button>
                      <var-button type="info" @click="showDeletePopup = false">
                        Cancel
                      </var-button>
                    </var-space>
                  </template>
                </var-result>
              </var-popup>
            </template>
          </var-menu>
        </var-space>
      </var-cell>
    </var-list>

    <PlaceHolderPage
      v-if="showPlaceholder"
      reason="Your have no project here"
    />
  </div>
</template>

<style scoped>
.result {
  width: 75vw !important;
}
</style>
