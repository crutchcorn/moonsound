import { createAsyncThunk, createSelector, createSlice } from "@reduxjs/toolkit";
import { invoke } from "@tauri-apps/api/core";

interface StateFromTauri {
    volume: number;
    speed: number;
    paused: boolean;
    "currently_playing_file_path": string | null;
}

interface Position {
    secs: number;
    nanos: number;
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
        return invoke<Position>("get_position")
    }
);

const initialState = {
    volume: 1,
    speed: 1,
    paused: true,
    position: { secs: 0, nanos: 0 },
    currentlyPlaying: null as string | null,
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
        });
        builder.addCase(getPosition.fulfilled, (state, action) => {
            state.position = action.payload;
        });
        // TODO: Add error handling
    }
});

export default tauriSlice.reducer;