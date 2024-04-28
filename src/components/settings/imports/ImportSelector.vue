<script setup lang="ts">

import {computed, ref} from "vue";
import {parseWaFile} from "../../../composables/Importers.ts";
import {ResponseType} from "../../../composables/Commands.ts";

enum IMPORT_TYPE {
  NONE,
  WA
}

const importType = ref(IMPORT_TYPE.NONE)

const emit = defineEmits(['importedAccounts'])

const accept = computed(function () {
  switch (importType.value) {
    case IMPORT_TYPE.WA:
      return '.wa.txt';
    default:
      return '';
  }
})

async function getFile(event: Event) {
  const file = event.target.files.item(0)
  const fileContents = await file.text();

  if (importType.value === IMPORT_TYPE.WA) {
    return importWaFile(fileContents);
  }
}

async function importWaFile(file: string) {
  const potentialAccounts = parseWaFile(file)
  const accounts = [];

  for (const potentialAccount of potentialAccounts) {
    const response = await potentialAccount;

    if (response.response === ResponseType.SUCCESS) {
      accounts.push(response.account);
    }
  }

  emit('importedAccounts',accounts)
}


</script>

<template>
  <select
    v-model="importType"
    class="form-select"
    aria-label="Select Import Type"
  >
    <option
      :value="IMPORT_TYPE.NONE"
      selected
    >
      Available Importers
    </option>
    <option :value="IMPORT_TYPE.WA">
      WA
    </option>
  </select>

  <input
    v-if="importType !== IMPORT_TYPE.NONE"
    class="mt-3"
    type="file"
    :accept="accept"
    @change="getFile"
  >
</template>

<style scoped lang="scss">

</style>