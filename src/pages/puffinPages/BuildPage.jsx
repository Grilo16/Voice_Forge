import { useState } from "react";
import styled from "styled-components";
import { invoke } from "@tauri-apps/api";

export const BuildPage = () => {

    const [rustOutput, setRustOutput] = useState("")
    const [createTableOutput, setCreateTableOutput] = useState("")
    const [insertOutput, setInsertOutput] = useState("")
    const [ssh, setSsh] = useState("")
    const [justDoIt, setJustDoIt] = useState("")

    const [launchOutput, setLaunchOutput] = useState("")
    const callRust = async () => {
        setRustOutput(await invoke("connect_to_db"))
        console.log("called")
    }


    

    let comand  = {comand: "tmux new-session -d -s my_session; source ~/puffin_env/bin/activate; python3 ~/spun/repos/speedy/script/run.py -i Asfas -d 2021-14 -n 99 --accent London --donorid Anything --donorvb --dry "}
    let query = {query: 2}
    // let query = {query: "select * from flags"}
    // let query2 = {query: "insert into flags (label, flag, input_type, required) values ('test', 'test', 'test', 0)"}
    const callCreate = async () => {
        setCreateTableOutput(await invoke("insert_data", query2) )
    }
    const callInsert = async () => {
        setCreateTableOutput(await invoke("insert_data", query) )
    }
    const connectSsh = async () => {
        setSsh(await invoke("connect_ssh", comand) )
    }

    const justDoItPlz = async () => {
        setJustDoIt(await invoke("just_do_it"))
    }


    const launchClient = async () => {
        setLaunchOutput(await invoke("launch_cloud_client"))
    }
    return (
        <>
        <h1>i'm the Build Page</h1>
        {/* <button onClick={() => callCreate()}>create</button>
        {createTableOutput}
        <button onClick={() => callInsert()}>insert</button>
        <button onClick={() => connectSsh()}>connect ssh</button>
        <button onClick={() => justDoItPlz()}>just do it</button> */}
        <button onClick={() => launchClient()}>Launch Client</button>
        <h1>
            {launchOutput}
            </h1>
        {insertOutput}
        <p>this is ssh : {ssh}</p>
        {rustOutput}
        </>

        
    )
    };