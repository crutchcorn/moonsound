import { Component } from "@angular/core";

@Component({
  selector: "light-glass-outlined",
  template: `
    <div class="innerContentsContainer">
      <ng-content></ng-content>
    </div>
  `,
  styles: [
    `
      :host {
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        padding: 2px;
        border-radius: 48px;
        background: linear-gradient(
          160deg,
          rgba(255, 255, 255, 0.2) 0%,
          rgba(255, 255, 255, 0) 41%,
          rgba(255, 255, 255, 0) 57%,
          rgba(255, 255, 255, 0.1) 100%
        );
      }

      .innerContentsContainer {
        flex-grow: 1;
        height: 1px;
        /* rgba(255,255,255,0.03) on top of #060606 */
        background: #131316;
        border-radius: calc(48px - 2px);
      }
    `,
  ],
})
export class LightGlassOutlined {}
