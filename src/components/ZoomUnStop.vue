<template>
  <v-container class="text-center">
    <v-row>
      <v-col>
        <v-img
          :src="logo"
          max-height="100"
          contain
        />
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-btn-toggle
          v-model="IdPassType"
          tile
          mandatory
        >
          <v-btn>
            Id{{"&"}}Password
          </v-btn>
          <v-btn>
            URL
          </v-btn>
        </v-btn-toggle>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <UseIdPass v-model:confno="_confno" v-model:pwd="_pwd" v-if="IdPassType===0"/>
        <UseURL v-model:url="url" v-if="IdPassType===1"/>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-text-field
        label="username"
        filled
        v-model="uname"
        ></v-text-field>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-switch
        v-model="onlyScreen"
        color="primary"
        >
          <template v-slot:label>
            {{"only screan share (no camera & audio)"}}
          </template>
        </v-switch>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-btn
          :color="changed ? 'primary' : undefined"
          :disabled="!changed"
          @click="send()"
        >
          set
        </v-btn>
      </v-col>
      <v-col>
        <v-btn
        :disabled="changed"
        @click="reload()"
        >
          reload
        </v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import UseURL from "./UseURL.vue"
import UseIdPass from "./UseIdPass.vue"
import { invoke } from '@tauri-apps/api/tauri'
import logo from "@/assets/logo.png"

interface IdPass {
  confno: string;
  pwd: string;
}

interface State {
  confno: string;
  pwd: string;
  onlyScreen: boolean;
  uname: string;
}

export default defineComponent({
  components: {
    UseURL,
    UseIdPass,
  },
  data () {
    return {
      logo: logo,
      IdPassType: 0,
      _confno: "",
      _pwd: "",
      onlyScreen: false,
      uname: "",
      url: "",
      old: {
        confno: "",
        pwd: "",
        onlyScreen: false,
        uname: "",
      },
    }
  },
  computed: {
    changed(): boolean {
      const {confno, pwd, old, onlyScreen, uname} = this;
      return !(
        confno === old.confno
        && pwd === old.pwd
        && onlyScreen === old.onlyScreen
        && uname === old.uname
      )
    },
    pwd(): string {
      if(this.IdPassType===0) return this._pwd;
      const url: string = this.url;
      try {
        return (new URL(url)).searchParams.get("pwd") ?? "";
      } catch (e) {
        return ""
      }
    },
    confno(): string {
      if(this.IdPassType===0) return this._confno;
      try {
        const url: string = this.url;
        return (new URL(url)).pathname.split("/").pop() ?? "";
      } catch (e) {
        return ""
      }
    }
  },
  methods: {
    send(){
      const {confno, pwd, old, onlyScreen, uname} = this;
      invoke("set_confno", {s: confno});
      invoke("set_pwd", {s: pwd});
      invoke("set_zc", {s: (onlyScreen ? 1 : 0).toString()});
      invoke("set_uname", {s: uname});
      old.confno = confno;
      old.pwd = pwd;
      old.onlyScreen = onlyScreen;
      old.uname = uname;
    },
    reload(){
      invoke("reload_zoom");
    }
  }
})
</script>