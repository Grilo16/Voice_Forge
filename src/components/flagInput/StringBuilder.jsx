import { useSelector } from "react-redux";
import { selectCredentials } from "../../features/reducers/machineReducer";
import { useState } from "react";
import styled from "styled-components";
import { FlagInput } from "./FlagInput";
import { StyledButton } from "../machineLauncher";
import { invoke } from "@tauri-apps/api";

export const StringBuilder = ({pageName, launchArgs, stateName, rustCommand, flagData}) => {
    
    console.log(stateName)
    const credentials = useSelector(selectCredentials)
    let argList = [] 
    const commandArgs = {sshCredentials: credentials, runFlags: argList}
    
    Object.entries(launchArgs).forEach(([_, {flag, value}]) => value === " " ? argList.push(flag) : value !== "" ? argList.push(flag, value): null)
    
 
    
    const [repairOutput, setRepairOutput] = useState([])
    const newOutput = `python3 ~/spun/repos/speedy/script/run.py ${argList.join(" ")}` 
    
    const executeCommand = async () => {
        setRepairOutput(await invoke(rustCommand, commandArgs))
    }

    return (
        <TaskSelectorDiv> 

            <h1 style={{margin: 0}}>{pageName}</h1>
            <OutputDiv>
                <h1>Forged Command : </h1>
                <h2> {newOutput} </h2>
            </OutputDiv>

            <form style={{display: "flex", flexDirection: "column", gap: "1rem"}} action="">
                {Object.entries(flagData).map((obj, index) => <FlagInput key={index} {...obj.at(1)} stateName={stateName} />)}
            </form>
                <StyledButton onClick={() => executeCommand()}>Forge </StyledButton>
        </TaskSelectorDiv>
    )
};


const TaskSelectorDiv = styled.div`
display: grid;
min-width: 50%;
max-width: 50rem;
grid-gap: 1rem;
grid-template-rows: 1fr auto;
`

const OutputDiv = styled.div`
border: 1px solid grey;
`
2