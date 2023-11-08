import {createSlice} from "@reduxjs/toolkit"
import { useSelector } from "react-redux"

const buildReducer = createSlice({
    name: "build",

    initialState: {
        args : {},
    },

    reducers: {
        pushBuildArgs: (state, action) => {
           state.args[action.payload.label] = {flag: action.payload.flag, value: action.payload.output}
        },

    }
})

export default buildReducer.reducer;
export const {pushBuildArgs} = buildReducer.actions;

export const selectBuildArgs = (state) => state.build.args

