import { useState } from "react";
import { FlagInput } from "../flagInput";
import { invoke } from "@tauri-apps/api";
import { useDispatch, useSelector } from "react-redux";
import { selectArgs, selectCredentials, setAllInstances, setCredentials } from "../../features/reducers/machineReducer";

export const MachineLauncher = () => {
    // "--machine t2.small --product TTS_deploy --name s04vXu0Qv_repair"
    const launchArgs = useSelector(selectArgs)
    const credentials = useSelector(selectCredentials)

    const repairArgs = {
        i: {
            label: "Machine type",
            flag: "machine",
            type: "text",
            required: true,
            altFlags : [],
        }, 
        d: {
            label: "Product",
            flag: "product",
            type: "text",
            required: true,
            altFlags : [],
        }, 
        n: {
            label: "Job name",
            flag: "name",
            type: "text",
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
        <>
        
        <p>{credentials?.username}</p>
        <p>{credentials?.host}</p>
        <p>{credentials?.ssh_command}</p>   
         {Object.entries(repairArgs).map((obj) => <FlagInput {...obj.at(1)} setOutput={setOutputObj} />)}
        <button onClick={() => launchInstance()}>Launch Instance</button>
        </>
    )
};