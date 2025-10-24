import { createApp } from "vue";
import App from "./App.vue";
import "./tailwind.css"

createApp(App).mount("#app");

export const client: boolean = Object(window).hasOwnProperty("__TAURI__")