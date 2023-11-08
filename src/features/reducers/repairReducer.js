import {createSlice} from "@reduxjs/toolkit"
import { useSelector } from "react-redux"

const repairReducer = createSlice({
    name: "repair",

    initialState: {
        args : {},
    },

    reducers: {
        pushRepairArgs: (state, action) => {
           state.args[action.payload.label] = {flag: action.payload.flag, value: action.payload.output}
        },

    }
})

export default repairReducer.reducer;
export const {pushRepairArgs} = repairReducer.actions;

export const selectRepairArgs = (state) => state.repair.args

