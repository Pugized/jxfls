import { createApp } from "vue";
import App from "./App.vue";
import "./tailwind.css"
import Logger from "./logger";

createApp(App).mount("#app");
Logger.info("Main", "Front-end started");

export function isMobile() {
    return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
        navigator.userAgent
    );
}