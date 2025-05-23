<template>
  <div
    class="min-h-screen bg-gradient-to-br from-indigo-200 via-purple-200 to-pink-200 dark:from-gray-900 dark:via-gray-800 dark:to-gray-700 p-4 font-sans"
  >
    <div
      class="max-w-md mx-auto backdrop-blur-md bg-white/60 dark:bg-white/10 shadow-xl rounded-xl p-6 border border-white/40 dark:border-white/20"
    >
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-2">
          <span class="text-3xl">📝</span>
          <h1
            class="text-3xl font-bold text-blue-700 dark:text-blue-300 drop-shadow"
          >
            NoDo
          </h1>
        </div>
        <button
          @click="darkMode = !darkMode"
          class="text-sm bg-gray-200 dark:bg-gray-600 text-gray-800 dark:text-white px-3 py-1 rounded shadow hover:shadow-md transition"
        >
          {{ darkMode ? "☀️ Light" : "🌙 Dark" }}
        </button>
      </div>

      <div class="mb-4">
        <label
          class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1"
          >Select List</label
        >
        <select
          v-model="selectedList"
          class="w-full border rounded p-2 bg-white/70 dark:bg-white/20 dark:text-white backdrop-blur"
        >
          <option v-for="list in lists" :key="list" :value="list">
            {{ list }}
          </option>
        </select>
      </div>

      <div class="flex gap-2 mb-4">
        <button @click="filter = 'all'" :class="filterBtn('all')">All</button>
        <button @click="filter = 'today'" :class="filterBtn('today')">
          Today
        </button>
        <button @click="filter = 'past'" :class="filterBtn('past')">
          Past Due
        </button>
        <button @click="filter = 'completed'" :class="filterBtn('completed')">
          Completed
        </button>
      </div>

      <form @submit.prevent="addTask" class="space-y-3 mb-6">
        <input
          v-model="newTask"
          class="w-full border border-gray-300 dark:border-gray-600 p-2 rounded bg-white/70 dark:bg-white/20 dark:text-white backdrop-blur"
          placeholder="Task title"
        />
        <textarea
          v-model="newDescription"
          class="w-full border border-gray-300 dark:border-gray-600 p-2 rounded bg-white/70 dark:bg-white/20 dark:text-white backdrop-blur"
          placeholder="Description (optional)"
        ></textarea>
        <input
          type="date"
          v-model="newDueDate"
          class="w-full border border-gray-300 dark:border-gray-600 p-2 rounded bg-white/70 dark:bg-white/20 dark:text-white backdrop-blur"
        />
        <select
          v-model="newRepeat"
          class="w-full border border-gray-300 dark:border-gray-600 p-2 rounded bg-white/70 dark:bg-white/20 dark:text-white backdrop-blur"
        >
          <option value="none">No Repeat</option>
          <option value="daily">Repeat Daily</option>
          <option value="weekly">Repeat Weekly</option>
          <option value="monthly">Repeat Monthly</option>
        </select>
        <div>
          <button
            @click.prevent="addAttachment"
            class="text-sm text-blue-700 dark:text-blue-300 underline"
          >
            + Add Attachment
          </button>
          <ul class="mt-2 space-y-1 text-sm text-gray-600 dark:text-gray-300">
            <li v-for="(file, index) in newAttachments" :key="index">
              📎 {{ file }}
            </li>
          </ul>
        </div>
        <button
          class="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 rounded shadow"
        >
          Add Task
        </button>
      </form>

      <ul class="space-y-3">
        <li
          v-for="task in filteredTasks"
          :key="task.id"
          :class="[
            'flex flex-col gap-1 backdrop-blur border p-3 rounded-lg shadow-sm hover:shadow-md transition',
            task.due_date && task.due_date < localToday && !task.completed
              ? 'bg-red-100 dark:bg-red-900 border-red-300 dark:border-red-700'
              : 'bg-white/70 dark:bg-white/10 border-gray-200 dark:border-gray-600',
          ]"
        >
          <div class="flex items-center gap-3">
            <input
              type="checkbox"
              :checked="task.completed"
              @change="toggleTask(task)"
              class="w-5 h-5 text-blue-600 dark:text-blue-400 rounded focus:ring-blue-500"
            />
            <input
              v-model="task.title"
              @blur="saveTasks"
              class="flex-1 bg-transparent outline-none text-black dark:text-white border-none focus:ring-0"
            />
            <button
              class="text-red-400 hover:text-red-600 text-lg font-bold px-2"
              @click="deleteTask(task.id)"
            >
              &times;
            </button>
          </div>
          <textarea
            v-if="task.description"
            v-model="task.description"
            @blur="saveTasks"
            class="text-sm bg-transparent text-gray-600 dark:text-gray-300 pl-8 w-full outline-none resize-none"
          ></textarea>
          <div
            v-if="task.due_date"
            class="text-xs text-gray-500 dark:text-gray-400 pl-8"
          >
            Due: {{ task.due_date }}
          </div>
          <div
            v-if="task.repeat && task.repeat !== 'none'"
            class="text-xs text-purple-500 dark:text-purple-300 pl-8"
          >
            Repeats {{ task.repeat }}
          </div>
          <ul
            v-if="task.attachments?.length"
            class="text-xs text-blue-500 dark:text-blue-300 pl-8 space-y-1 pt-1"
          >
            <li
              v-for="(file, i) in task.attachments"
              :key="i"
              class="truncate flex items-center gap-2"
            >
              📎 {{ file }}
              <button
                @click="openAttachment(file)"
                class="text-green-600 hover:underline text-xs"
              >
                Open
              </button>
              <button
                @click="removeAttachment(task, i)"
                class="text-red-400 hover:text-red-600 text-xs"
              >
                Remove
              </button>
            </li>
          </ul>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { openPath as openOpener } from "@tauri-apps/plugin-opener";

