import { createApp } from "vue";
import { createMemoryHistory, createRouter } from "vue-router";
import App from "./App.vue";
import HomeView from "./HomeView.vue";
import ProjectView from "./ProjectView.vue";
import SettingsView from "./SettingsView.vue";
import AboutView from "./AboutView.vue";
import CreateProjectView from "./CreateProjectView.vue";
import OpenProjectView from "./OpenProjectView.vue";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { createPinia } from "pinia";
import Varlet from "@varlet/ui";
import "@varlet/ui/es/style";
import { Themes, StyleProvider } from "@varlet/ui";
import "@varlet/touch-emulator";
import { library } from "@fortawesome/fontawesome-svg-core";
import {
  faCode,
  faFileCode,
  faCogs,
  faCircleInfo,
  faDove,
  faCog,
  faKiwiBird,
} from "@fortawesome/free-solid-svg-icons";
import { faRust } from "@fortawesome/free-brands-svg-icons";

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

const pinia = createPinia();
StyleProvider(Themes.md3Light);

library.add(
  faCode,
  faFileCode,
  faCogs,
  faCircleInfo,
  faDove,
  faRust,
  faCog,
  faKiwiBird,
);

// eslint-disable-next-line @typescript-eslint/no-explicit-any
(window as any).router = router;

createApp(App)
  .use(router)
  .use(pinia)
  .use(Varlet)
  .component("font-awesome-icon", FontAwesomeIcon)
  .mount("#app");
