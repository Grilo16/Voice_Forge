import { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import styled from "styled-components";
import { pushArgs, selectArgs } from "../../features/reducers/machineReducer";
import { pushBuildArgs, selectBuildArgs } from "../../features/reducers/buildReducer";
import { pushBlendArgs, selectBlendArgs } from "../../features/reducers/blendReducer";
import { pushRepairArgs, selectRepairArgs } from "../../features/reducers/repairReducer";

export const FlagInput = ({label,  flag,  type,  altFlags,  required,  options, stateName,}) => {
    
    
    const dispatch = useDispatch();
    const [selectedFlag, setSelectedFlag] = useState({label: "", flag: "", type: "",});
    const pushToState = stateName === "build" 
                        ? pushBuildArgs 
                        : stateName === "blend"
                        ? pushBlendArgs
                        : stateName === "repair"
                        ? pushRepairArgs
                        : pushArgs
    const args = stateName === "build" 
                ? useSelector(selectBuildArgs) 
                : stateName === "blend"
                ? useSelector(selectBlendArgs)
                : stateName === "repair"
                ? useSelector(selectRepairArgs)
                : useSelector(selectArgs)

    let argValue = ""; 


    useEffect(() => {
        if (stateName && !args[selectedFlag[label]]?.flag === ""){
            dispatch(pushToState({label: label, flag: flag, output: ""}))
        }
        setSelectedFlag({label : label, flag: flag, type: type, required: required, options: options})
    }, [])

    if (stateName) {
        argValue = args[selectedFlag.label]?.value ?? ""
    } else {
        argValue = args[flag]
    }

    const altFlagsSelectors = (
        altFlags.map((flag, index) => (
            <ToggleFlag key={index} onClick={()=>setSelectedFlag({...flag})} selected >
                {flag.altLabel}
            </ToggleFlag>
        ))
    )

  
  if (selectedFlag.type === "dropdown") {
   
    const displayOptions = selectedFlag.options?.map((option, index) => (
      <StyledOption key={index}>{option}</StyledOption>
    ));

    return (
      <StyledLabel>
        <div style={{ display: "flex", gap: "1rem", alignItems: "center" }}>
          <ToggleFlag onClick={()=> altFlags.length ? (setSelectedFlag({label : label, flag: flag, type: type, required: required, options: options}), dispatch(pushToState({label: selectedFlag.label, flag: selectedFlag.flag, output: ""}))) : null} selected>
            {label}
          </ToggleFlag>
          {altFlagsSelectors}
        </div>
        <StyledSelect onChange={(e) => dispatch(pushToState({label: selectedFlag.label, flag: selectedFlag.flag, output: e.target.value}))}>
        <StyledOption></StyledOption>
            {displayOptions}
        </StyledSelect>
      </StyledLabel>
    );
  }

  if (selectedFlag.type === "checkbox") {
    return (
        <StyledLabel>
        <div style={{ display: "flex", gap: "1rem", alignItems: "center" }}>
          <ToggleFlag  onClick={()=> altFlags.length ? (setSelectedFlag({label : label, flag: flag, type: type, required: required, options: options}), dispatch(pushToState({label: selectedFlag.label, flag: selectedFlag.flag, output: ""}))) : null} selected>
            {label}
            </ToggleFlag>
          {altFlagsSelectors}
        </div>
        <StyledInput
          type={selectedFlag.type}
          required={selectedFlag?.required}
          checked={argValue}
          onChange={(e) => (dispatch(pushToState({label: selectedFlag.label, flag: selectedFlag.flag, output: e.target.checked ? " " : ""})))}
          />
      </StyledLabel>
    )
  }

  return (
    <StyledLabel>
      <div style={{ display: "flex", gap: "1rem", alignItems: "center" }}>
        <ToggleFlag onClick={()=> altFlags.length ? (setSelectedFlag({label : label, flag: flag, type: type, required: required, options: options}), dispatch(pushToState({label: selectedFlag.label, flag: selectedFlag.flag, output: ""}))) : null} selected>
          {label}
          </ToggleFlag>
          {altFlagsSelectors}
      </div>
      <StyledInput
        type={selectedFlag.type}
        required={selectedFlag?.required}
        value={argValue}
        onChange={(e) => dispatch(pushToState({label: selectedFlag.label, flag: selectedFlag.flag, output: e.target.value}))}
        />
    </StyledLabel>
  );
};

const StyledSelect = styled.select`
  border-radius: 5px;
  align-self: stretch;
  min-height: 2rem;
`;
const StyledOption = styled.option`
  text-align: center;
`;

const StyledLabel = styled.label`
  display: grid;
  grid-template-columns: 1fr 1fr;
  justify-content: stretch;
  grid-template-rows: 2rem;
`;

const ToggleFlag = styled.div`
  background-color: ${({ selected }) => (selected ? "#A7A8A9" : "#D7D8D9")};
  min-width: 8rem;
  border-radius: 5px;
  display: flex;
  justify-content: center;
`;

const StyledInput = styled.input`
text-align: center;
`;