interface Task {
  id: number;
  title: string;
  completed: boolean;
  list: string;
  description?: string;
  due_date?: string;
  repeat?: "none" | "daily" | "weekly" | "monthly";
  attachments?: string[];
}

const darkMode = ref(false);
watch(darkMode, (enabled) => {
  document.documentElement.classList.toggle("dark", enabled);
});

const lists = ref<string[]>(["General", "Work", "Personal"]);
const selectedList = ref("General");
const tasks = ref<Task[]>([]);
const newTask = ref("");
const newDescription = ref("");
const newDueDate = ref("");
const newRepeat = ref<"none" | "daily" | "weekly" | "monthly">("none");
const newAttachments = ref<string[]>([]);
const filter = ref<"all" | "today" | "past" | "completed">("all");

const localToday = new Date().toISOString().split("T")[0];

const filteredTasks = computed(() => {
  return tasks.value.filter((task) => {
    if (task.list !== selectedList.value) return false;
    if (filter.value === "completed") return task.completed;
    if (filter.value === "today") return task.due_date === localToday;
    if (filter.value === "past")
      return (
        task.due_date !== undefined &&
        task.due_date < localToday &&
        !task.completed
      );
    return true;
  });
});

function filterBtn(type: typeof filter.value) {
  return [
    "px-3 py-1 rounded border border-gray-300 dark:border-gray-600",
    filter.value === type
      ? "bg-blue-500 text-white"
      : "bg-white/70 dark:bg-white/20 dark:text-white",
  ];
}

async function loadTasks() {
  try {
    const result = await invoke<Task[]>("load_tasks");
    tasks.value = result;
  } catch (e) {
    console.error("[loadTasks] Failed:", e);
  }
}

async function saveTasks() {
  try {
    await invoke("save_tasks", { tasks: tasks.value });
  } catch (e) {
    console.error("[saveTasks] Failed:", e);
  }
}

function addTask() {
  if (!newTask.value.trim()) return;
  tasks.value.push({
    id: Date.now(),
    title: newTask.value,
    completed: false,
    list: selectedList.value,
    description: newDescription.value || undefined,
    due_date: newDueDate.value || undefined,
    repeat: newRepeat.value || "none",
    attachments:
      newAttachments.value.length > 0 ? [...newAttachments.value] : undefined,
  });
  newTask.value = "";
  newDescription.value = "";
  newDueDate.value = "";
  newRepeat.value = "none";
  newAttachments.value = [];
  saveTasks();
}

function deleteTask(id: number) {
  tasks.value = tasks.value.filter((t) => t.id !== id);
  saveTasks();
}

function toggleTask(task: Task) {
  task.completed = !task.completed;

  if (
    task.completed &&
    task.repeat &&
    task.repeat !== "none" &&
    task.due_date
  ) {
    const newDueDate = new Date(task.due_date);
    switch (task.repeat) {
      case "daily":
        newDueDate.setDate(newDueDate.getDate() + 1);
        break;
      case "weekly":
        newDueDate.setDate(newDueDate.getDate() + 7);
        break;
      case "monthly":
        newDueDate.setMonth(newDueDate.getMonth() + 1);
        break;
    }
    tasks.value.push({
      ...task,
      id: Date.now(),
      completed: false,
      due_date: newDueDate.toISOString().split("T")[0],
    });
  }

  saveTasks();
}

async function addAttachment() {
  const selected = await open({ multiple: true });
  if (Array.isArray(selected)) {
    newAttachments.value.push(...selected);
  } else if (selected) {
    newAttachments.value.push(selected);
  }
}

async function openAttachment(path: string) {
  await openOpener(path)
    .then(() => console.log("Opened:", path))
    .catch((e: any) => console.error("[openAttachment] Failed:", e));
}

function removeAttachment(task: any, index: number) {
  task.attachments.splice(index, 1);
  saveTasks();
}

onMounted(() => {
  loadTasks();
});
</script>
<style scoped>
body {
  background-color: transparent;
  font-family: "Inter", sans-serif;
}
</style>
