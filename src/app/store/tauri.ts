import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { invoke } from "@tauri-apps/api/core";

interface TauriState {
    volume: number;
    speed: number;
    paused: boolean;
    position: { "secs": number, "nanos": number };
    "currently_playing_file_path": string | null;
}

// Create async thunk for playing music
export const musicSync = createAsyncThunk(
    "tauri/musicSync",
    async () => {
        return invoke<TauriState>("get_redux_store_state")
    }
);

export const tauriSlice = createSlice({
    name: "tauri",
    initialState: {
        volume: 1,
        speed: 1,
        paused: true,
        position: {secs: 0, nanos: 0},
        currentlyPlaying: null as string | null,
    },
    reducers: {},
    extraReducers: (builder) => {
        builder.addCase(musicSync.fulfilled, (state, action) => {
            state.volume = action.payload.volume;
            state.speed = action.payload.speed;
            state.paused = action.payload.paused;
            state.position = action.payload.position;
            state.currentlyPlaying = action.payload["currently_playing_file_path"];
        });
        // TODO: Add error handling
    }
});

export default tauriSlice.reducer;