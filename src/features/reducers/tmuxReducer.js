import {createSlice} from "@reduxjs/toolkit"
import { useSelector } from "react-redux"

const tmuxReducer = createSlice({
    name: "tmux",

    initialState: {
        args : {},
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



// python3 ~/spun/repos/speedy/script/run.py --donorvb -i asdfasdf -d asfasf -n sadfasdf --accent sadfasdf --vb --donorpreproc voicefixer --dry --donorid asfasf --preproc postfish --superdry