import { useState } from "react";
import reactLogo from "../assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { MachineLauncher } from "../components";

export const HomePage = () => {
    const [greetMsg, setGreetMsg] = useState("");
    const [name, setName] = useState("");
  
    async function greet() {
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      setGreetMsg(await invoke("greet", { name }));
    }

    return (
      <div>
        <h1>Voice Forge</h1>
        <MachineLauncher/>
      </div>
    )
};

