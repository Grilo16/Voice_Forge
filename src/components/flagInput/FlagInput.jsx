import { useEffect, useState } from "react"
import styled from "styled-components"

export const FlagInput = ({label, flag, type, altFlags, required, setOutput }) => {

    const [selectedFlag, setSelectedFlag] = useState({})
  
    
    const [flagValue, setFlagValue] = useState("")
    const output = {
        flag: `${selectedFlag.flag ?? flag} `,
        value: `${flagValue} `,
        type: type
    }

    useEffect(() => {
                setOutput(current => ({...current, [label]: {...output}}))
    }, [flagValue])


    return (
        <StyledLabel> 
            <div style={{display: "flex", gap: "1rem", alignItems:"center"}}>
            <ToggleFlag onClick={()=>(setSelectedFlag({}), altFlags.length ? flagValue === " " ?  setFlagValue(""): null : null)} selected={selectedFlag.label ? false : true}>{label}</ToggleFlag>
            {altFlags.map(flag => <ToggleFlag onClick={()=>(setSelectedFlag(flag),altFlags.length ? flagValue === " " ?  setFlagValue(""): null : null)} selected={selectedFlag.label === flag.label? true: false}>{flag.label}</ToggleFlag>)}
            </div> 
        <StyledInput  type={selectedFlag.type ?? type} 
                value={flagValue}
                required={required} 
                onChange={(e) => !selectedFlag.type 
                        ? type === "checkbox" 
                            ? setFlagValue(e.target.checked ? " " : "") 
                            : setFlagValue(e.target.value)
                        : selectedFlag.type === "checkbox"
                        ? setFlagValue(e.target.checked ? " " : "") 
                        : setFlagValue(e.target.value) } 
                />
        </StyledLabel>
    )
}

const StyledLabel = styled.label`
display: grid;
grid-template-columns: 1fr 1fr ;
justify-content: stretch;
grid-template-rows: 2rem;

`

const ToggleFlag = styled.div`
background-color: ${({selected}) => selected ? "#A7A8A9" : "#D7D8D9"};
min-width: 8rem;
border-radius: 5px;
display: flex;
justify-content: center;

`

const StyledInput = styled.input`
min-height: 1rem;

`