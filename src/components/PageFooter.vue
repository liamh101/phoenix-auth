<script setup lang="ts">
  const emit = defineEmits(['showTokens', 'showSettings']);

  function tokensSelected() {
    emit('showTokens');
  }

  function settingsSelected()
  {
    emit('showSettings');
  }


</script>

<template>
  <footer class="footer">
    <div class="container-fluid h-100">
      <div class="row h-100">
        <div class="col-6 p-0">
          <div class="d-grid gap-2 h-100 divider">
            <button
              class="btn footer-button"
              @click="tokensSelected"
            >
              <i class="fa-solid fa-users fa-2xl" />
            </button>
          </div>
        </div>
        <div class="col-6 p-0">
          <div class="d-grid gap-2 h-100">
            <button
              class="btn footer-button"
              @click="settingsSelected"
            >
              <i class="fa-solid fa-gears fa-2xl" />
            </button>
          </div>
        </div>
      </div>
    </div>
  </footer>
</template>

<style lang="scss" scoped>
.footer {
  position: fixed;
  bottom: 0;
  width: 100%;
  height: 4rem;
  line-height: 60px;
  background-color: #f5f5f5;
}

[data-theme="dark"] .footer {
  background-color: #131313;
}

.footer-button {
  border-radius: 0;
}

.divider {
  border-right: #D9D9D9 solid;
  border-width: 1px;
}

[data-theme="dark"] .divider  {
  border-right: #262626 solid;
}

</style>