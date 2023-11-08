import {createSlice} from "@reduxjs/toolkit"
import { useSelector } from "react-redux"

const blendReducer = createSlice({
    name: "blend",

    initialState: {
        args : {},
    },

    reducers: {
        pushBlendArgs: (state, action) => {
           state.args[action.payload.label] = {flag: action.payload.flag, value: action.payload.output}
        },

    }
})

export default blendReducer.reducer;
export const {pushBlendArgs} = blendReducer.actions;

export const selectBlendArgs = (state) => state.blend.args


