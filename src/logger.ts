import { invoke, isTauri } from "@tauri-apps/api/core";

export default class Logger {
    public static warn(scope: string, message: string) {
        console.log(`[WARN][${scope}] ${message}`);
        if (isTauri())
            invoke("logger_warn", { scope: `Web - ${scope}`, message });
    }
    public static info(scope: string, message: string) {
        console.log(`[INFO][${scope}] ${message}`);
        if (isTauri())
            invoke("logger_info", { scope: `Web - ${scope}`, message });
    }
    public static error(scope: string, message: string) {
        console.log(`[ERROR][${scope}] ${message}`);
        if (isTauri())
            invoke("logger_error", { scope: `Web - ${scope}`, message });
    }
    public static fatal(scope: string, message: string) {
        console.log(`[FATAL][${scope}] ${message}`);
        if (isTauri())
            invoke("logger_fatal", { scope: `Web - ${scope}`, message });
    }
}