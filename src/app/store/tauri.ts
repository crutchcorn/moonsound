import {createAsyncThunk, createSelector, createSlice, PayloadAction} from "@reduxjs/toolkit";
import { invoke } from "@tauri-apps/api/core";

export interface Duration {
    secs: number;
    nanos: number;
}

interface StateFromTauri {
    volume: number;
    speed: number;
    paused: boolean;
    "currently_playing_file_path": string | null;
    "currently_playing_duration": Duration | null;
}

// Create async thunk for playing music
export const musicSync = createAsyncThunk(
    "tauri/musicSync",
    async () => {
        return invoke<StateFromTauri>("get_redux_store_state")
    }
);

const initialState = {
    volume: 1,
    speed: 1,
    paused: true,
    position: null as Duration | null,
    currentlyPlayingPath: null as string | null,
    duration: null as Duration | null
}

// TODO: Rename to `playbackStateSlice`
export const tauriSlice = createSlice({
    name: "tauri",
    initialState,
    reducers: {
        setPosition: (state, action: PayloadAction<Duration>) => {
            state.position = action.payload
        }
    },
    extraReducers: (builder) => {
        builder.addCase(musicSync.fulfilled, (state, action) => {
            state.volume = action.payload.volume;
            state.speed = action.payload.speed;
            state.paused = action.payload.paused;
            state.currentlyPlayingPath = action.payload["currently_playing_file_path"];
            state.duration = action.payload["currently_playing_duration"];
        });
        // TODO: Add error handling
    }
});

export const {setPosition} = tauriSlice.actions;

export default tauriSlice.reducer;
