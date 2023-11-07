import { configureStore } from "@reduxjs/toolkit";
import machineReducer from "./reducers/machineReducer";
import tmuxReducer from "./reducers/tmuxReducer";

export const store = configureStore({
    reducer: {
        machine: machineReducer,
        tmux: tmuxReducer,
    }
})
