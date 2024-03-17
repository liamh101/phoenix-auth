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
    <div class="container-fluid">
      <div class="row">
        <div class="col-6">
          <div class="d-grid gap-2">
            <button class="btn" @click="tokensSelected">
              Tokens
            </button>
          </div>
        </div>

        <div class="col-6">
          <div class="d-grid gap-2">
            <button class="btn" @click="settingsSelected">
              Settings
            </button>
          </div>
        </div>
      </div>
    </div>
  </footer>
</template>

<style lang="scss" scoped>
.footer {
  position: absolute;
  bottom: 0;
  width: 100%;
  height: 60px;
  line-height: 60px;
  background-color: #f5f5f5;
}

</style>