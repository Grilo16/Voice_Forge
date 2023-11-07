import { useState } from "react";
import styled from "styled-components";
import { invoke } from "@tauri-apps/api";
import { selectArgs, selectCredentials } from "../../features/reducers/machineReducer";
import { useSelector } from "react-redux";

export const BuildPage = () => {

    const launchArgs = useSelector(selectArgs)
    const credentials = useSelector(selectCredentials)

    const [ssh, setSsh] = useState("")
    const [launchOutput, setLaunchOutput] = useState("")
    const [credentialList, setCredentialList] = useState([])
    const [tmuxCommandOutput, setTmuxCommandOutput] = useState([])


    let comand  = {comand: "tmux new-session -d -s my_session; source ~/puffin_env/bin/activate; python3 ~/spun/repos/speedy/script/run.py -i Asfas -d 2021-14 -n 99 --accent London --donorid Anything --donorvb --dry "}
    let query = {query: 2}

    const launchClient = async () => {
        setLaunchOutput(await invoke("launch_cloud_instance"))
    }
    const getCredentials = async () => {
        setCredentialList(await invoke("get_ssh_credentials"))
    }


    let argList = [] 
    Object.entries(launchArgs).forEach(([_, {flag, value}]) => argList.push(flag, value))


    const executeCommand = async () => {
        setTmuxCommandOutput(await invoke("run_tmux_command", {sshCredentials: credentials, runFlags: argList}))
    }
    console.log(tmuxCommandOutput)

    return (
        <>
        <h1>i'm the Build Page</h1>
        <button onClick={() => executeCommand()}>test command</button>
        {/* <button onClick={() => getCredentials()}>get credentials</button>
        <button onClick={() => getCredentials()}>get credentials</button> */}
        <h1>{launchOutput}</h1>
        </>

        
    )
    };