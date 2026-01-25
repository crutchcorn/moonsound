import { Component } from "@angular/core";
import { LightGlassOutlined } from "../components/light-glass-outlined";

@Component({
  selector: "app-settings",
  template: `
    <div class="container">
      <light-glass-outlined>
        <div class="innerContents">
          <p>This is a placeholder for Moonsound settings</p>
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

      p {
        margin: 0;
      }
    `,
  ],
  imports: [LightGlassOutlined],
})
export class Settings {}
