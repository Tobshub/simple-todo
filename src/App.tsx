import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useRef, useState } from "react";

// simple todo app
// backend logic & persistence is handled in tauri
// database is sqlite
// put is all in a docker container(make sure the data persists)
function App() {
  const inputRef = useRef<HTMLInputElement>(null);
  const [todos, setTodos] = useState<{ id: number; content: string }[]>([]);

  const getTodos = async () => {
    const todos = await invoke<{ id: number; content: string }[]>("api", {
      action: "get",
    });
    setTodos(todos);
  };

  const addTodo = async (content: string) => {
    if (content.length) {
      await invoke("api", {
        action: "add",
        content,
      });
      await getTodos();
    }
  };

  const deleteTodo = async (id: number) => {
    await invoke("api", { action: "delete", id });
    await getTodos();
  };

  useEffect(() => {
    getTodos();
  }, []);

  return (
    <main>
      <h1>Simple To-do</h1>
      <form
        className="add-form"
        onSubmit={async (e) => {
          e.preventDefault();
          if (inputRef.current) {
            const content = inputRef.current.value;
            setTodos((state) => [
              ...state,
              { id: (state.at(-1)?.id ?? 0) + 1, content },
            ]);
            await addTodo(content);
            inputRef.current.value = "";
          }
        }}
      >
        <input placeholder="New To-do" ref={inputRef} />
        <button type="submit">Add To-do</button>
      </form>
      <ul className="todo-list">
        {todos.map((todo) => (
          <li key={todo.id}>
            <span>{todo.content}</span>
            <button className="delete-btn" onClick={() => deleteTodo(todo.id)}>
              DELETE
            </button>
          </li>
        ))}
      </ul>
    </main>
  );
}

export default App;
