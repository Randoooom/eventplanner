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
  <FormDialog icon="mdi-plus-thick" v-model="dialog">
    <template #activator="{ activate }">
      <slot name="activator" :activate="activate" />
    </template>

    <v-card-title>
      {{ $t(title) }}
    </v-card-title>

    <v-card-subtitle v-if="event">
      {{ event.id }}
    </v-card-subtitle>

    <v-card-text class="mt-3">
      <v-row>
        <v-col cols="12">
          <v-text-field
            color="accent"
            :label="$t('page.dashboard.event.create.title')"
            v-model="current.title"
            :rules="[required($t)]"
          />
        </v-col>

        <v-col cols="12">
          <v-card-subtitle>
            {{ $t("page.dashboard.event.create.dates") }}
          </v-card-subtitle>
          <v-divider />

          <FormDatePicker
            class="mt-2"
            v-model="current.dates"
            multi-dates
            :min-date="new Date()"
          />
        </v-col>
      </v-row>
    </v-card-text>

    <v-card-actions>
      <v-btn
        variant="text"
        color="accent"
        :disabled="disabled(current.title)"
        @click="save"
      >
        {{ $t("form.action.confirm") }}
      </v-btn>
    </v-card-actions>
  </FormDialog>
</template>

<script setup lang="ts">
import {
  required,
  ref,
  onMounted,
  useClone,
  disabled,
  useNuxtApp,
} from "#imports";
import { notificationEmitter } from "~/stores/notifications";
import moment from "moment";

const title = computed(() =>
  props.event
    ? "page.dashboard.event.edit"
    : "page.dashboard.event.create.title"
);
const tooltip = computed(() => (props.event ? undefined : "dashboard.create"));

const { $surrealdb } = useNuxtApp();
const notifications = notificationEmitter();
const emit = defineEmits(["refresh"]);
const props = defineProps({
  event: { type: Object, required: false },
  customActivator: { type: Boolean, default: false },
  floating: { type: Boolean, default: false },
});
const dialog = ref(false);

onMounted(() => {
  if (props.event) {
    current.value = useClone(props.event);
  } else {
    current.value = useClone(base);
  }
});

const base = {
  title: "",
  dates: [] as string[] | Date[],
};
const current = ref({
  title: "",
  dates: [] as string[] | Date[],
});

async function save() {
  current.value.dates = current.value.dates.map((date: string | Date) =>
    moment.utc(date).toISOString()
  );
  try {
    if (props.event) {
      await $surrealdb.connection.update(props.event.id, {
        dates: current.value.dates,
        title: current.value.title,
      });
    } else {
      await $surrealdb.connection.create("event", current.value);
    }

    notifications.attachNotification({
      icon: "mdi-check",
      color: "success",
      title: "form.save.response.success.title",
      content: "form.save.response.success.description",
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
