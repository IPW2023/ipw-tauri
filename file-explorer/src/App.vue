<script setup lang="ts">
import {invoke} from "@tauri-apps/api"
import {ref, Ref} from "vue";

const currentDir: Ref<string> = ref("")
const files: Ref<string[]> = ref([])

/* TO-DO (3)
  Show the current directory on the screen.
*/

/* TO-DO (4)
  Show the contents of the current directory on the screen.
  Once a new file is created by the user, it should also appear on the screen.
*/

/* TO-DO (5)
  Allow the user to view and edit any text file by clicking on one of the displayed files.
*/

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

function createNewFile() {
  /* TO-DO (1)
    Using Tauri commands, make a function that creates a new file in the current directory.
    (HINT: use std::fs)
  */
}

function writeToFile() {
  /* TO-DO (2)
        Allow the user to write to the file once its been created.
  */
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
