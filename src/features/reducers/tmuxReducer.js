import {createSlice} from "@reduxjs/toolkit"
import { useSelector } from "react-redux"

const tmuxReducer = createSlice({
    name: "tmux",

    initialState: {
        args : {},
        repairFlags: [],
    },

    reducers: {
        pushTmuxArgs: (state, action) => {
           state.args[action.payload.label] = {flag: action.payload.flag, value: action.payload.output}
        },

    }
})

export default tmuxReducer.reducer;
export const {pushTmuxArgs} = tmuxReducer.actions;

export const selectTmuxArgs = (state) => state.tmux.args