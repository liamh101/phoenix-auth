<script setup lang="ts">
  import { onMounted } from 'vue'
  import {getOsTheme} from "../composables/OperatingSystem.ts";

  async function setupTheme() {
    const systemTheme = await getOsTheme();

    document.documentElement.setAttribute("data-theme", systemTheme);
    document.documentElement.setAttribute("data-bs-theme", systemTheme);
    localStorage.setItem('theme', systemTheme);
  }


  onMounted(() => setupTheme())

</script>

<template>

</template>

<style scoped lang="scss">

</style>