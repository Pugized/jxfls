<script setup lang="ts">
    import logo from "./assets/jxfls.png"
    import { isTauri } from "@tauri-apps/api/core";
    import TrafficLight from "./TrafficLight.vue";
    let bar_class = "h-15 w-full bg-white pl-0 absolute border-b-1 border-slate-300 basis-2 flex flex-row " + (isTauri() ? "draggable" : "");
</script>

<template>
    <div v-bind:class="bar_class">
        <TrafficLight class="ml-1 "/>
        <img v-bind:src="logo" alt="" class="h-10 relative mt-2 ml-3"/>
    </div>
</template>

<style scoped>
    div.draggable {
        -webkit-app-region: drag;
    }
</style>