import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

export default function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1 className="text-3xl font-semibold font-display">hana welcomes</h1>
      <p>This setup won't take long.</p>
    </main>
  );
}