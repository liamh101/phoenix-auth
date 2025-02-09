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
      transform="scale(0.45,0.45)rotate(-90 50 50)"
    />
  </svg>
</template>

<style scoped lang="scss">

:root {
  --countdown-stroke: black;
}

[data-theme="dark"] {
  --countdown-stroke: white;
}

circle {
  fill-opacity: 0;
  fill: white;
  stroke: black;
  stroke-width: 5;
  stroke-dasharray: 0;
  stroke-dashoffset: 250;
  animation: v-bind(animationTimeout) rotate linear infinite;

}

[data-theme="dark"] circle {
  stroke: white;
  fill: black;
}

@keyframes rotate {
  0% {
    stroke-dashoffset: 0;
    stroke-dasharray: 250;
  }

  75% {
    stroke: var(--countdown-stroke);
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