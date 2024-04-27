<script setup lang="ts">
import {computed, Ref, ref} from "vue";
import {parseWaFile} from "../../composables/Importers.ts";
import {AccountAlgorithm, createNewAccount, DraftAccount, ResponseType} from "../../composables/Commands.ts";

enum IMPORT_TYPE {
    NONE,
    WA
  }

  interface displayEditor {
    [key: number]: boolean | undefined
  }

  const importType = ref(IMPORT_TYPE.NONE)
  const draftAccounts = ref([]) as Ref<DraftAccount[]>
  const displayNameEditor = ref({}) as Ref<displayEditor>
  const selectAll = ref(true)

  const failedImports = ref([])
  const importComplete = ref(false)

  const accept = computed(function () {
    switch (importType.value) {
      case IMPORT_TYPE.WA:
        return '.wa.txt';
      default:
        return '';
    }
  })

  const importMessage = computed(function () {
    const approvedAccounts = draftAccounts.value.filter(draftAccount => draftAccount.import)

    if (approvedAccounts.length === failedImports.value.length) {
      return 'All Accounts Failed To Import'
    }

    if (failedImports.value.length) {
      return failedImports.value.length + '/' + approvedAccounts.length + ' Failed To Import';
    }

    return 'All Accounts Imported Successfully'
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

    for (const potentialAccount of potentialAccounts) {
      const response = await potentialAccount;

      if (response.response === ResponseType.SUCCESS) {
        draftAccounts.value.push(response.account);
      }
    }
  }

  async function confirmAccounts() {
    const approvedAccounts = draftAccounts.value.filter(draftAccount => draftAccount.import)

    for (const approvedAccount of approvedAccounts) {

      console.log(approvedAccount)
      const response = await createNewAccount(approvedAccount.name, approvedAccount.secret, approvedAccount.otp_digits, approvedAccount.totp_step, AccountAlgorithm.AUTODETECT)

      if (response.response === ResponseType.FAILURE) {
        failedImports.value.push(approvedAccount)
      }
    }

    importComplete.value = true
  }

  function updateSelectAll() {
    draftAccounts.value.forEach(draftAccount => draftAccount.import = selectAll.value)
  }

  function openEditor(index: number) {
    displayNameEditor.value[index] = true
  }

  function closeEditor(index: number) {
    displayNameEditor.value[index] = false
  }

</script>

<template>
  <div class="card overflow-auto">
    <select
        v-if="!importComplete"
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
      v-if="!importComplete && importType !== IMPORT_TYPE.NONE"
      class="mt-5"
      type="file"
      :accept="accept"
      @change="getFile"
    >

    <table
      v-if="!importComplete && draftAccounts.length"
      class="table table-striped mt-3"
    >
      <thead>
        <tr>
          <th>
            <div class="form-check">
              <input
                v-model="selectAll"
                class="form-check-input"
                type="checkbox"
                @change="updateSelectAll"
              >
              <label
                class="form-check-label"
                for="flexCheckDefault"
              />
            </div>
          </th>
          <th>
            Name
          </th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(draftAccount,index) in draftAccounts"
          :key="index"
        >
          <td>
            <div class="form-check">
              <input
                v-model="draftAccount.import"
                class="form-check-input"
                type="checkbox"
              >
              <label
                class="form-check-label"
                for="flexCheckDefault"
              />
            </div>
          </td>
          <td>
            <span
              v-if="!displayNameEditor[index]"
              @click="openEditor(index)"
              v-text="draftAccount.name"
            />

            <div
              v-if="displayNameEditor[index]"
              class="input-group"
            >
              <input
                v-model="draftAccount.name"
                class="form-control"
              >
              <button
                class="btn btn-primary"
                type="button"
                @click="closeEditor(index)"
              >
                Confirm
              </button>
            </div>
          </td>
        </tr>
        <tr>
          <td colspan="2">
            <div class="text-center">
              <button
                class="btn btn-primary"
                type="button"
                @click="confirmAccounts"
              >
                Import Accounts
              </button>
            </div>
          </td>
        </tr>
      </tbody>
    </table>


    <div v-if="importComplete">
      <div>
        <h2 class="text-center" v-text="importMessage"></h2>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">

</style>