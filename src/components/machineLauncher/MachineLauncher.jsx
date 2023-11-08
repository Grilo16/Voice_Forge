import { useState } from "react";
import { FlagInput } from "../flagInput";
import { invoke } from "@tauri-apps/api";
import { useDispatch, useSelector } from "react-redux";
import { selectArgs, selectCredentials, setAllInstances, setCredentials } from "../../features/reducers/machineReducer";
import styled from "styled-components";

export const MachineLauncher = () => {
    // "--machine t2.small --product TTS_deploy --name s04vXu0Qv_repair"
    const launchArgs = useSelector(selectArgs)
    const credentials = useSelector(selectCredentials)

    const repairArgs = {
        n: {
            label: "Job name",
            flag: "name",
            type: "text",
            required: true,
            altFlags : [],
        }, 
        i: {
            label: "Machine type",
            flag: "machine",
            type: "dropdown",
            options: [ "g4dn.xlarge", "t2.small", "t2.medium", "p3.xlarge"],
            required: true,
            altFlags : [],
        }, 
        d: {
            label: "Product",
            flag: "product",
            type: "dropdown",
            options: ["TTS_research", "TTS_deploy"],
            required: true,
            altFlags : [],
        }, 
    }

    const dispatch = useDispatch()
    let argList = [] 
    const [outputObj, setOutputObj] = useState([])
    const [result, setResult] = useState({})
    const instanceFlags = {instanceFlags: argList}
    const launchInstance = async () => {
        const result = await invoke("launch_instance", launchArgs);
        const allInstances = await invoke("get_ssh_credentials");
        dispatch(setAllInstances(allInstances?.data))
        setResult(result?.data);
        dispatch(setCredentials(result?.data));
    };
    return (
        <InputDiv>
            {Object.entries(repairArgs).map((obj) => <FlagInput {...obj.at(1)} setOutput={setOutputObj} />)}
            <StyledButton onClick={() => launchInstance()}>Launch Instance</StyledButton>
        </InputDiv>
    )
};

export const StyledButton = styled.button`
margin: 0.5rem;
max-height: 2rem;
min-height: 2rem;
padding: 0.1rem;
background: linear-gradient(142deg, rgba(239,141,116,1) 0%, rgba(229,47,127,1) 100%);
color: white;
border: none;
&: hover {
    box-shadow: 0rem 0rem 0.4rem 0rem grey;
}

`

const InputDiv = styled.div`
display: grid;
grid-auto-rows: 2.5rem;
`

const StyledSelect = styled.select`
border-radius: 5px;
align-self: stretch;
margin: 0.25rem;
`
const StyledOption = styled.option`
text-align: center;
`