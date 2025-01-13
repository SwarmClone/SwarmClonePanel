<!-- Login.vue -->
<template>
  <v-container class="fill-height" fluid>
    <v-row align="center" justify="center">
      <v-col cols="12" sm="8" md="4">
        <v-card class="elevation-12" max-width="400">
          <v-card-title class="text-center">
            <h2 class="text-h5 font-weight-bold">登录到 SwarmClone</h2>
          </v-card-title>
          <v-card-text>
            <v-form @submit.prevent="login">
              <v-text-field
                label="用户名"
                name="login"
                prepend-inner-icon="mdi-account"
                type="text"
                v-model="username"
                outlined
                dense
                class="mb-4 rounded-input"
              ></v-text-field>
              <v-text-field
                id="password"
                label="密码"
                name="password"
                prepend-inner-icon="mdi-lock"
                type="password"
                v-model="password"
                outlined
                dense
                class="rounded-input"
              ></v-text-field>
              <v-btn
                color="primary"
                block
                large
                type="submit"
                class="mt-6 rounded-button"
                :loading="loading"
                @click="loading = true"
              >
                登录
              </v-btn>
            </v-form>
            <v-alert
              v-if="errorMessage"
              type="error"
              class="mt-4"
            >
              {{ errorMessage }}
            </v-alert>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import xss from 'xss';

export default {
  emits: ['close-login'],
  setup(props, { emit }) {
    const username = ref('');
    const password = ref('');
    const loading = ref(false);
    const errorMessage = ref('');

    function login() {
      // 过滤用户输入，防止XSS攻击
      const safeUsername = xss(username.value);
      const safePassword = xss(password.value);

      console.log('用户名:', safeUsername);
      console.log('密码:', safePassword);

      invoke('login', { username: safeUsername, password: safePassword })
        .then((response) => {
          console.log(response);
          emit('close-login');
        })
        .catch((error) => {
          console.error(error);
          errorMessage.value = '账号或密码错误';
        })
        .finally(() => {
          loading.value = false;
        });
    }

    return {
      username,
      password,
      login,
      loading,
      errorMessage
    };
  },
};
</script>

<style scoped>
.v-card {
  border-radius: 12px;
  transition: box-shadow 0.3s ease;
}

.v-card:hover {
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.2);
}

.v-card-title {
  padding-top: 24px;
  padding-bottom: 16px;
}

.v-card-text {
  padding: 16px;
}

.v-card-actions {
  padding: 16px;
}

.rounded-input .v-input__control .v-input__slot {
  border-radius: 8px;
}

.rounded-button {
  border-radius: 8px;
  transition: background-color 0.3s ease;
}

.rounded-button:hover {
  background-color: #1976d2;
}

.font-weight-bold {
  font-weight: bold;
}
</style>