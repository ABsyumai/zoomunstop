
<template>
  <v-container>
    <v-row>
      <v-col>
        <v-text-field
          label="ID"
          filled
          v-model="_confno"
        ></v-text-field>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-text-field
          label="password"
          filled
          v-model="_pwd"
        ></v-text-field>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
export default defineComponent({
  props: {
    confno: {
      type: String,
      required: true,
    },
    pwd: {
      type: String,
      required: true,
    },
  },
  emits: [
    "update:confno",
    "update:pwd"
  ],
  data() {
    return {
    }
  },
  computed: {
    _confno: {
      set(value: string) {
        this.$emit("update:confno", value)
      },
      get(): string {
        return this.$props.confno;
      }
    },
    _pwd: {
      set(value: string) {
        this.$emit("update:pwd", value)
      },
      get(): string {
        return this.$props.pwd;
      }
    }
  }
})
</script>