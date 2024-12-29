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
  <FormFloatingDialog icon="mdi-plus-thick" v-model="dialog">
    <template #activator="{ activate }">
      <slot name="activator" :activate="activate" />
    </template>

    <v-card-title>
      {{ $t("visitor.list.add") }}
    </v-card-title>

    <v-card-text class="mt-3">
      <v-row>
        <v-col cols="12">
          <v-text-field
            color="accent"
            :label="$t('form.username')"
            v-model="username"
            :rules="[required($t)]"
          />
        </v-col>
      </v-row>
    </v-card-text>

    <v-card-actions>
      <v-btn
        variant="text"
        color="accent"
        :disabled="disabled(username)"
        @click="save"
      >
        {{ $t("form.action.confirm") }}
      </v-btn>
    </v-card-actions>
  </FormFloatingDialog>
</template>

<script setup lang="ts">
import {
  required,
  ref,
  watch,
  disabled,
  useNuxtApp,
  useLocalePath,
} from "#imports";
import { notificationEmitter } from "~/stores/notifications";
import clipboard from "clipboardy";

const localePath = useLocalePath();
const { $surrealdb } = useNuxtApp();
const notifications = notificationEmitter();
const emit = defineEmits(["refresh"]);
const props = defineProps({
  event: { type: Object, required: true },
});
const dialog = ref(false);
const username = ref("");

watch(dialog, (value: boolean) => {
  if (value) username.value = "";
});

async function save() {
  try {
    const visitor = await $surrealdb.connection
      .query(
        'CREATE visitor CONTENT { "username": $username, "event": type::thing("event", $event) }',
        {
          username: username.value,
          event: props.event.id,
        }
      )
      .then((value: any) => value[0][0]);

    // copy the voting link
    await clipboard.write(
      `https://${document.location.hostname}${localePath("/vote")}?visitor=${
        visitor.id.id
      }`
    );

    notifications.attachNotification({
      icon: "mdi-check",
      color: "success",
      title: "visitor.create.response.success.title",
      content: "visitor.create.response.success.description",
    });
    emit("refresh");
  } catch {
    notifications.attachNotification({
      icon: "mdi-alert",
      color: "error",
      title: "form.save.response.error.title",
      content: "form.save.response.error.description",
    });
  }

  dialog.value = false;
}
</script>
