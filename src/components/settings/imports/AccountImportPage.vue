<script setup lang="ts">
import {Ref, ref} from "vue";
import {DraftAccount} from "../../../composables/Commands.ts";
import ImportTable from "./ImportTable.vue";
import ImportSelector from "./ImportSelector.vue";
import ImportResultMessage, {ImportDetail} from "./ImportResultMessage.vue";

const emit = defineEmits(['goBackToAccounts'])

const proposedAccounts = ref([]) as Ref<DraftAccount[]>
const resultDetails = ref(null) as Ref<ImportDetail|null>

function setProposedAccounts(newAccounts: Array<DraftAccount>) {
  proposedAccounts.value = newAccounts
}

function setResultDetails(importDetails: ImportDetail) {
  resultDetails.value = importDetails
}

function resetImport() {
  proposedAccounts.value = [];
  resultDetails.value = null;
}

function backToAccounts() {
  emit('goBackToAccounts')
}

</script>

<template>
  <div class="overflow-auto">
    <import-selector
      v-if="!resultDetails"
      class="selector-container"
      @imported-accounts="setProposedAccounts"
    />

    <import-table
      v-if="!resultDetails"
      class="mt-3"
      :accounts="proposedAccounts"
      @complete="setResultDetails"
    />

    <import-result-message
      v-if="resultDetails"
      class="mt-5"
      :import-details="resultDetails"

      @import-accepted="backToAccounts"
      @import-another-file="resetImport"
    />
  </div>
</template>

<style scoped lang="scss">
  .card {
    border-bottom: none;
  }

  .selector-container {
    padding-left: 5px;
    padding-right: 5px;
  }
</style>