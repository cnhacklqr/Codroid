import { createApp } from "vue";
import { createMemoryHistory, createRouter } from "vue-router";
import App from "./App.vue";
import HomeView from "./HomeView.vue";
import ProjectView from "./ProjectView.vue";
import SettingsView from "./SettingsView.vue";
import AboutView from "./HomeView.vue";
import CreateProjectView from "./CreateProjectView.vue";
import OpenProjectView from "./OpenProjectView.vue";
import "vuetify/styles";
import { createVuetify } from "vuetify";
import "@mdi/font/css/materialdesignicons.css";

const routes = [
  { path: "/", component: HomeView },
  { path: "/project", component: ProjectView },
  { path: "/createProject", component: CreateProjectView },
  { path: "/openProject", component: OpenProjectView },
  { path: "/settings", component: SettingsView },
  { path: "/about", component: AboutView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

const vuetify = createVuetify({
  icons: {
    defaultSet: "mdi",
  },
});

createApp(App).use(router).use(vuetify).mount("#app");
