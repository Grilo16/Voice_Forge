import styled from "styled-components"
import { invoke } from "@tauri-apps/api";
import { useDispatch, useSelector } from "react-redux";
import { selectAllInstances, selectCredentials, setAllInstances, setCurrentCredentialsById } from "../../../../features/reducers/machineReducer";
import { useEffect } from "react";

export const DisplayInstanceStatus = () => {

    const credentials = useSelector(selectCredentials)
    const sshInstances = useSelector(selectAllInstances)

    const dispatch = useDispatch()
    const getAllInstances = async () => {
        const allInstances = await invoke("get_ssh_credentials");
        dispatch(setAllInstances(allInstances.data))
    }
        
    useEffect(() => {
        getAllInstances()
    }, [])

    const instanceOptions = sshInstances?.map((instance, index) => <StyledOption key={index} value={instance.id}>{instance.job_name}</StyledOption>)
    const data = {
        jobName: "s04vXu0Qv_repair",
        machineType:"t2.small",
        product: "TTS_deploy",
    }

    // "--machine t2.small --product TTS_deploy --name s04vXu0Qv_repair"

    const copyContent = async () => {
        try {
          await navigator.clipboard.writeText(credentials?.ssh_command);
          console.log('Content copied to clipboard');
        } catch (err) {
          console.error('Failed to copy: ', err);
        }
      }


    return (
        <DisplayInstanceDiv> 

        <InstanceSelectorDiv> 
            <LargerLabelh1>Available Instances</LargerLabelh1>
            <StyledSelect onChange={(e) => dispatch(setCurrentCredentialsById(e.target.value))}>
                {instanceOptions}
            </StyledSelect>
        </InstanceSelectorDiv>

        <InstanceDetailsDiv>

            <LargerLabelh1>Current Instance </LargerLabelh1>
            <InstanceInfoDiv> 
                <InfoLabelh2>Job Name:</InfoLabelh2>
                <DataContainerDiv> 
                    <InfoDataP>{credentials.job_name}</InfoDataP>
                </DataContainerDiv>
            </InstanceInfoDiv>
            <InstanceInfoDiv> 
                <InfoLabelh2>Machine Type:</InfoLabelh2>
                <DataContainerDiv> 
                    <InfoDataP>{credentials.machine_type}</InfoDataP>
                </DataContainerDiv>
            </InstanceInfoDiv>
            <InstanceInfoDiv> 
                <InfoLabelh2>Product:</InfoLabelh2>
                <DataContainerDiv> 
                    <InfoDataP>{credentials.product}</InfoDataP>
                </DataContainerDiv>
            </InstanceInfoDiv>
        </InstanceDetailsDiv>
            <StyledButton onClick={copyContent}>Copy ssh command</StyledButton>
        </DisplayInstanceDiv>
    )
}

const StyledButton = styled.button`
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

const StyledSelect = styled.select`
border-radius: 5px;
align-self: stretch;
margin: 0.25rem;
`
const StyledOption = styled.option`
text-align: center;
`


const InstanceSelectorDiv = styled.div`
background-color: #a7a8a9;
display: flex;
flex-direction: column;
justify-content: center;
align-items: center;
border-radius: 5px;
margin: 0.5rem;
padding: 0.2rem;
`

const InfoLabelh2 = styled.h2`
margin: 0 1rem 0.1rem;
font-size: 1.2rem;
justify-self: left;
font-weight: 500;
`
const LargerLabelh1 = styled.h1`
margin: 0.25rem 1rem 0;
font-size: 1.2rem;
justify-self: center;
font-weight: 600;
white-space: nowrap;
border-bottom: solid 1px black;
`

const InfoDataP = styled.p`
`

const DataContainerDiv = styled.div`
display: flex;
justify-content: center;
align-items: center;
background-color: #d7d8d9;
border-radius: 5px;
max-height: 2rem;
margin: 0 0.5rem 0.5rem;
`

const InstanceInfoDiv = styled.div`
display: grid;
`

const DisplayInstanceDiv = styled.div`
background-color: #f7f8f9;
min-height: 8rem;
display: flex;
flex-direction: column;
margin: 0.5rem;
border-radius: 5px;
margin-top: auto;
`

const InstanceDetailsDiv = styled.div`
background-color: #a7a8a9;
display: flex;
flex-direction: column;
justify-content: center;
border-radius: 5px;
margin: 0 0.5rem;
`