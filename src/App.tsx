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

  useEffect(() => {
    getTodos();
  }, []);

  return (
    <div>
      <h2>Hello World</h2>
      <form
        onSubmit={async (e) => {
          e.preventDefault();
          if (inputRef.current) {
            await addTodo(inputRef.current.value);
          }
        }}
      >
        <input placeholder="add todo" ref={inputRef} />
        <button type="submit">SUBMIT</button>
      </form>
      <ul>
        {todos.map((todo) => (
          <li key={todo.id}>
            <span>
              {todo.content}
            </span>
            <button>DELETE</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
