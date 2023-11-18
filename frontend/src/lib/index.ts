import { invalidateAll } from "$app/navigation";
import type TodoInterface from "../interfaces/TodoInterface";

const url = "http://localhost:8080"

export const getTodos = async (): Promise<TodoInterface[] | undefined> => {
  try {
    const res = await fetch(`${url}/todo`);
    if (!res.ok) {
      throw new Error("Failed to fetch todos");
    }
    const data: TodoInterface[] = await res.json();
    return data;
  } catch (error: any) {
    console.error("Error fetching todos:", error);
    return undefined
  }
};

export const addTodo = async (todo: TodoInterface) => {
  try {
    const request = await fetch(`${url}/todo/add`, {
      method: "POST",
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(todo)
    })
    if (!request.ok) {
      throw new Error('Failed to add todo');
    };
    const data = await request.json();
    invalidateAll()
    return true;
  } catch (error) {
    console.error('Error adding todo:', error);
    return false
  }
}

export const toggleCompleted = async (id: string) => {
  try {
    const res = await fetch(`${url}/todo/toggle/${id}`, {
      method: "PUT"
    })
    if (!res.ok) {
      throw new Error("Failed to toggle todo.");
    }
  } catch (error) {
    console.error("Error fetching todos:", error);
  }
}