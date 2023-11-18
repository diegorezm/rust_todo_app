<script lang="ts">
  const get_todos = () => {
    const response = fetch("http://localhost:8080/todo");
    return response;
  };

  

</script>

<div class="app" />
