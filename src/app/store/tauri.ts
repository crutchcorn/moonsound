import { createAsyncThunk, createSelector, createSlice } from "@reduxjs/toolkit";
import { invoke } from "@tauri-apps/api/core";

interface Duration {
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

export const getPosition = createAsyncThunk(
    "tauri/getPosition",
    async () => {
        return invoke<Duration>("get_position")
    }
);

const initialState = {
    volume: 1,
    speed: 1,
    paused: true,
    position: null as Duration | null,
    currentlyPlaying: null as string | null,
    duration: null as Duration | null
}

export const tauriSlice = createSlice({
    name: "tauri",
    initialState,
    reducers: {},
    extraReducers: (builder) => {
        builder.addCase(musicSync.fulfilled, (state, action) => {
            state.volume = action.payload.volume;
            state.speed = action.payload.speed;
            state.paused = action.payload.paused;
            state.currentlyPlaying = action.payload["currently_playing_file_path"];
            state.duration = action.payload["currently_playing_duration"];
        });
        builder.addCase(getPosition.fulfilled, (state, action) => {
            state.position = action.payload;
        });
        // TODO: Add error handling
    }
});

export default tauriSlice.reducer;