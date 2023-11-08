import { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import styled from "styled-components";
import { pushArgs, selectArgs } from "../../features/reducers/machineReducer";
import {
  pushTmuxArgs,
  selectTmuxArgs,
} from "../../features/reducers/tmuxReducer";

export const FlagInput = ({
  label,
  flag,
  type,
  altFlags,
  required,
  setOutput,
  tmux,
  options,
}) => {
  const args = tmux ? useSelector(selectTmuxArgs) : useSelector(selectArgs);
  const [selectedFlag, setSelectedFlag] = useState({});

  const [flagValue, setFlagValue] = useState("");
  const output = {
    flag: `${selectedFlag.flag ?? flag} `,
    value: `${flagValue} `,
    type: type,
  };

  const dispatch = useDispatch();

  useEffect(() => {
    setOutput((current) => ({ ...current, [label]: { ...output } }));
  }, [flagValue]);

  if (type === "dropdown") {
    const displayOptions = options?.map((option) => (
      <StyledOption>{option}</StyledOption>
    ));


    return (
      <StyledLabel>
        <div style={{ display: "flex", gap: "1rem", alignItems: "center" }}>
          <ToggleFlag selected>
            {label}
          </ToggleFlag>
        </div>
        <StyledSelect onChange={(e) => dispatch(pushArgs({ flag: flag, output: e.target.value }))}>
        <StyledOption></StyledOption>
            {displayOptions}
        </StyledSelect>
      </StyledLabel>
    );
  }

  return (
    <StyledLabel>
      <div style={{ display: "flex", gap: "1rem", alignItems: "center" }}>
        <ToggleFlag
          onClick={() => (
            setSelectedFlag({}),
            altFlags.length
              ? flagValue === " "
                ? setFlagValue("")
                : null
              : null
          )}
          selected={selectedFlag.label ? false : true}
        >
          {label}
        </ToggleFlag>
        {altFlags.map((flag) => (
          <ToggleFlag
            onClick={() => (
              setSelectedFlag(flag),
              altFlags.length
                ? flagValue === " "
                  ? setFlagValue("")
                  : null
                : null
            )}
            selected={selectedFlag.label === flag.label ? true : false}
          >
            {flag.label}
          </ToggleFlag>
        ))}
      </div>
      <StyledInput
        type={selectedFlag.type ?? type}
        required={required}
        value={args[label]?.value}
        onChange={(e) =>
          dispatch(
            tmux
              ? pushTmuxArgs({
                  label: label,
                  flag: flag,
                  output: e.target.value,
                })
              : pushArgs({ flag: flag, output: e.target.value })
          )
        }
        // onChange={(e) => !selectedFlag.type
        //         ? type === "checkbox"
        //             ? setFlagValue(e.target.checked ? " " : "")
        //             : setFlagValue(e.target.value)
        //         : selectedFlag.type === "checkbox"
        //         ? setFlagValue(e.target.checked ? " " : "")
        //         : setFlagValue(e.target.value) }
      />
    </StyledLabel>
  );
};

const StyledSelect = styled.select`
  border-radius: 5px;
  align-self: stretch;
  min-height: 2rem;
  min-width: 100%;
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

const StyledInput = styled.input``;
