<script setup lang="ts">

</script>

<template>
  <header class="header nav-fill">
    <div class="container-fluid h-100">
      <slot />
    </div>
  </header>
</template>

<style>

.header {
  position: fixed;
  top: 0;
  max-height: 3.5rem;
  width: 100%;
}

</style>
