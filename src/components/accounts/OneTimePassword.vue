 <script setup lang="ts">
 import {onMounted, ref} from "vue";
 import {generateToken} from "../../composables/Commands.ts";

const props = defineProps({
  accountId: {
    type: Number,
    required: true,
  }
});

const emit = defineEmits(['otp']);

const DEFAULT_TEXT = '------'

let otp = ref(DEFAULT_TEXT);

async function getOneTimePassword() {
  const token = (await generateToken(props.accountId)).token

  emit('otp', token);

  otp.value = token;
}

onMounted(() => {
  getOneTimePassword()
  setInterval(() => getOneTimePassword(), 30000)
})

</script>

<template>
  <div class="d-grid gap-2">
    <span
      class="list-item-text align-middle"
      v-text="otp"
    />
  </div>
</template>