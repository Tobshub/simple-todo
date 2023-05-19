import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useRef, useState } from "react";

// simple todo app
// backend logic & persistence is handled in tauri
// database is sqlite
// put is all in a docker container(make sure the data persists)
function App() {
  const inputRef = useRef<HTMLInputElement>(null);
  const [todos, setTodos] = useState<{ id: number; content: string }[]>([]);

  const getTodos = () => {
    invoke("api", { action: "get" }).then((res) => console.log(res));
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
            await invoke("api", {
              action: "add",
              content: inputRef.current.value,
            }).then((res) => {
              console.log(res);
            });
          }
        }}
      >
        <input placeholder="add todo" ref={inputRef} />
        <button type="submit">SUBMIT</button>
      </form>
      <button onClick={getTodos}>REFETCH</button>
    </div>
  );
}

export default App;
