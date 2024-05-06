<script setup lang="ts">
  import {computed} from "vue";

  const props = defineProps({
    timeout: {
      type: Number,
      required: true,
    }
  });

  const animationTimeout = computed(() => props.timeout + 's')
</script>

<template>
  <svg
    height="45"
    width="45"
  >
    <circle
      cx="50"
      cy="50"
      r="40"
      transform="scale(0.45,0.45)"
    />
  </svg>
</template>

<style scoped lang="scss">

circle {
  fill: white;
  stroke: black;
  stroke-width: 5;
  stroke-dasharray: 0;
  stroke-dashoffset: 250;
  animation: v-bind(animationTimeout) rotate linear infinite;
}

@keyframes rotate {
  0% {
    stroke-dashoffset: 0;
    stroke-dasharray: 250;
  }

  75% {
    stroke: black;
  }

  76% {
    stroke: red;
  }

  100% {
    stroke: red;
    stroke-dashoffset: 250;
    stroke-dasharray: 250;
  }
}


</style>