<script setup lang="ts">
import {invoke} from "@tauri-apps/api"
import {ref, Ref} from "vue";

const filePath: Ref<string> = ref("")
const files: Ref<string[]> = ref([])

const fileName = ref("")
const fileContents = ref("")
const showFile = ref(false)
const successfulWrite = ref(false)

function createNewFile() {
  /* TO-DO (1)
      Using Tauri commands, make a function that creates a new file in the current directory.
      (HINT: use std::fs - https://doc.rust-lang.org/std/fs/)
  */
}

function writeToFile() {
  /* TO-DO (3)
      Create a function that writes contents to the file that has been created.
  */
}

function viewFile(file: string) {
  
}

/* TO-DO (6) - bonus
  Allow the user to navigate to different directories.
  You will need to also show directories within the current directory and check if the
  user clicks on a directory. If they do, show the contents of that directory to the screen.
*/
</script>

<template>
  <div class="container">
    <h1>File Explorer</h1>

    <div class="files">

      <!-- TO-DO (4)
        Show the contents of the current directory on the screen.
        Once a new file is created by the user, it should also appear on the screen. 
      -->
      
      <button v-for="file in files" @click="viewFile(file)">
        {{ file }}
      </button>

      <!-- TO-DO (5)
        Allow the user to view and edit any text file by clicking on one of the displayed files. 
      -->

      <br>

      <input v-model="fileName" placeholder="File Name">

      <br>

      <button @click="createNewFile()">Create New File</button>
    </div>

    <div v-if="showFile">
      <!-- TO-DO (2)
        Show the absolute path to the newly created file.
        (HINT: use std::env)
      -->
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
  button {
    width: 200px;
  }

  .files {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>

