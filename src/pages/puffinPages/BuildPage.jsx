import { useState } from "react";
import styled from "styled-components";
import { invoke } from "@tauri-apps/api";

export const BuildPage = () => {

    const [rustOutput, setRustOutput] = useState("")
    const [createTableOutput, setCreateTableOutput] = useState("")
    const [insertOutput, setInsertOutput] = useState("")

    const callRust = async () => {
        setRustOutput(await invoke("connect_to_db"))
        console.log("called")
    }

    let query = {query: 2}
    // let query = {query: "select * from flags"}
    // let query2 = {query: "insert into flags (label, flag, input_type, required) values ('test', 'test', 'test', 0)"}
    const callCreate = async () => {
        setCreateTableOutput(await invoke("insert_data", query2) )
    }
    const callInsert = async () => {
        setCreateTableOutput(await invoke("insert_data", query) )
    }
    

    return (
        <>
        <h1>i'm the Build Page</h1>
        <button onClick={() => callCreate()}>create</button>
        {createTableOutput}
        <button onClick={() => callInsert()}>insert</button>
        {insertOutput}

        {rustOutput}
        </>

        
    )
    };