import {
  ApplicationConfig,
  provideZonelessChangeDetection,
} from "@angular/core";
import { provideRouter } from "@angular/router";

import { routes } from "./app.routes";
import {
  provideQueryClient,
  QueryClient,
} from "@tanstack/angular-query-experimental";
import { provideRedux } from "@reduxjs/angular-redux";
import store from "./store";

export const queryClient = new QueryClient();

export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(routes),
    provideZonelessChangeDetection(),
    provideQueryClient(queryClient),
    provideRedux({ store }),
  ],
};
