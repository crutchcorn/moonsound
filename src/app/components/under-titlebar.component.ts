import { Component, input } from "@angular/core";
import { platform } from '@tauri-apps/plugin-os';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { injectQuery } from "@tanstack/angular-query-experimental";

@Component({
    selector: "app-under-titlebar-macos",
    template: `
        <div class="under-titlebar-macos">
            <img class="logo" [src]="theme() === 'light' ? 'assets/header-logo-black.svg' : 'assets/header-logo-white.svg'" alt="" height="32" width="32"/>
        </div>
    `,
    styles: [`
        .under-titlebar-macos {
            display: flex;
            flex-direction: row;
            justify-content: flex-end;
        }
        
        .logo {
            opacity: 0.5;
            mix-blend-mode: plus-lighter;
        }
    `]
})
export class UnderTitlebarMacos {
    theme = input<'light' | 'dark'>();
}

@Component({
    selector: "app-under-titlebar",
    imports: [UnderTitlebarMacos],
    template: `
        @if (currentPlatform === 'macos') {
            <app-under-titlebar-macos [theme]="theme.data() ?? 'dark'"/>
        } @else {
            <div style="height: 32px"></div>
        }
    `
})
export class UnderTitlebar {
    currentPlatform = platform();
    theme = injectQuery(() => ({
        queryKey: ['theme'],
        queryFn: async () => await getCurrentWindow().theme() ?? "dark"
    }))
}