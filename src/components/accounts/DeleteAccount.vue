<script setup lang="ts">
import {deleteAccount, ResponseType} from "../../composables/Commands.ts";

const emit = defineEmits(['success']);

const props = defineProps({
  accountId: {
    type: Number,
    required: true,
  },
})

  async function confirmDelete() {
    const response = await deleteAccount(props.accountId)

    if (response.response !== ResponseType.SUCCESS) {
      return;
    }

    emit('success');
  }
</script>

<template>
  <div class="d-grid gap-2">
    <button class="btn btn-danger" @click="confirmDelete">Delete</button>
  </div>
</template>

<style scoped>

</style>