import { useState } from "react";
import styled from "styled-components";
import { invoke } from "@tauri-apps/api";

export const BuildPage = () => {

   
    const [ssh, setSsh] = useState("")
    const [launchOutput, setLaunchOutput] = useState("")


    

    let comand  = {comand: "tmux new-session -d -s my_session; source ~/puffin_env/bin/activate; python3 ~/spun/repos/speedy/script/run.py -i Asfas -d 2021-14 -n 99 --accent London --donorid Anything --donorvb --dry "}
    let query = {query: 2}

    const launchClient = async () => {
        setLaunchOutput(await invoke("launch_cloud_client"))
    }
    return (
        <>
        <h1>i'm the Build Page</h1>
        <button onClick={() => launchClient()}>Launch Client</button>
        <h1>{launchOutput}</h1>
        </>

        
    )
    };