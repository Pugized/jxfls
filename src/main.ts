import { createApp } from "vue";
import App from "./App.vue";
import "./tailwind.css"
import { isTauri } from "@tauri-apps/api/core";

createApp(App).mount("#app");
console.log(isTauri());

export function isMobile() {
    return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
        navigator.userAgent
    );
}