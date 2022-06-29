<template>
  <v-container>
    <v-row>
      <v-col>
        <v-text-field
          label="URL"
          filled
          :rules="[urlValidator]"
          v-model="_url"
        ></v-text-field>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    url: {
      type: String,
      required: true,
    },
  },
  emits: [
    "update:url",
  ],
  data () {
    return {
    }
  },
  methods: {
      urlValidator(src: string) {
        function urlValidatorImpl(s: string): boolean {
          try {
            const p = new URL(s);
            return p.searchParams.has("pwd")
          } catch (error) {
            return false
          }
        }
        return urlValidatorImpl(src) || "invalid url"
      },
  },
  computed: {
    _url: {
      set(value: string) {
        this.$emit("update:url", value);
      },
      get(): string {
        return this.$props.url;
      }
    },
  }
})
</script>