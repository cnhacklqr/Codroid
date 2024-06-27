import { defineStore } from "pinia";
import { ref } from "vue";

export const useAppGlobal = defineStore("app-global", () => {
  const appBartitle = ref("");

  return {
    appBartitle,
  };
});
