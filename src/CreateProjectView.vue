<script setup lang="ts">
import { computed, ComputedRef, onMounted, ref, Ref } from "vue";
import { useRouter } from "vue-router";
import { useAppGlobal } from "./stores/appGlobal";
import { commands, ProjectInfo, Template } from "./bindings";
import { useProjectInfoGlobal } from "./stores/projectInfoGlobal";

const router = useRouter();
const appGlobal = useAppGlobal();
const projectInfoGlobal = useProjectInfoGlobal();

onMounted(() => (appGlobal.appBartitle = "CreateProject"));

// stepper
const step = ref(0);
const stepPrefixed = computed(() => step.value + 1);

const next = () => (step.value += 1);
const prev = () => {
  if (stepPrefixed.value === 1) {
    router.replace("/");
  } else {
    step.value -= 1;
  }
};

const templateList: Array<Template> = ["empty", "rustBinary", "rustLibrary"];

const projectName = ref("");
const projectTemplate: Ref<Template | null> = ref(null);
const projectInfo: ComputedRef<ProjectInfo | null> = computed(() => {
  if (projectTemplate.value === null) {
    return null;
  }

  return {
    name: projectName.value,
    template: projectTemplate.value,
  };
});

// stepper
const finishedList: Record<number, ComputedRef<boolean>> = {
  [1]: computed(() => projectTemplate.value !== null),
  [2]: computed(() => projectName.value !== ""),
};

const nextButtonDisabled = computed(
  () => !finishedList[stepPrefixed.value].value,
);

const success = ref(false);

const confirmCreation = () => {
  commands.projectManagerCreateProject(projectInfo.value!);
  success.value = true;
};

const openProject = () => {
  projectInfoGlobal.projectInfo = projectInfo.value!;
  router.push("project");
};
const backToHome = () => router.replace("/");
</script>

<template>
  <div>
    <var-space align="center" justify="center">
      <var-steps :active="step" class="stepper">
        <var-step>Select Project Template</var-step>
        <var-step>Configure Project</var-step>
        <var-step>Confirm Project Information</var-step>
      </var-steps>
    </var-space>

    <var-space align="center" justify="center">
      <var-card
        v-if="stepPrefixed === 1"
        title="Create New Project"
        subtitle="Select Project Template"
        class="stepper-content"
      >
        <template #description>
          <var-space>
            <p class="description">
              1. Understand project type: Determine the type of project before
              selecting a template.<br />
              2. Plan project: Consider long-term plan and goals when choosing a
              template.<br />
              3. Compare options: Choose from multiple templates with strengths
              and weaknesses in mind.<br />
            </p>
          </var-space>

          <var-space>
            <var-select v-model="projectTemplate" class="input">
              <var-option
                v-for="(labal, index) in templateList"
                :key="index"
                placeholder="empty"
                :label="labal"
              />
            </var-select>
          </var-space>
        </template>

        <template #extra>
          <var-space>
            <var-button type="primary" @click="prev">Prev</var-button>
            <var-button
              type="primary"
              :disabled="nextButtonDisabled"
              @click="next"
              >Next</var-button
            >
          </var-space>
        </template>
      </var-card>

      <var-card
        v-if="stepPrefixed === 2"
        title="Create New Project"
        subtitle="Configure Project"
        class="stepper-content"
      >
        <template #description>
          <var-space>
            <p class="description">
              1. Use clear and concise language.<br />
              2. Focus on the most important information.<br />
              3. Avoid unnecessary words or phrases.<br />
              4. Keep it simple and easy to understand.<br />
            </p>
          </var-space>

          <var-space>
            <var-input
              v-model="projectName"
              placeholder="Project Name"
              class="input"
            />
          </var-space>
        </template>

        <template #extra>
          <var-space>
            <var-button type="primary" @click="prev">Prev</var-button>
            <var-button
              type="primary"
              :disabled="nextButtonDisabled"
              @click="confirmCreation"
              >Create Project</var-button
            >
          </var-space>
        </template>
      </var-card>

      <var-popup
        v-model:show="success"
        :default-style="false"
        :close-on-click-overlay="false"
        :close-on-key-escape="false"
      >
        <var-result
          class="result"
          title="Successed"
          description="Your Project was created successfully."
        >
          <template #footer>
            <var-space direction="row" align="center">
              <var-button type="success" @click="openProject">
                Open Project
              </var-button>

              <var-button type="success" @click="backToHome">
                Back to Home Page
              </var-button>
            </var-space>
          </template>
        </var-result>
      </var-popup>
    </var-space>
  </div>
</template>

<style scoped>
.result {
  width: 75vw !important;
}

.stepper {
  margin-top: 5vh;
  width: 80vw;
}

.stepper-content {
  width: 80vw;
  margin-top: 5vh;
  margin-bottom: 5vh;
}

.description {
  margin: auto;
  margin-top: 1rem;
  margin-left: 1rem;
  color: #9e9e9e;
  font-size: small;
}

.input {
  margin-left: 1rem;
  width: 15rem;
}
</style>
