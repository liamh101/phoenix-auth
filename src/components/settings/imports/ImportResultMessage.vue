<script setup lang="ts">
import {computed} from "vue";

export interface ImportDetail {
  failed: number,
  attempted: number,
}

const props = defineProps({
  importDetails: {
    type: Object,
    required: true,
  },
})

const emit = defineEmits(['importAnotherFile', 'importAccepted'])


const importMessage = computed(function () {
  if (props.importDetails.attempted === props.importDetails.failed) {
    return 'All Accounts Failed To Import'
  }

  if (props.importDetails.failed) {
    return props.importDetails.failed + '/' + props.importDetails.attempted + ' Failed To Import';
  }

  return 'All Accounts Imported Successfully'
})

function backToAccount() {
  return emit('importAccepted')
}

function importAnotherFile() {
  return emit('importAnotherFile')
}
</script>

<template>
  <div>
    <div class="container-fluid">
      <h2
        class="text-center"
        v-text="importMessage"
      />

      <div class="row">
        <div class="col text-center">
          <button
            class="btn btn-primary"
            @click="backToAccount"
          >
            Back To Accounts
          </button>
        </div>

        <div class="col text-center">
          <button
            class="btn btn-primary"
            @click="importAnotherFile"
          >
            Import Another File
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">

</style>