import {createSlice} from "@reduxjs/toolkit"
import { useSelector } from "react-redux"

const machineReducer = createSlice({
    name: "machine",

    initialState: {
        args : {},
        sshCredentials : {
            id: null,
            username: null,
            host: null,
            ssh_command: null
        }
    },

    reducers: {
        pushArgs: (state, action) => {
           state.args[action.payload.label] = {flag: action.payload.flag, value: action.payload.output}
        },

        setCredentials: (state, action) => {
            state.sshCredentials.id = action.payload?.id
            state.sshCredentials.username = action.payload?.host
            state.sshCredentials.host = action.payload?.username
            state.sshCredentials.ssh_command = action.payload?.ssh_command
        }


    }
})

export default machineReducer.reducer;
export const {pushArgs, setCredentials} = machineReducer.actions;

export const selectArgs = (state) => state.machine.args
export const selectCredentials = (state) => state.machine.sshCredentials