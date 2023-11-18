<script lang="ts">
  import { onMount } from "svelte";
  import Todo from "../components/TodoComponent.svelte";
  import Form from "../components/Form.svelte";
  import type TodoInterface from "../interfaces/TodoInterface";

  export let data;
  let todos: TodoInterface[] = [];

  onMount(() => {
    todos = data.todos || [];
  });
</script>

<div class="container my-2 border">
  <Form />
  <Todo {todos} />
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 1em;
  }
</style>
