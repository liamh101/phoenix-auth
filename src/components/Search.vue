<script setup lang="ts">
  import {ref} from "vue";

  const emit = defineEmits(['updated'])

  const search = ref('')

  function onUpdated() {
    emit('updated', search.value)
  }

</script>

<template>
  <input v-model="search" @change="onUpdated" class="form-control" type="search" placeholder="Search">
</template>