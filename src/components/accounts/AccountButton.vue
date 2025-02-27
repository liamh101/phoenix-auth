<script setup lang="ts">
import {computed, ref} from "vue";

const emit = defineEmits(['displayPassword', 'displayLabel']);

const showLabelButton = ref(false)

const props = defineProps({
  buttonColour: {
    type: String,
    required: true,
  },
})

function toggleButton() {
  showLabelButton.value = !showLabelButton.value;
  fireEvent();
}

function fireEvent() {
  if (showLabelButton.value) {
    emit('displayPassword');
    return;
  }

  emit('displayLabel');
}


const formattedColour = computed(() => '#' + props.buttonColour)
</script>

<template>
  <button
    v-if="!showLabelButton"
    class="btn icon-colour btn-circle btn-lg"
    @click="toggleButton"
  >
    <i class="fa-solid fa-star-of-life icon-size" />
  </button>

  <button
    v-if="showLabelButton"
    class="btn icon-colour btn-circle btn-lg"
    @click="toggleButton"
  >
    <i class="fa-solid fa-tag icon-size" />
  </button>
</template>

<style scoped lang="scss">
$buttonColour: v-bind("formattedColour");

.btn-circle {
  border-radius: 50%;
}

.icon-colour {
  background-color: $buttonColour;
  border-color: $buttonColour;
}

.icon-colour:hover {
  filter: brightness(85%);
}

.icon-size {
  height: 20px;
  width: 20px;
}
</style>