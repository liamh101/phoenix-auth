<script setup lang="ts">
import {Ref, ref} from "vue";
import {DraftAccount} from "../../../composables/Commands.ts";
import ImportTable from "./ImportTable.vue";
import ImportSelector from "./ImportSelector.vue";
import ImportResultMessage, {ImportDetail} from "./ImportResultMessage.vue";

const proposedAccounts = ref([]) as Ref<DraftAccount[]>
const resultDetails = ref(null) as Ref<ImportDetail|null>

function setProposedAccounts(newAccounts: Array<DraftAccount>) {
  proposedAccounts.value = newAccounts
}

function setResultDetails(importDetails: ImportDetail) {
  resultDetails.value = importDetails
}

</script>

<template>
  <div class="card overflow-auto">
    <import-selector
      v-show="!resultDetails"
      @imported-accounts="setProposedAccounts"
    />

    <import-table
      v-show="!resultDetails"
      class="mt-3"
      :accounts="proposedAccounts"
      @complete="setResultDetails"
    />

    <import-result-message
      v-if="resultDetails"
      :import-details="resultDetails"
    />
  </div>
</template>

<style scoped lang="scss">

</style>