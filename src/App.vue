<template>
  <v-app>
    <v-main>
      <ZoomUnStop/>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import HelloWorld from './components/HelloWorld.vue'
import ZoomUnStop from './components/ZoomUnStop.vue'

export default defineComponent({
  components: {
    ZoomUnStop,
  },

  data() {
    return {
      //
    }
  },
})
</script>
