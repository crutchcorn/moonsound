import { configureStore } from "@reduxjs/toolkit";
import tauriReducer from "./tauri";

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>;
// Inferred type: {counter: CounterState}
export type AppDispatch = typeof store.dispatch;

const store = configureStore({
  reducer: {
    tauri: tauriReducer,
  },
});

export default store;
