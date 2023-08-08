<!--
  -     Copyright (C) 2023 Fritz Ochsmann
  -
  -     This program is free software: you can redistribute it and/or modify
  -     it under the terms of the GNU Affero General Public License as published
  -     by the Free Software Foundation, either version 3 of the License, or
  -     (at your option) any later version.
  -
  -     This program is distributed in the hope that it will be useful,
  -     but WITHOUT ANY WARRANTY; without even the implied warranty of
  -     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  -     GNU Affero General Public License for more details.
  -
  -     You should have received a copy of the GNU Affero General Public License
  -     along with this program.  If not, see <https://www.gnu.org/licenses/>.
  -->

<template>
  <v-container class="d-flex justify-center align-center">
    <v-card width="500px">
      <v-card-title>
        {{ $t("signup.title") }}
      </v-card-title>

      <v-card-text>
        <v-row>
          <v-col cols="12">
            <v-text-field
              color="accent"
              :label="$t('form.username')"
              v-model="username"
              :rules="[required($t)]"
            />
          </v-col>

          <v-col cols="12">
            <FormPasswordInput color="accent" v-model="password" strength />
          </v-col>

          <v-col cols="12">
            <FormPasswordInput
              color="accent"
              v-model="confirm"
              strength
              label="form.repeat"
            />
          </v-col>
        </v-row>
      </v-card-text>

      <v-card-actions>
        <v-btn
          variant="text"
          color="accent"
          :disabled="
            disabled($t, username, password, confirm) || confirm !== password
          "
          @click="signup"
        >
          {{ $t("form.confirm") }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import {
  required,
  disabled,
  ref,
  useNuxtApp,
  useRouter,
  useLocalePath,
} from "#imports";
import { notificationEmitter } from "~/stores/notifications";

const { $surrealdb } = useNuxtApp();
const emitter = notificationEmitter();
const localePath = useLocalePath();

const username = ref("");
const password = ref("");
const confirm = ref("");

async function signup() {
  try {
    await $surrealdb.signup(username.value, password.value);
    emitter.attachNotification({
      color: "success",
      content: "signup.success.description",
      icon: "mdi-check",
      title: "signup.success.title",
    });

    await useRouter().push(localePath("/dashboard"));
  } catch (e) {
    console.log(e);
    emitter.attachNotification({
      color: "error",
      content: "signup.error.description",
      icon: "mdi-alert",
      title: "signup.error.title",
    });
  }
}
</script>

<style lang="sass" scoped>
.v-container
  height: 100%
</style>
