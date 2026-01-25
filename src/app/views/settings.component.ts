import { Component } from "@angular/core";
import { LightGlassOutlined } from "../components/light-glass-outlined";

@Component({
  selector: "app-settings",
  template: `
    <div class="container">
      <light-glass-outlined>
        <div class="innerContents">
          <h1 class="title">Settings</h1>
          <hr class="hr" />
        </div>
      </light-glass-outlined>
    </div>
  `,
  styles: [
    `
      .container {
        min-height: 100vh;
        /* TODO: Change to CSS variable */
        background: #060606;
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        padding: 2rem;
      }

      .container > light-glass-outlined {
        flex-grow: 1;
        height: 1px;
      }

      .innerContents {
        padding: 2rem;
      }

      .title {
        margin: 0;
        font-family: "Roboto Flex", sans-serif;
        font-variation-settings:
          "wght" 500,
          "wdth" 151,
          "GRAD" 150,
          "XOPQ" 96;
      }

      .hr {
        all: unset;
        display: block;
        height: 2px;
        width: 100%;
        background: rgba(255, 255, 255, 0.08);
        margin-top: 20px;
        margin-bottom: 20px;
      }

      p {
        margin: 0;
      }
    `,
  ],
  imports: [LightGlassOutlined],
})
export class Settings {}
