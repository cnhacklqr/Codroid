import { createApp } from "vue";
import { createMemoryHistory, createRouter } from "vue-router";
import App from "./App.vue";
import HomeView from "./HomeView.vue";
import ProjectView from "./ProjectView.vue";
import SettingsView from "./SettingsView.vue";
import AboutView from "./HomeView.vue";
import "mdui/mdui.css";

const routes = [
  { path: "/", component: HomeView },
  { path: "/project", component: ProjectView },
  { path: "/settings", component: SettingsView },
  { path: "/about", component: AboutView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

createApp(App).use(router).mount("#app");
