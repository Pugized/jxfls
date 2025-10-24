<script setup lang="ts">
    import logo from "./assets/jxfls.png"
</script>

<template>
    <div class="draggable h-20 w-full bg-white absolute">
        <img v-bind:src="logo" alt="" class="scale-15 absolute ml-1 mt-auto ms-auto">
    </div>
</template>

<style scoped>
    div.draggable {
        --webkit-app-region: drag;
    }
</style>