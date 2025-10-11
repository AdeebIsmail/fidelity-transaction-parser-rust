<script setup lang="ts">
// App.vue now serves as the main router container
</script>

<template>
  <div id="app">
    <router-view />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

#app {
  width: 100%;
  height: 100vh;
}

body {
  margin: 0;
  padding: 0;
}
</style>
