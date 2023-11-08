import {createSlice} from "@reduxjs/toolkit"
import { useSelector } from "react-redux"

const machineReducer = createSlice({
    name: "machine",

    initialState: {
        args : {
            machine: "",
            product: "",
            name: "",
        },
        sshCredentials : {
            id: null,
            username: null,
            host: null,
            job_name: null,
            machine_type: null, 
            product: null,
            ssh_command: null
        },
        instances: []
    },

    reducers: {
        pushArgs: (state, action) => {
           state.args[action.payload.flag] = action.payload.output
        },

        setCredentials: (state, action) => {
            state.sshCredentials.id = action.payload?.id
            state.sshCredentials.username = action.payload?.username
            state.sshCredentials.host = action.payload?.host
            state.sshCredentials.job_name = action.payload?.job_name
            state.sshCredentials.machine_type = action.payload?.machine_type
            state.sshCredentials.product = action.payload?.product
            state.sshCredentials.ssh_command = action.payload?.ssh_command
        },
        
        setAllInstances: (state, action) => {
           state.instances = [...action.payload]
        },

        setCurrentCredentialsById: (state, action) => {
            state.sshCredentials = {...state.instances.filter(credentials => credentials.id == action.payload).at(0)}
        }

        
    }
})

export default machineReducer.reducer;
export const {pushArgs, setCredentials, setAllInstances, setCurrentCredentialsById} = machineReducer.actions;

export const selectArgs = (state) => state.machine.args
export const selectAllInstances = (state) => state.machine.instances
export const selectCredentials = (state) => state.machine.sshCredentials