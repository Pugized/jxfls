<script setup lang="ts">
    import { invoke, isTauri } from "@tauri-apps/api/core";
    import { isMobile } from "./main.ts";
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

    const onHover = (id: string) => {
        const element = document.getElementById(id);
        if (element !== null)
            element.className = "h-3 w-3 icon-hover p-0 rounded-[10px] relative content-center justify-center w-[90%] h-[90%]";
    }
    const onOut = (id: string) => {
        const element = document.getElementById(id);
        if (element !== null)
            element.className = "h-3 w-3 icon p-0 rounded-[10px] relative content-center justify-cente w-[90%] h-[90%]";
    }
    const onPressed = (id: string) => {
        const element = document.getElementById(id);
        if (element !== null)
            element.className = "h-3 w-3 pressed p-0 rounded-[10px] relative content-center justify-center w-[90%] h-[90%]";
    }
    const onClose = async () => {
        if (!isTauri())
            return;
        invoke("close", { code: 0 })
        
    }
    const onMinimize = async () => {
        if (!isTauri())
            return;
        await getCurrentWebviewWindow().minimize();
    }
</script>

<template>
    <div v-if="isTauri() && !isMobile()" class="flex flex-row">
        <div id="red-light" class="h-3 w-3 rounded-[10px] bg-[#FF5F57] red-light mt-auto mb-auto ml-2 border-[0.5px] border-black/20 relative p-0" @mouseover="onHover('close')" @mouseout="onOut('close')" @mousedown="onPressed('close')" @click="onClose">
            <div id="close" class="icon p-0 rounded-[10px] absolute content-center justify-center m-0 w-[90%] h-[90%]">
                <svg xmlns="http://www.w3.org/2000/svg" class="rounded-[10px] relative content-center justify-center" viewBox="0 0 8 8">
                    <g stroke="#000000" stroke-width="1" stroke-linecap="round">
                        <line x1="2.5" y1="2.5" x2="6.5" y2="6.5"/>
                        <line x1="6.5" y1="2.5" x2="2.5" y2="6.5"/>
                    </g>
                </svg>
            </div>
        </div>
        <div id="yellow-light" class="h-3 w-3 rounded-[10px] bg-[#FEBC2E] yellow-light mt-auto mb-auto ml-2 border-[0.5px] z-10 border-black/20 relative" @mouseover="onHover('minimize')" @mouseout="onOut('minimize')" @close="onMinimize">
            <div id="minimize" class="icon p-0 rounded-[10px] absolute content-center justify-center m-0 w-[90%] h-[90%]">
                <svg xmlns="http://www.w3.org/2000/svg" class="rounded-[10px] relative content-center justify-center" viewBox="0 0 8 8">
                    <g stroke="#000000" stroke-width="1" stroke-linecap="round">
                        <line x1="2" y1="4.5" x2="7" y2="4.5"/>
                    </g>
                </svg>
            </div>
        </div>
    </div>
</template>

<style scoped>
    .icon {
        opacity: 0;
    }
    .icon-hover {
        opacity: 40%;
    }
    .red-light {
        background-color: #FF5F57;
        -webkit-app-region: no-drag;
    }
    .red-light:hover {
        background-color: #FF7B72;
        -webkit-app-region: no-drag;
    }
    .yellow-light {
        background-color: #FEBC2E;
        -webkit-app-region: no-drag;
    }
    .yellow-light:hover {
        background-color: #FFD460;
        -webkit-app-region: no-drag;
    }
    .red-light:active:hover {
        background-color: #E2463E;
        -webkit-app-region: no-drag;
    }
    .yellow-light:active:hover {
        background-color: #E1A21A;
        -webkit-app-region: no-drag;
    }
</style>