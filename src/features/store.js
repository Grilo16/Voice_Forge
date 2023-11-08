import { configureStore } from "@reduxjs/toolkit";
import machineReducer from "./reducers/machineReducer";
import tmuxReducer from "./reducers/tmuxReducer";
import repairReducer from "./reducers/repairReducer";
import buildReducer from "./reducers/buildReducer";
import blendReducer from "./reducers/blendReducer";

export const store = configureStore({
    reducer: {
        machine: machineReducer,
        repair: repairReducer,
        build: buildReducer,
        blend: blendReducer,
        tmux: tmuxReducer,
    }
})
