 <script setup lang="ts">
 import {computed, onMounted, ref} from "vue";
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

const showToken = ref(false);

async function getOneTimePassword() {
  const token = (await generateToken(props.accountId)).token

  emit('otp', token);

  otp.value = token;
}

async function toggleToken() {
  showToken.value = !showToken.value;
  await getOneTimePassword();
}

const tokenValue = computed(() => showToken.value ? otp.value : DEFAULT_TEXT)

onMounted(() => setInterval(() => getOneTimePassword(), 30000))

</script>

<template>
  <div class="d-grid gap-2">
    <button class="btn" type="button" @click="toggleToken" v-text="tokenValue"></button>
  </div>
</template>