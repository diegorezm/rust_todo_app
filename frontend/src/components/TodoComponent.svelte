<script lang="ts">
  import type TodoInterface from "../interfaces/TodoInterface";
  import { toggleCompleted } from "$lib";
  export let todos: TodoInterface[];

  const updateCompleted = async (id: string) => {
    await toggleCompleted(id);
    const updatedTodos = todos.map((todo) => {
      if (todo.id === id) {
        return { ...todo, completed: !todo.completed };
      }
      return todo;
    });
    todos = updatedTodos;
  };
</script>

<section class="todo">
  <div class="todo__wrapper">
    {#each todos as todo}
      <div class="card">
        <div class="todo__checkbox">
          <input
            type="checkbox"
            class="btn-check"
            id={todo.id}
            autocomplete="off"
            checked={todo.completed}
            on:change={() => updateCompleted(todo.id || "")}
          />
          <label class="btn btn-outline-primary" for={todo.id}>
            {todo.completed ? "Checked" : "Todo"}
          </label>
        </div>
        <div class="todo__name">
          <h1>{todo.todo}</h1>
        </div>
      </div>
    {/each}
  </div>
</section>

<style>
  .todo {
    display: flex;
    flex-direction: row;
  }
  .todo__wrapper {
    display: flex;
    flex-direction: column;
    gap: 1em;
  }
  .card {
    display: flex;
    padding: 0.5em;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    gap: 5rem;
  }
</style>
