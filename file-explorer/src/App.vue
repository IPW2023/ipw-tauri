<script setup lang="ts">
import {invoke} from "@tauri-apps/api"
import {ref} from "vue";

const current_dir = ref("")

invoke("get_current_directory").then((dir: string) => current_dir.value = dir)

const fileName = ref("")
const showFile = ref(false)

/* TO-DO (1)
  
*/

function createNewFile() {
  invoke("create_new_file", { fileName: fileName.value })
  .then(() => {
    showFile.value = true;
  })
  .catch((error) => {
    console.log(error);
  })
}
</script>

<template>
  <div class="container">
    <h1>File Explorer</h1>

    <p>Current directory is: {{ current_dir }}</p>

    <!-- <div class="column">
      
    </div> -->
    <input v-model="fileName" placeholder="File Name">

    <button @click="createNewFile()">Create New File</button>

    <div v-if="showFile">
      <p>{{ current_dir }}\{{ fileName }}</p>
    </div>
    
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
