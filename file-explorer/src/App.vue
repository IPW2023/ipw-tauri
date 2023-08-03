<script setup lang="ts">
import {invoke} from "@tauri-apps/api"
import {ref, Ref} from "vue";

const currentDir: Ref<string> = ref("")
const files: Ref<string[]> = ref([])

invoke("get_current_directory").then((dir) => currentDir.value = dir as string)

invoke("get_current_files", { currentDir: currentDir.value })
.then((files_vec) => files.value = files_vec as string[])
.catch((error) => {
  console.log(error)
})

const fileName = ref("")
const fileContents = ref("")
const showFile = ref(false)
const successfulWrite = ref(false)

/* TO-DO (1)
  Using Tauri commands, make a function that creates a new file in the current directory.
*/

function createNewFile() {
  invoke("create_new_file", { fileName: fileName.value })
  .then(() => {
    showFile.value = true
    invoke("get_current_files", { currentDir: currentDir.value })
      .then((files_vec) => files.value = files_vec as string[])
      .catch((error) => {
        console.log(error)
    })
  })
  .catch((error) => {
    console.log(error)
  })
}

function writeToFile() {
  invoke("write_to_file", { 
    fileName: fileName.value, 
    fileContents: fileContents.value 
  })
  .then(() => {
    fileContents.value = ""
    successfulWrite.value = true;
  })
  .catch((error) => {
    console.log(error)
  })
}
</script>

<template>
  <div class="container">
    <h1>File Explorer</h1>

    <p>Current directory is: {{ currentDir }}</p>

    <p v-for="file in files">
      {{ file }}
    </p>

    <input v-model="fileName" placeholder="File Name">

    <button @click="createNewFile()">Create New File</button>

    <div v-if="showFile">
      <p>{{ currentDir }}\{{ fileName }}</p>
      <textarea v-model="fileContents"></textarea>
      <button @click="writeToFile()">Save</button>
      <p v-if="successfulWrite">Saved!</p>
    </div>
    
  </div>
</template>

<style scoped>
  ul {
    list-style-type: none;
  }
</style>
