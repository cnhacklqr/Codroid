import { defineStore } from "pinia";
import { Ref, ref } from "vue";
import { ProjectInfo } from "../bindings";

export const useProjectInfoGlobal = defineStore("project-info-global", () => {
  const projectInfo: Ref<ProjectInfo | null> = ref(null);
  return {
    projectInfo,
  };
});
