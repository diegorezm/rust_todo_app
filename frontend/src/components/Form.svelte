<script lang="ts">
  import { addTodo } from "$lib";
  import type TodoInterface from "../interfaces/TodoInterface";


  const handleSubmit = async (event: Event) => {
    const form = event.target as HTMLFormElement;
    const formData = new FormData(form);
    const todo = formData.get("todo") as string;
    if (!todo) {
      console.error("Todo cannot be empty");
      return;
    }

    const newTodo: TodoInterface = {
      todo,
      completed: false,
    };

    try {
      const response = await addTodo(newTodo);
      if (response) {
        form.reset();
      }
    } catch (error) {
      console.error("Error adding todo:", error);
    }
  };
</script>

<form on:submit|preventDefault={handleSubmit} class="form" method="POST">
  <div class="input_div">
    <label for="new_todo" class="form-label">New todo:</label>
    <input
      type="text"
      class="input form-control"
      id="new_todo"
      name="todo"
      placeholder="your new todo..."
    />
    <button type="submit" class="btn btn-primary">Submit</button>
  </div>
</form>

<style>
  .form {
    width: 80%;
  }
  .input_div {
    position: relative;
  }
  .input {
    height: 60%;
  }
  .btn {
    position: absolute;
    top: 45%;
    right: 0;
    height: 55%;
  }
</style>
