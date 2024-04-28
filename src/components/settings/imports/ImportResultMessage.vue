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


const importMessage = computed(function () {

  if (props.importDetails.attempted === props.importDetails.failed) {
    return 'All Accounts Failed To Import'
  }

  if (props.importDetails.failed) {
    return props.importDetails.failed + '/' + props.importDetails.attempted + ' Failed To Import';
  }

  return 'All Accounts Imported Successfully'
})
</script>

<template>
  <div>
    <div>
      <h2
        class="text-center"
        v-text="importMessage"
      />
    </div>
  </div>
</template>

<style scoped lang="scss">

</style>