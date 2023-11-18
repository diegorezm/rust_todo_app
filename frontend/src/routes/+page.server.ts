import {  getTodos } from "$lib";

export async function load() {
  const todos = await getTodos()
  return { todos }
}
