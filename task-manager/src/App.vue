<script setup lang="ts">
import {ref, Ref} from "vue";

/* TO-DO (8)
  Make a two separate components: one for creating the task, and one for viewing all tasks.
  The user should be able to navigate back and forth between these two screens.
  (HINT: use VueRouter)
*/

interface Task {
  id: number,
  name: string,
  description: string,
  priority: number
}

const taskName: Ref<string> = ref("")
const taskDescription: Ref<string> = ref("")
const taskPriority: Ref<number> = ref(0)

/* TO-DO (6)
  Implement a way for the user to add multiple tasks.
  (HINT: use v-for to show all tasks to the screen)
*/
const tasks: Ref<Task[]> = ref([])

function createTask() {
  /* TO-DO (4)
  Validate the user input before showing it on the screen.
  For example, the task must have a name. It's your choice what checks you want to make.
  Only if the user input is valid, will you show it on the screen.
*/
}

</script>

<template>
  <div class="container">
    <h1>Create a New Task</h1>
    <!-- TO-DO (1) 
      Make a way for a user to create a new task. 
      The task should have a name, description and priority. 
    -->

    <!-- TO-DO (2)
      Make it so that this button creates a new task with the given name, description and priority.
    -->
    <button>Create Task</button>

    <div class="show-task">
      <!-- TO-DO (3)
        Show the newly created task on the screen.
        The user should be able to see the name, description and priority of the task they have just created.
      -->

      <!-- TO-DO (5)
      If the input is not valid, show an alert on the screen.
      If it is valid, show the newly created task.
      (HINT: use v-if)
      -->

      <!-- TO-DO (9) - bonus
        Implement a way for the user to check off tasks (mark them as 'done')
        You decide what the done tasks will look like (eg crossed out, grayed out, deleted etc.)
      -->

      <!-- TO-DO (10) - bonus
        Implement a filter for each priority number.
        For example, the user can select to display only tasks that have priority 1.
      -->
    </div>
  </div>
</template>

<style scoped>
/* TO-DO (7)
  Use CSS to change the task name color depending on the priority.
  (HINT: use attribute binding) 
*/
</style>
