<script setup lang="ts">
  import {getSyncLogs, SyncLog, SyncLogType} from "../../../composables/Commands.ts";
  import {onMounted, ref, computed} from "vue";
  import { DateTime } from "luxon";

  let logArray: SyncLog[] = [];
  const logs = ref(logArray);

  async function getLogs() {
    const response = await getSyncLogs();

    logs.value = response.logs;
  }

  function getRowColour(log: SyncLog) {
    switch (log.log_type) {
      case SyncLogType.ERROR: {
        return 'table-danger';
      }
      default: {
        return '';
      }
    }
  }

  function formatTimestamp(log: SyncLog) {
    const date = DateTime.fromSeconds(log.timestamp);

    return date.toLocaleString(DateTime.DATETIME_SHORT)
  }

  onMounted(() => getLogs())
</script>

<template>
  <table class="table">
    <thead>
    <tr>
      <th class="col">
        Log
      </th>
      <th class="col">
        Timestamp
      </th>
    </tr>
    </thead>
    <tbody>
      <tr :key="log.id" v-for="log in logs" :class="getRowColour(log)">
        <td v-text="log.log"></td>
        <td v-text="formatTimestamp(log)"></td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped lang="scss">

</style>